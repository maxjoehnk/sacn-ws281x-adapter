use std::fmt::{Display, Formatter};
use clap::{Parser, ValueEnum};
use rs_ws281x::StripType;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct SacnWs281xAdapterArgs {
    #[arg(short = 'p', long)]
    pub pixel_count: usize,
    #[arg(short = 'c', long, default_value_t = 150)]
    pub pixels_per_universe: usize,
    #[arg(short = 'u', long)]
    pub universes: Option<Vec<u16>>,
    #[arg(short = 'm', long, default_value_t)]
    pub pixel_mode: PixelMode
}

#[derive(Default, Debug, Clone, Copy, ValueEnum)]
pub enum PixelMode {
    Sk6812Rgbw,
    Sk6812Rbgw,
    Sk6812Gbrw,
    Sk6812Grbw,
    Sk6812Brgw,
    Sk6812Bgrw,
    Ws2811Rgb,
    Ws2811Rbg,
    Ws2811Grb,
    #[default]
    Ws2811Gbr,
    Ws2811Brg,
    Ws2811Bgr,
    Ws2812,
    Sk6812,
    Sk6812W,
}

impl Display for PixelMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<PixelMode> for StripType {
    fn from(value: PixelMode) -> Self {
        match value {
            PixelMode::Sk6812Rgbw => Self::Sk6812Rgbw,
            PixelMode::Sk6812Rbgw => Self::Sk6812Rbgw,
            PixelMode::Sk6812Gbrw => Self::Sk6812Gbrw,
            PixelMode::Sk6812Grbw => Self::Sk6812Grbw,
            PixelMode::Sk6812Brgw => Self::Sk6812Brgw,
            PixelMode::Sk6812Bgrw => Self::Sk6812Bgrw,
            PixelMode::Ws2811Rgb => Self::Ws2811Rgb,
            PixelMode::Ws2811Rbg => Self::Ws2811Rbg,
            PixelMode::Ws2811Grb => Self::Ws2811Grb,
            PixelMode::Ws2811Gbr => Self::Ws2811Gbr,
            PixelMode::Ws2811Brg => Self::Ws2811Brg,
            PixelMode::Ws2811Bgr => Self::Ws2811Bgr,
            PixelMode::Ws2812 => Self::Ws2812,
            PixelMode::Sk6812 => Self::Sk6812,
            PixelMode::Sk6812W => Self::Sk6812W,
        }
    }
}
