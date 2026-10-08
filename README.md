# Black Ocean for Zed

A dark theme with blues and greens for the [Zed](https://zed.dev) editor, ported from
[Black Ocean](https://github.com/Zamerick/black-ocean) for VS Code by Alex Oxthorn
(itself inspired by Dayle Rees's Rainglow).

It includes two variants:

- **Black Ocean**: the original muted ocean palette
- **Black Ocean Neon**: brighter, more saturated syntax colors

## Installation

### From the extension registry

1. Open the command palette and run `zed: extensions`.
2. Search for **Black Ocean** and click **Install**.
3. Run `theme selector: toggle` and pick **Black Ocean** or **Black Ocean Neon**.

### As a dev extension

1. Clone this repository.
2. In Zed, run `zed: install dev extension` and choose the cloned folder.
3. Select the theme with `theme selector: toggle`.

## Palette

| Role                         | Black Ocean | Neon      |
| ---------------------------- | ----------- | --------- |
| Background                   | `#101316`   | `#101316` |
| Foreground                   | `#dfdfdf`   | `#dfdfdf` |
| Comments                     | `#60778c`   | `#60778c` |
| Keywords, tags               | `#007aae`   | `#00b8ff` |
| Constants, types, attributes | `#019d76`   | `#00df88` |
| Functions, numbers           | `#15b8ae`   | `#00f0dd` |
| Strings                      | `#7ebea0`   | `#44e8b0` |

## Development

The theme file is generated, so don't edit `themes/black-ocean.json` by hand. Change the
palette or mappings in `generator/src/main.rs`, then regenerate:

```sh
cargo run --manifest-path generator/Cargo.toml
```

Reload the dev extension in Zed to see the change.

## License

MIT. See [LICENSE](LICENSE). The original colors are © 2018 Alex Oxthorn.
