# Enabled debug

Sometimes it's helpful to have some more information during
execution. That is where debug mode helps:

    cargo run --features=debug

# Building Disp

Currently the Rust package does an unsavory hack of exposing c abi functions in an executable. In order to accomplish this, additional configuration needs to be set in a .cargo/config file in the home directory, with the following contents:

  [target.x86_64-unknown-linux-gnu]
  rustflags = ["-C", "link-args=-Wl,-export-dynamic"]
