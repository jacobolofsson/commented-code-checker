# install cargo if not already avaliable
which cargo || curl https://sh.rustup.rs -sSf | sh -s -- -y && source "$HOME/.cargo/env"
# update lock-file with new version
cargo update --workspace
