Installing / Building Disp
==========================

Disp is currently only available by building from source. To do so, you will need:

* `the Rust programing language tools <https://rustup.rs/>`_
* `LLVM 7.0 <http://llvm.org/docs/GettingStarted.html>`_ (see your operating system's package manager, there is probably an easier way to build rather than from source.)

Currently the Rust package does an unsavory hack of exposing c abi functions in an executable (so that llvm can read such external functions). In order to accomplish this, additional configuration needs to be set in a .cargo/config file in the $HOME directory, with the following contents::

  [target.x86_64-unknown-linux-gnu]
  rustflags = ["-C", "link-args=-Wl,-export-dynamic"]

Once the above is satisfied, you can build Disp by running::

  cargo build --release

And for development::

  cargo run <file_to_run>


Developing Disp
***************

Enabling Debug Features
-----------------------

Sometimes it's helpful to have some more information during
execution. That is where debug mode helps:

    cargo run --features=debug

Additional output will be printed during disp file compilation, such as:

* grammar parser output
* LLVM IR
