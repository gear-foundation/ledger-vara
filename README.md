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

    Ledger Nano S:

    ```bash
    cargo ledger build nanos
    ```

    Ledger Nano S Plus:

    ```bash
    cargo ledger build nanosplus
    ```

    Ledger Nano X:

    ```bash
    cargo ledger build nanox
    ```

2. Find `app-vara` file in the `target/nano{s|splus|x}/release` directory.

## Upload to the Ledger S Plus

0. **Linux only**. Tune udev rules:

    ```bash
    cd `mktemp -d`
    wget https://raw.githubusercontent.com/LedgerHQ/udev-rules/master/add_udev_rules.sh
    chmod +x add_udev_rules.sh
    sudo ./add_udev_rules.sh
    ```

1. Reset your Ledger to the factory settings:

    - Plug the device and enter PIN to unlock
    - Enter **Settings**, choose **Security**
    - Scroll down to **Reset device** and choose it
    - Enter PIN to confirm hardware reset

2. Enter recovery mode:

    - Unplug device, press right button and while keeping it pressed, plug device back
    - Wait until the welcome screen appears

3. Load the app:

    Ledger Nano S:

    ```bash
    cargo ledger build nanos --load
    ```

    Ledger Nano S Plus:

    ```bash
    cargo ledger build nanosplus --load
    ```

    Ledger Nano X:

    ```bash
    cargo ledger build nanox --load
    ```

# License

The source code is released under the terms of the [Apache License 2.0](LICENSE).
