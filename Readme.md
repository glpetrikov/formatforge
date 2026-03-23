# formatforge

Convert between data formats from the command line.

[![Crates.io](https://img.shields.io/crates/v/formatforge?color=orange)](https://crates.io/crates/formatforge)
[![License: MIT](https://img.shields.io/badge/license-MIT-orange.svg)](LICENSE)

## Supported formats

| Format | Extension | Notes |
|--------|-----------|-------|
| JSON | `.json` | |
| TOML | `.toml` | No null values |
| YAML | `.yaml`, `.yml` | |
| XML | `.xml` | |
| ENV | `.env` | Key=value pairs |
| KDL | `.kdl` | [kdl-lang.com](https://kdl.dev) |
| CBOR | `.cbor` | Binary format |
| INI | `.ini` | |
| JSON5 | `.json5` | Comments, trailing commas |
| JSONC | `.jsonc` | JSON with comments |
| MessagePack | `.msgpack` | Binary format |

## Installation

```bash
cargo install formatforge
```

## Usage

```bash
# Format is detected automatically from file extension
formatforge input.json output.toml
formatforge config.yaml config.json
formatforge data.toml data.yaml

# Pack to binary
formatforge data.json data.cbor
formatforge data.json data.msgpack

# Unpack from binary
formatforge data.cbor data.json
formatforge data.msgpack data.toml

# Override format manually
formatforge data.json out.yaml --to yaml
formatforge input --from json output.toml
```

## Examples

Convert a JSON config to TOML:
```bash
formatforge config.json config.toml
```

Escape XML hell:
```bash
formatforge legacy.xml modern.json
```

Pack data for transport, unpack for reading:
```bash
formatforge data.yaml data.cbor    # pack
formatforge data.cbor data.yaml    # unpack
```

## License

MIT — see [LICENSE](LICENSE)

## Links

- [crates.io](https://crates.io/crates/formatforge)
- [GitHub](https://github.com/glpetrikov/formatforge)
