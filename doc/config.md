# Config

The config file defines the app configuration. This contains a list of
`key=value` pairs defining the look and behaviour of the app. Currently the only
supported

- `entry-background:Color` (required) - The background color of an entry
- `entry-text-color:Color` (required) - The text color of an entry
- `entry-radius:u8` - The border radius of an entry, defaults to `0`
- `entry-height:f32` - The height of an entry, only works if `entry-width` is
  also provided
- `entry-width:f32` - The width of an entry, only works if `entry-height` is
  also provided
- `entry-text-size:f32` - The text size of entries, defaults to `12`

## Colors

Currently, colors can be written in the following formats

- `#RRGGBB` - RGB in hex format
- `#RRGGBBAA` - RGBA in hex format
- `rgb(r,g,b)` - RGB in decimal format
- `rgba(r,g,b,a)` - RGBA in decimal format
