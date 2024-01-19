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

## Variables

### Declaration

`Rust` is typed, however, you can declare a variable without a explicit type:

```rust
// Explicit
let hello_message: &str = "Hello, world!";
```

```rust
// No explicit
let hello_message = "Hello, world!";
```

But the VSCode's extension [rust-analyzer](https://rust-analyzer.github.io/)
suggested me to do it explicitly.

`Rust` has some `primitive types`:

- Numbers
  - Integer: (Signed (i) and Unsigned(u)): you must specify it's bit size (8-bit,
    16-bit, 32-bit, 64-bit or 128-bit). However, you can set a bit size that
    depends on the kind of computer that runs the program (isize, usize).
  - Floating point: (f32, f64). On modern CPUs, both have approximately the same
    speed but f64 has grater precision.
- Booleans
- Characters

It also has more complex ones like `Strings Slice` (Strings, &str and more) or
`Tuples`.

> Characters and Strings Slice are valid `UTF-8` representations (like emojis).

### Displaying

To display it's value you need the `println!` function that needs a string
literal: `"{}", <var name>`:

```rust
println!("{}", hello_message);
```

### Modifying variables

#### Types

If you try to assign a value of a different type you will get an error
because Rust is typed:

```rust
let hello_message: &str = "Hello, world!";

hello_message = 5;
```

```text
error[E0308]: mismatched types
  --> src\main.rs:18:19
   |
1  |   let hello_message: &str = "Hello, world!";
   |                      ---- expected due to this type
3  |   hello_message = 5;
   |                   ^ expected `&str`, found integer
```

#### Values

You can't modify the content of any variable because are like constants:

```rust
let hello_message: &str = "Hello, world!";

hello_message = "Bye!";
```

```text
error[E0384]: cannot assign twice to immutable variable `hello_message`
  --> src\main.rs:33:3
   |
1  |   let hello_message: &str = "Hello, world!";
   |       -------------
   |       |
   |       first assignment to `hello_message`
   |       help: consider making this binding mutable: `mut hello_message`
   |
3  |   hello_message = "Bye!";
   |   ^^^^^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
```

To do so you must add `mut` tag when you declare a variable that can be mutable:

```rust
let mut hello_message: &str = "Hello, world!";
```

However, you can `shadow` variables, even without the `mut` tag, by declaring
them again. This is like overriding it:

```rust
let hello_message: &str = "Hello, world!";

let hello_message: i32 = 5;

let hello_message: i32 = hello_message + 27;

println!("Your number is {}", hello_message)
```

But in this example, the compiler will warn me because I'm not using the first
value of `hello_message`:

```text
warning: unused variable: `hello_message`
 --> src\main.rs:6:7
  |
1 |   let hello_message: &str = "Hello, world!";
  |       ^^^^^^^^^^^^^ help: if this is intentional, prefix it with an
  |                           underscore: `_hello_message`
  |
  = note: `#[warn(unused_variables)]` on by default
```
