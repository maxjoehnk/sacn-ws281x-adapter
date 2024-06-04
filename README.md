# sacn-ws281x-adapter

This is a simple adapter that allows you to control WS281x LEDs using the E1.31 (sACN) protocol.

## Usage

```bash
sacn-ws281x-adapter -p 300 -c 150 -u 1 -u 2 -u 3 -u 4
```

## Options

### Pixel Count (`-p`)

Define how many pixels should be addressed

### Pixels per Universe (`-c`)

**Default**: 150

Define how many pixels should be addressed per universe. This is useful if you want to split a strip into multiple fixtures.

### Universes (`-u`)

Define which universes should be listened to.

#### Example
`-u 1 -u 2 -u 3 -u 4`

This would listen on Universes 1-4 splitting the pixels according to the `Pixels per Universe` option.

### Pixel Mode (`-m`)

**Default**: Ws2811Gbr

Define the pixel mode. This should match the pixel type you are using.

#### Available Options

* Sk6812Rgbw
* Sk6812Rbgw
* Sk6812Gbrw
* Sk6812Grbw
* Sk6812Brgw
* Sk6812Bgrw
* Ws2811Rgb
* Ws2811Rbg
* Ws2811Grb
* Ws2811Gbr
* Ws2811Brg
* Ws2811Bgr
* Ws2812
* Sk6812
* Sk6812W

### GPIO Pin (`-g`, `--gpio`)

**Default**: 18

Define the port the led strip is connected to.
