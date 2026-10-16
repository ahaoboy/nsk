# nsk

A small toolbox for Nintendo Switch content. It currently installs firmware
packages, installs key files, and reserves the command surface for Switch Army
Knife (SAK) style format conversion, with each SAK command documented but not
yet implemented.

## Build

```sh
cargo build --release
```

The binary is written to `target/release/nsk`.

## Features

| Feature | Default | Enables                                                                                          |
| ------- | ------- | ------------------------------------------------------------------------------------------------ |
| `cli`   | yes     | The `clap`-based command-line front end (`cli`, `sak`, the `firmware`/`keys` commands and the `nsk` binary). |

The library part (`registry`, `firmware`, `keys`) has no CLI dependency, so it
can be used on its own:

```sh
cargo build --no-default-features       # library only, no clap
```

With `--no-default-features` the `nsk` binary is skipped (it declares
`required-features = ["cli"]`).

## Install firmware

```sh
nsk firmware install <PACKAGE> <TARGET> [--layout <LAYOUT>] [--skip-existing]
```

* `PACKAGE` — the firmware `.zip` package. NCAs may be nested in subfolders, so
  every archive entry is walked recursively; only files whose base name is an
  NCA (32 hex digits followed by `.nca` or `.cnmt.nca`) are installed.
* `TARGET` — the directory that receives the installed content.
* `--layout` — the directory layout the emulator expects:

  | Layout      | Path                         | Used by                 |
  | ----------- | ---------------------------- | ----------------------- |
  | `flat`      | `<TARGET>/<id>.nca/00`       | eden / yuzu / ruzu (default) |
  | `two-digit` | `<TARGET>/000000XX/<id>.nca` | bucketed installs       |

  The `XX` bucket is the first byte of the SHA-256 over the 16 raw bytes of the
  NCA id, matching the upstream `RegisteredCache`. The default `flat` layout is
  what the reference shell installer produces, including the trick of relocating
  a plain `<id>.nca` file out of the way before creating the `<id>.nca/`
  directory.
* `--skip-existing` — keep NCAs that are already present. NCAs are content
  addressed, so the default is to (re)write them, mirroring the `cp -f` in the
  reference script.

## Install keys

```sh
nsk keys install <PACKAGE> <TARGET> [--skip-existing]
```

* `PACKAGE` — the key `.zip` package (`prod.keys`, `title.keys`, ...). Files may
  be nested (for example `Keys-22.1.0/prod.keys`); every `*.keys` entry is
  installed flat into `TARGET`.
* `TARGET` — the key directory (for example `user/keys`).
* `--skip-existing` — keep key files that are already present.

## Packaging example

Both packaging steps of a firmware variant collapse into two commands:

```bash
variant_root=$(package_root "ruzu-$version")

nsk keys install     "dist/$prodkeys_zip" "$variant_root/user/keys"
nsk firmware install "dist/$firmware_zip" "$variant_root/user/nand/system/Contents/registered"
```

This replaces the previous unzip / `find … -exec cp` / `<id>.nca/00`
reorganisation blocks, and does not need temporary `ProdKeys` / `Firmware`
directories.

## Switch Army Knife (reserved)

The SAK feature set is exposed as top-level commands (flattened, not nested
under a `sak` group). Arguments are already defined, but the handlers only
report that they are not implemented yet.

```sh
nsk --help
```

| Command            | Purpose                                          |
| ------------------ | ------------------------------------------------ |
| `patch-xci`        | Patch an XCI so it boots without valid signatures |
| `split-xci`        | Split an XCI into 4 GiB parts                    |
| `merge-xci`        | Merge split XCI parts back into one file         |
| `update-xci`       | Merge game updates and DLC into an XCI           |
| `xci-to-nsp`       | Convert an XCI into an NSP                       |
| `compress-xci`     | Compress an XCI into an XCZ                      |
| `decompress-xcz`   | Decompress an XCZ back into an XCI               |
| `extract-firmware` | Extract the firmware from an XCI                 |
| `patch-nsp`        | Patch an NSP so it installs without signatures   |
| `split-nsp`        | Split an NSP into 4 GiB parts                    |
| `merge-nsp`        | Merge split NSP parts back into one file         |
| `update-nsp`       | Merge game updates and DLC into an NSP           |
| `nsp-to-xci`       | Convert an NSP into an XCI                       |
| `compress-nsp`     | Compress an NSP into an NSZ                      |
| `decompress-nsz`   | Decompress an NSZ back into an NSP               |
| `extract-save`     | Extract Switch savegames from the user partition |
| `forwarder`        | Build an installable NSP forwarder from an NRO   |
| `format-sd`        | Format a device as FAT32                         |

## Module map

| Module      | Feature | Responsibility                                                     |
| ----------- | ------- | ------------------------------------------------------------------ |
| `registry`  | -       | NCA identifiers, storage layouts and the `RegisteredCache` writer   |
| `firmware`  | -       | `FirmwarePackage` reader; the `firmware install` command with `cli` |
| `keys`      | -       | `KeysPackage` reader and `KeyStore`; the `keys install` command with `cli` |
| `cli`       | `cli`   | Top-level argument parsing and dispatch                            |
| `sak`       | `cli`   | Reserved SAK command surface, flattened into the top-level CLI     |
| `error`     | -       | Crate-wide `thiserror` error type                                  |
| `archive`   | -       | Internal zip/copy plumbing shared by the stores (private)           |

Errors use [`thiserror`](https://docs.rs/thiserror), the CLI is built with
[`clap`](https://docs.rs/clap) (optional) and the packages are read with
[`zip`](https://docs.rs/zip).
