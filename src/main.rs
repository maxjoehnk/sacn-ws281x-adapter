use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use clap::Parser;
use itertools::Itertools;
use pinboard::NonEmptyPinboard;
use rs_ws281x::{ChannelBuilder, ControllerBuilder};
use sacn_unofficial::packet::ACN_SDT_MULTICAST_PORT;
use sacn_unofficial::receive::SacnReceiver;

use crate::args::SacnWs281xAdapterArgs;

mod args;

const PIXELS_PER_UNIVERSE: usize = 150;

fn main() {
    env_logger::init();
    let args = SacnWs281xAdapterArgs::parse();
    let strip_type = args.pixel_mode.into();
    log::info!("sacn-ws281x-adapter v{} starting up", env!("CARGO_PKG_VERSION"));
    let universes = vec![0; args.pixel_count / PIXELS_PER_UNIVERSE]
        .into_iter()
        .enumerate()
        .map(|(i, _)| i as u16 + 1)
        .collect::<Vec<_>>();
    let universes = args.universes.unwrap_or(universes);
    log::info!("Listening on Universes {}", universes.iter().join(", "));
    let pixel_buffer = Arc::new(NonEmptyPinboard::new(vec![[0u8; 4]; args.pixel_count]));
    let send_buffer = Arc::clone(&pixel_buffer);

    let sacn_handle = std::thread::Builder::new()
        .name("sACN Receive".into())
        .spawn(move || {
            log::debug!("Starting sACN Thread...");
            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::from([0, 0, 0, 0])), ACN_SDT_MULTICAST_PORT);
            let mut dmx_rcv = SacnReceiver::with_ip(addr.clone(), None).unwrap();

            dmx_rcv.set_announce_source_discovery(true);
            dmx_rcv.set_is_multicast_enabled(true).unwrap();
            dmx_rcv.listen_universes(&universes).unwrap();
            log::info!("listening on {addr:?}");

            loop {
                match dmx_rcv.recv(None) {
                    Ok(packets) => {
                        log::trace!("Received sACN packet {packets:?}");
                        let mut buffer = send_buffer.read();
                        for packet in packets {
                            for (index, universe_id) in universes.iter().enumerate() {
                                if packet.universe == *universe_id {
                                    let led_offset = index * PIXELS_PER_UNIVERSE;
                                    for (i, chunk) in packet.values
                                        .iter()
                                        .skip(1)
                                        .copied()
                                        .chunks(3)
                                        .into_iter()
                                        .take(PIXELS_PER_UNIVERSE)
                                        .enumerate() {
                                        let chunk = chunk.collect::<Vec<_>>();
                                        buffer[led_offset + i] = [chunk[0], chunk[1], chunk[2], 0];
                                    }
                                }
                            }
                        }
                        send_buffer.set(buffer);
                    }
                    Err(err) => log::error!("Error receiving sACN {err:?}"),
                }
            }
        })
        .unwrap();

    let pixel_handle = std::thread::Builder::new()
        .name("Ws281x Writer".into())
        .spawn(move || {
            log::debug!("Starting Ws281x Thread...");
            let mut led_controller = ControllerBuilder::new()
                .channel(0, ChannelBuilder::new()
                    .pin(18)
                    .count(args.pixel_count as i32)
                    .strip_type(strip_type)
                    .brightness(255)
                    .build())
                .build()
                .unwrap_or_else(|err| {
                    log::error!("Unable to configure strip: {err:?}");

                    std::process::exit(1);
                });

            loop {
                let mut requires_render = false;
                let leds = led_controller.leds_mut(0);
                log::trace!("Pixel Buffer: {leds:?}");
                for (i, chunk) in pixel_buffer.get_ref()
                    .iter()
                    .enumerate() {
                    let before = leds[i];
                    leds[i] = [chunk[0], chunk[1], chunk[2], 0];

                    if before != leds[i] {
                        requires_render = true;
                    }
                }
                if requires_render {
                    log::trace!("Rendering {leds:?}");
                    if let Err(err) = led_controller.render() {
                        log::error!("Unable to render strip: {err:?}");
                    }
                }
            }
        })
        .unwrap();

    sacn_handle.join().unwrap();
    pixel_handle.join().unwrap();
}
