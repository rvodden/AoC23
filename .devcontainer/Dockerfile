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
		bash-completion \
		unzip \
		ssh \
		git \
		curl \
		libssl-dev \
		rust-gdb \
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

RUN git clone --branch stable https://github.com/nojhan/liquidprompt.git /root/.liquidprompt
RUN echo '[[ $- = *i* ]] && source ~/.liquidprompt/liquidprompt' >> /root/.bashrc
RUN echo 'export CARGO_HOME=/root/.cargo' >> /root/.bashrc
RUN mkdir -p /root/.local/share/bash-completion/completions/
RUN printf '. <(rustup completions bash)\n'       >/root/.local/share/bash-completion/completions/rustup
RUN printf '. <(rustup completions bash cargo)\n' >/root/.local/share/bash-completion/completions/cargo
RUN CARGO_HOME=/root/.cargo cargo install just cargo-generate
RUN echo 'export PATH=${PATH}:/root/.cargo/bin' > /root/.bashrc

USER vscode
RUN git clone --branch stable https://github.com/nojhan/liquidprompt.git /home/vscode/.liquidprompt
RUN echo '[[ $- = *i* ]] && source ~/.liquidprompt/liquidprompt' >> /home/vscode/.bashrc
RUN echo 'export CARGO_HOME=/home/vscode/.cargo' >> /home/vscode/.bashrc
RUN echo 'export PATH=${PATH}:/home/vscode/.cargo/bin'
RUN mkdir -p /home/vscode/.local/share/bash-completion/completions/
RUN printf '. <(rustup completions bash)\n'       >/home/vscode/.local/share/bash-completion/completions/rustup
RUN printf '. <(rustup completions bash cargo)\n' >/home/vscode/.local/share/bash-completion/completions/cargo
RUN CARGO_HOME=/home/vscode/.cargo cargo install just cargo-generate
RUN echo 'export PATH=${PATH}:/home/vscode/.cargo/bin' > /home/vscode/.bashrc