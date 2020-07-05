# Debian | Ubuntu | Arch based distributions

`git clone https://github.com/iinc0gnit0/RVuln`

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

`source $HOME/.cargo/env`

`cd RVuln`

Debian/Ubuntu based distributions: `sudo apt install openssl-dev`

Arch based distrobutions: `sudo pacman -S openssl`

`cargo build --release`

`mv target/release/RVuln .`

# Windows 10 (won't give you the best experience)

Go to the follwoing link to install Rust https://turreta.com/2019/09/06/how-to-install-rust-on-windows-10/

Then download the zip https://github.com/iinc0gnit0/RVuln/archive/master.zip and extract it

Go to the directory RVuln directory in and use do `cargo build --release`

The executable will be located at `target/release/RVuln.exe`
