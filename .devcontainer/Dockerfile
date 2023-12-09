FROM ubuntu:22.04

RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		g++ \
		gcc \
		libc6-dev \
		make \
		pkg-config \
		wget \
		sudo \
		less \
		ca-certificates \
		unzip \
		ssh \
		git \
		curl \
		libssl-dev \
	; \
	rm -rf /var/lib/apt/lists/*



# Install Rust for the correct CPU architecture
ENV RUST_VERSION=1.74.1
ENV RUSTUP_HOME=/opt/rust
ENV CARGO_HOME=/opt/rust
ENV PATH=${PATH}:/opt/rust/bin
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path

WORKDIR /tmp
RUN rm -rf "${TMP_RUST_DIR}"

RUN useradd -ms /bin/bash vscode \
		&& usermod -aG www-data,sudo vscode

RUN echo "%sudo ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

RUN git clone --branch stable https://github.com/nojhan/liquidprompt.git ~/.liquidprompt
# Only load Liquidprompt in interactive shells, not from a script or from scp
RUN echo '[[ $- = *i* ]] && source ~/.liquidprompt/liquidprompt' >> ~/.bashrc
RUN echo 'export CARGO_HOME=/root/.cargo' >> /root/.bashrc
RUN echo 'export CARGO_HOME=/home/vscode/.cargo' >> /home/vscode/.bashrc

USER vscode
RUN CARGO_HOME=/home/vscode/.cargo cargo install just cargo-generate