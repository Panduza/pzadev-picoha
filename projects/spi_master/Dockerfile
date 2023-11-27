FROM rust:bullseye as openocd-build-stage

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y \
    git \
    make \
    libusb-1.0-0-dev \
    libtool \
    pkg-config \
    autoconf \
    automake \
    texinfo \
    libhidapi-dev

RUN git clone https://github.com/openocd-org/openocd.git -b v0.12.0 && \
    cd openocd && \
    ./bootstrap && \
    ./configure && \
    make && \
    make install

FROM rust:bullseye as picotool-build-stage

ARG PICOTOOL_VERSION=1.1.1
ARG PICOTOOL_SDK_VERSION=1.5.0

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y \
    git \
    build-essential \
    cmake \
    libusb-1.0-0-dev \
    pkg-config

RUN git clone https://github.com/raspberrypi/pico-sdk.git -b ${PICOTOOL_SDK_VERSION}

RUN git clone https://github.com/raspberrypi/picotool.git -b ${PICOTOOL_VERSION} && \
    mkdir build && \
    cd build && \
    PICO_SDK_PATH=/pico-sdk cmake /picotool && \
    make && \
    make install

FROM rust:bullseye

# Arguments
ARG SERIALGID=20 # GID of group used to access serial devices

RUN apt-get update -y
RUN apt-get install -y sudo

# Tools
RUN apt-get install -y \
build-essential \
pkg-config \
gdb-multiarch \
vim

# openocd dependencies
RUN apt-get install -y libhidapi-dev libncurses5 libusb-1.0-0-dev

# Dev. dependencies
RUN apt-get install -y libudev-dev

# Create dev. user
RUN delgroup dialout
RUN addgroup dialout --gid ${SERIALGID}
RUN adduser builder
RUN usermod -aG dialout builder
RUN adduser builder plugdev

# Configure sudo
RUN echo "builder ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

# Install toolchain
RUN rustup target install thumbv6m-none-eabi

# Install various tools
USER builder
ENV CARGO_HOME=/home/builder/.cargo
RUN cargo install elf2uf2-rs
RUN cargo install cargo-generate
RUN cargo install flip-link
RUN cargo install probe-rs --features=cli --locked

# RUN useradd --create-home builder -s /bin/bash
# RUN adduser builder plugdev
# RUN adduser builder dialout

# add openocd to the docker image
COPY --from=openocd-build-stage /usr/local/bin /usr/local/bin
COPY --from=openocd-build-stage /usr/local/share /usr/local/share

COPY --from=picotool-build-stage /usr/local/bin /usr/local/bin