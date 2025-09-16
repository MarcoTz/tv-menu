# Menu Entries

The entries use a config format similar to `.desktop` consisting of a list of
`key = value` pairs which define the entry. Currently, the following keys are
supported.

- `title: String` (required) - The title shown in the menu
- `launch: String` (required) - The command to run on selection

As of now, entries are hardcoded to the `entries` directory in the project root.
