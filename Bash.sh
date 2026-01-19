cargo clean
sccache --show-stats
cargo build
sccache --show-stats
apt-get install build-essential autoconf libssl-dev libyaml-dev zlib1g-dev libffi-dev libgmp-dev rustc
