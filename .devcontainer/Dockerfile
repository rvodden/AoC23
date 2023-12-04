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
RUN  curl https://sh.rustup.rs -sSf | sh -s -- -y

RUN useradd -ms /bin/bash vscode \
		&& usermod -aG www-data,sudo vscode

RUN echo "%sudo ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

RUN git clone --branch stable https://github.com/nojhan/liquidprompt.git ~/.liquidprompt
# Only load Liquidprompt in interactive shells, not from a script or from scp
RUN echo "[[ $- = *i* ]] && source ~/.liquidprompt/liquidprompt" >> ~/.bashrc

USER vscode
RUN cargo install just cargo-generate