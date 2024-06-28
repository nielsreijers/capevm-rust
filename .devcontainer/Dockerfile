FROM fedora:40

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN sudo dnf -y install avr-gcc
RUN sudo dnf -y install avr-libc
RUN sudo dnf -y install openssl-devel
RUN sudo dnf -y group install "Development Tools"
RUN sudo dnf -y install java

# from https://code.visualstudio.com/remote/advancedcontainers/add-nonroot-user
# Create the user
RUN groupadd --gid $USER_GID $USERNAME
RUN useradd --uid $USER_UID --gid $USER_GID -m $USERNAME
RUN sudo usermod -aG wheel $USERNAME
RUN sudo echo 'vscode:1234' | chpasswd
# [Optional] Set the default user. Omit if you want to keep the default as root.
USER $USERNAME

RUN sudo -u vscode curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN . "$HOME/.cargo/env" && cargo install cargo-generate
RUN . "$HOME/.cargo/env" && cargo install cargo-expand
