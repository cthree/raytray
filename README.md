# The Ray Tracer Challenge (in Rust)

Buy the book --> [The Ray Tracer Challenge](https://www.amazon.ca/Ray-Tracer-Challenge-Test-Driven-Renderer/dp/1680502719)

You'll need the following installed on whatever system/container you are using to write and debug your code:
* [rustup](https://rustup.rs/)
* Visual Studio Build Tools/Xcode Tools/Clang, The native C/C++ build tools for your platform
* an IDE, use [Visual Studio Code](https://code.visualstudio.com/), not Visual Sudio, or Jetbrains if you have a preference. I use VS Code and that's what I'll describe.

## rustup

Installing the Rust toolchain is easy. Point your browser here: (https://rustup.rs/) and follow the instructions.

## VS Code

The rust toolchain installation had you install the native build tools so that leaves the IDE: VS Code. Click the link (https://code.visualstudio.com/) and install it. Here are some extensions I recommend you install:

* [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml)
* [Rust (rls)](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
* [Rust Test Lens](https://marketplace.visualstudio.com/items?itemName=hdevalke.rust-test-lens)

I also suggest you install [watchexec](https://github.com/watchexec/watchexec) using `cargo install watchexec`. It's invaluable to have running in a terminal or the IDE terminal running your tests each time the local source repository changes, like when you save. I run `watchexec -c cargo test` in an IDE terminal while I work. The trick to TDD is to run the tests as often as possible.

## Rust Resources

Rust has rich documentation and expressive tools [docs.rs](https://docs.rs/) is the portal to documentation, *all of it*. Look under the _Rust_ pulldown for links to the book (_must read_) and other resources.