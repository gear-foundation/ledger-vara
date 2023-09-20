# Vara Application for Ledger Nano S / S PLus / X

Vara application for Ledger Nano S, S Plus, and X.

## Prerequisites

0. Install Rust using [`rustup`](https://rustup.rs/):

    ```
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

3. Install [`ledgerctl`](https://github.com/LedgerHQ/ledgerctl):

    ```
    pip3 install --upgrade protobuf setuptools ecdsa
    pip3 install ledgerwallet
    ```

2. Install [`cargo-ledger`](https://github.com/LedgerHQ/cargo-ledger):

    ```
    cargo install cargo-ledger --git https://github.com/LedgerHQ/cargo-ledger
    cargo ledger setup
    ```

3. Install the toolchain:

    Ubuntu:

    ```bash
    sudo apt install clang gcc-arm-none-eabi gcc-multilib
    ```

    macOS:

    ```bash
    brew install arm-none-eabi-gcc
    ```
## Build

0. Clone this repo:

    ```bash
    git clone https://github.com/gear-foundation/ledger-vara
    cd ledger-vara
    ```

1. Build the app:

    ```
    export CPATH=`xcrun --show-sdk-path`/usr/include # macOS only

    cargo ledger build nanos
    cargo ledger build nanosplus
    cargo ledger build nanox
    ```
