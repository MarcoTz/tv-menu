# Menu Entries

The entries use a config format similar to `.desktop` consisting of a list of
`key = value` pairs which define the entry. Currently, the following keys are
supported.

- `title: String` (required) - The title shown in the menu
- `launch: String` (required) - The command to run on selection
- `icon:String` - The Icon to be displayed. Icons are currently searched for in
  the following directories
  - `usr/share/icons`
  - `usr/share/pixmaps` As of now, no `svg` icons are supported. When the no
    icon is specified, no icon is shown

As of now, entries are hardcoded to the `entries` directory in the project root.
