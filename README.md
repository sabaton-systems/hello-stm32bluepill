# Drive-By-Wire board STM32

## Dependencies

#### 0. Rust

Install Rust using rustup. See instructions at https://www.rust-lang.org/tools/install .

We use rust nightly for now. Make nightly default by entering the following command.

```
rustup default nightly
# Download the sources for the thumbv7m target
rustup target add thumbv7m-none-eabi
```

#### 1. `flip-link`:

```console
$ cargo install flip-link
```
#### 2. `probe-run`:

``` console
$ # make sure to install v0.2.0 or later
$ cargo install probe-run
```
#### 3. GCC arm toolchain (ubuntu)

sudo apt install gcc-arm-none-eabi

## Getting the source

You already know where the source code is, it is this repository. Clone the repository.


You can compile, flash and run the binary on the Bluepill with this single command,
```
cargo rb  dbw1 --release
```
Connect the STLink programmer to the Bluepill.

## Permission error when trying to flash

You may face a permission error when trying to flash. This can happen if your user does not
have access to the STLink USB device. Follow the instructions at https://stackoverflow.com/questions/23312087/error-3-opening-st-link-v2-device/23321232#23321232 to allow your user id access to the debugger.  Shown below is an example of my working configuration.

```
cat /etc/udev/rules.d/45-usb-stlink-v2.rules
#STLINK V2
ATTRS{idProduct}=="3748", ATTRS{idVendor}=="0483", MODE="666", GROUP="plugdev"
```

