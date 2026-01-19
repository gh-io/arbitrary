git clone https://github.com/rust-lang/docs.rs.git docs.rs
cd docs.rs
git submodule update --init
# Configure the default settings for external services
cp .env.sample .env
# Create the DOCSRS_PREFIX directory
mkdir -p ignored/cratesfyi-prefix/crates.io-index
# Builds the docs.rs binary
SQLX_OFFLINE=1 cargo build
# Start the external services.
docker compose up --wait db s3
# anything that doesn't run via docker-compose needs the settings defined in
# .env. Either via `. ./.env` as below, or via any dotenv shell integration.
. ./.env
# allow downloads from the s3 container to support the /crate/.../download endpoint
mcli policy set download docsrs/rust-docs-rs
# Setup the database you just created
cargo run --bin docs_rs_admin -- database migrate
# Update the currently used toolchain to the latest nightly
# This also sets up the docs.rs build environment.
# This will take a while the first time but will be cached afterwards.
cargo run --bin docs_rs_builder -- build update-toolchain
# Build a sample crate to make sure it works
cargo run --bin docs_rs_builder -- build crate regex 1.3.1
# This starts the web server but does not build any crates.
# It does not automatically run the migrations, so you need to do that manually (see above).
cargo run --bin docs_rs_web
# If you want the server to automatically restart when code or templates change
# you can use `cargo-watch`:
cargo watch -x "run --bin docs_rs_web"
