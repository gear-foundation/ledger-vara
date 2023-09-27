# Vara Application for Ledger Nano S / S Plus / X

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

    macOS only:

    ```bash
    export CPATH=`xcrun --show-sdk-path`/usr/include
    ```

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

    - Unplug device, press left and right buttons simultaneously and while keeping them pressed, plug device back
    - Wait until the menu appears and choose **Recovery mode**

3. Onboard the device with PIN=1111 and mnemonic phrase:

    ```bash
    ledgerctl onboard 1111 "bottom drive obey lake curtain smoke basket hold race lonely fit walk"
    ```

    Wait for a while. Then install the CA:

    ```bash
    ledgerctl install-ca dev
    ```

    Confirm the installation on the device.

4. Load the app:

    ```bash
    cargo ledger build nanosplus --load
    ```

    Confirm the installation on the device.

## Live Demo

1. Go to https://ledger.vara.rs
2. Use arrow keys (← and →) to simulate pressing Ledger's buttons (left and right).
3. Use the down arrow key (↓) to simulate pressing both Ledger's buttons simultaneously.
4. The application will be automatically restarted after quitting. Please reload the page after waiting for 5-10 seconds. The server may return a 502 error for some time; please try to reload the page again.
5. Note that application is stateful and will not work properly if there are several users at the same time.
6. For available APDU commands, see documentation (will be available soon).

# License

The source code is released under the terms of the [Apache License 2.0](LICENSE).
