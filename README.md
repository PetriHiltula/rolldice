# rolldice

CLI tool for dice rolling.

Usage: ```rolldice <number>d<dice><modifier>```

eg. ```rolldice 2d6+1```

## Building

Install Rust https://www.rust-lang.org/tools/install

Clone or download repository, eg.

    $ git clone https://github.com/PetriHiltula/rolldice.git

Build a binary

    $ cd rolldice
    $ cargo build --release

Copy binary to your preferred location or run binary from build directory

    $ ./target/release/rolldice 2d6+6

## Prebuilt binaries

Some prebuilt binaries exist under ```builds``` direcory. Binaries are provided as is and without any guarantees.
