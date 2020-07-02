# Debian | Ubuntu | Arch based distributions

`git clone https://github.com/iinc0gnit0/RVuln`

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

`source $HOME/.cargo/env`

`cd RVuln`

Debian/Ubuntu based distributions: `sudo apt install openssl-dev`

Arch based distrobutions: `sudo pacman -S openssl`

`cargo build --release`

`mv target/release/RVuln .`
