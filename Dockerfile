# Basic Rust development container based on Ubuntu 22.04
FROM ubuntu:22.04

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=1000

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    git \
    pkg-config \
    libssl-dev \
    libclang-dev \
    clang \
    cmake \
    openssh-client \
    tzdata \
    gnupg \
 && rm -rf /var/lib/apt/lists/*

# Create a non-root user for development
RUN groupadd --gid ${USER_GID} ${USERNAME} \
 && useradd -s /bin/bash --uid ${USER_UID} --gid ${USER_GID} -m ${USERNAME} \
 && mkdir -p /home/${USERNAME}/.cargo /home/${USERNAME}/.rustup \
 && chown -R ${USERNAME}:${USERNAME} /home/${USERNAME}

ENV HOME=/home/${USERNAME}
ENV PATH=${HOME}/.cargo/bin:${PATH}

# Install rustup and the stable toolchain, then add common components
USER ${USERNAME}
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable \
 && rustup component add rust-src clippy rustfmt \
 && cargo install cargo-edit || true

USER root

WORKDIR /workspace

# Default to the non-root user
USER ${USERNAME}

CMD ["/bin/bash"]
