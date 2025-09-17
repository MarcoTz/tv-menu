# Config

The config file defines the app configuration. The config is written in a
toml-like format, that is, a list of `key=value` pairs separated into different
sections starting with `[Section]` supported.

Anyting after a `//` in a line will be ignored as a comment

## Sections

### Empty

The "Empty" Section is the section starting the config file before any other
sections begin. Here, the following keys are supported

- `background:Color` - The background color of the window, defaults to black
- `text-color:Color` - The text color of the window, this does not apply to,
  defaults to white entries
- `padding:f32` - The padding of the window, defaults to `0`
- `columns:u64` - Maximum number of columns per row, defaults to the current
  window size

### `[Entries]`

The `Entries` section defines how entries are displayed in the menu. Currently,
the following keys are supported

- `background:Color` - The background color of an entry, defaults to transparent
- `text-color:Color` - The text color of an entry, defaults to transparent
- `border-radius:u8` - The border radius of an entry, defaults to `0`
- `height:f32` - The height of an entry, only works if `entry-width` is also
  provided
- `width:f32` - The width of an entry, only works if `entry-height` is also
  provided
- `text-size:f32` - The text size of entries, defaults to `12`

## Colors

Currently, colors can be written in the following formats

- `#RRGGBB` - RGB in hex format
- `#RRGGBBAA` - RGBA in hex format
- `rgb(r,g,b)` - RGB in decimal format
- `rgba(r,g,b,a)` - RGBA in decimal format
