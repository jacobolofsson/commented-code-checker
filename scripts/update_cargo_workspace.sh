# install cargo if not already avaliable
which cargo || curl https://sh.rustup.rs -sSf | sh -s -- -y
# update lock-file with new version
cargo update --workspace
