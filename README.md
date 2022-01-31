# UseL-OS

A useless rusty os.

## How to get it to run

Clone and enter this repository
```sh
git clone https://github.com/einsjannis/usel-os && cd usel-os
```

Install the cargo bootimage binary
```sh
cargo install bootimage
```

Add required components
```sh
rustup component add rust-src
rustup component add llvm-tools-preview
```

Install QEMU on your system (on Arch as follows: `pacman -S qemu` or `pacman -S qemu-headless`)

Run a qemu vm with the os through the `cargo run` command

## Inspired by

[Philipp Oppermann's os](https://os.phil-opp.com/)

