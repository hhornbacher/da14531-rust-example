# Rust example application for DA14531

## Setting up your environment

This description is written for Ubuntu / Linux Mint.

First we need to setup the environment, therefor we need to copy the [SDK](https://www.renesas.com/eu/en/products/interface-connectivity/wireless-communications/bluetooth-low-energy/da14531-smartbond-ultra-low-power-bluetooth-51-system-chip#design_development) (Section: Software Downloads) to this directory and give it the name `sdk`:

```bash
mv /path/to/my/sdk/6.0.16.1144 ./sdk
```

Next we can either build the dockerized buildsystem image or install all dependencies on our own.

### Dockerized (recommended)

Docker needs to be installed for this to work!

Just run following command:

```bash
./buildsystem.sh
```

This will spawn a bash shell and you can continue with building the project.

### Own system

In order to build the project we need to run following commands to install all dependencies:

```bash
# Install system dependencies
sudo apt-get install -y \
        build-essential gcc-arm-none-eabi \
        binutils-arm-none-eabi libclang1 cmake \
        curl libnewlib-arm-none-eabi

# Install rust toolchains with rustup (nightly, with target thumbv6m-none-eabi)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --default-toolchain nightly --target thumbv6m-none-eabi
```

After these steps you should be ready to compile the project.


## Building the project

In order to build the project we need to execute following commands:
```bash
# This one ONLY if you're using the docker build system
./buildsystem.sh

# If not using docker you will need to replace the SDK_PATH in example-project/CMakeLists.txt with your own absolute path to the SDK!!

# Create the build directory, here you'll find the elf/bin/hex files after compilation
mkdir build
cd build/

# Run cmake to setup build environment
# (Still neccessary, because rust's ldd makes problems with the linker script, so we're using gcc's ld for linking)
cmake ..

# Start the build process
make

```