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

Rust has rich documentation and expressive tools [docs.rs](https://docs.rs/) is the portal to documentation. Look under the _Rust_ pulldown for links to the book (*_must read_*) and other resources.

## Start the Challenge

The Ray Tracer Challenge presents you with a series of incremental _user stories_ written as a BDD/Cucumber specification. To complete the challenge you'll need to write one of more tests which will pass once you've written the code.

That means you first need to understand how to write a test in Rust and run it using `cargo test` to see if it passes or not. [Chapter 11](https://doc.rust-lang.org/book/ch11-00-testing.html) of the *Rust Book* talks about how to do that. The *Cargo Book* also talks about running automated tests and setting up CI for Rust. I recommend you read both thoroughly as writing tests will be your primary interface to the code.

Before we get too deep I need to point out that I expect you to read the *Rust Book* and all the other documentation sources to learn the Rust language and tooling. This challenge is a way to practice what is in the books but will not teach you to program in Rust.

I suggest you play around with `cargo`, create a new project, build it, run it and get familiar wit hthe process. Once you have the basics down, start a new binary (application) project with `cargo new` and begin.

I will be tagging this project with the completed chapters as I do them. If you want to see what I did, checkout the chapter tag of this repository and you'll get all the code and tests written to complete that chapter. Think about the tests you need to write. Your test should model an ideal API for the unwritten code.