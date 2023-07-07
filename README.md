# `rtic-blinky`

A basic blinky based on [RTIC](https://rtic.rs/2/book/en/) and its [template](https://github.com/rtic-rs/defmt-app-template) implemented for the `nRF52840` chip.

The blinky will... well... blink a pair of LEDs.
Driven by the `nRF52480`'s timer peripheral, the `nRF52480-DK`'s LEDs `LED1` and `LED2` get blinked alternatingly.
Each LED gets blinked twice before switching to the other one.

## Dependencies

### 1. `nRF52840-DK`

This repository is targeted at a `nRF52840-DK` development kit.

### 2. `flip-link`

```console
$ cargo install flip-link
```

### 3. `probe-run`

```console
$ # make sure to install v0.2.0 or later
$ cargo install probe-run
```

## Setup

### 1. Clone the project

```console
$ git clone https://github.com/90degs2infty/rtic-blinky.git rtic-blinky
```

### 2. Run!

```console
$ # `rb` is an alias for `run --bin`
$ DEFMT_LOG=info cargo rb blinky
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
