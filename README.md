# Rust Learning Path

First steps with Rust following
[Microsoft's Crash
Course](https://learn.microsoft.com/en-us/training/paths/rust-first-steps/).

## Hello, World

There are 2 ways to do so:

1. Create a `.rs` file by you're own. Then, compile and run it:

    ```bash
    # Create a file
    touch main.rc
    ```

    ```bash
    # Compile it
    rustc main.rc
    ```

    ```bash
    # Run it (on Windows)
    .\main.exe
    ```

    ```bash
    # Run it (on Unix)
    ./main
    ```

1. Use `Cargo` (it is installed with `Rust`):

    ```bash
    # Create a working dir
    cargo new hello-cargo
    ```

    I prefer this way because `Cargo` creates a folder structure and adds some
    paths to `.gitignore`. However, It's better to use the [Rust
    .gitignore](https://github.com/github/gitignore/blob/main/Rust.gitignore)
    provided by GitHub.

    `Cargo` creates a `src` folder with a `main.rc` file inside. There is a
    simple `Hello, World!` in Rust:

    ```rust
    fn main() {
      println!("Hello, world!")
    }
    ```

    Furthermore, `Cargo` is like `npm` in a `JS` project. It manages the
    libraries or packages that a project needs. It creates a file named
    `Cargo.toml` that is similar to the `package.json`.

    In order to run the code you must compile it first. With `Cargo`, this steps
    are made together:

    ```bash
    # Compile and run it (inside working dir)
    cargo run
    ```
