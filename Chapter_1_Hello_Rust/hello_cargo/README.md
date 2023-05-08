`# Hello Cargo!

- In this section we will learn about cargo.
- Cargo is a package manager and build system for Rust. 
- Cargo handles a lot of things. Such as :
  - handling dependencies
  - building the code
  - etc
`
## Installation
<!-- TODO --> add installation steps.

## Use

### Creating Cargo project
- To use Cargo, we need to create a Cargo project.
```cmd
$ cargo new hello_cargo
$ cd hello_cargo
```
- We can build a cargo project using the command "cargo new *project_name*"
- It will create following files and directories:
  - src (dir)
  - target (dir)
  - .gitignore
  - Cargo.lock
  - Cargo.toml

- Let's see them one by one.

#### 1. src 
- src directory will contain all the source code.
- by default it will contain main.rs file. 
- main.rs file is must in order to build the project and run it.
- main.rs will contain by default following code:
```rust
fn main() {
    println!("Hello, world!");
}
```
  
#### 2. target 
- target directory will contain all the generated files and output files in it.

#### 3 .gitignore
- When we create a cargo project, git is by default initialized and gitignore file is created.
  
#### 4. Cargo.lock
- This file will contain all the dependencies version related info in it.

#### 5. Cargo.toml
- This file will contain all the configurations of the Cargo project.
- By default it will look something like this:
```TOML
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```
- The first line, [package], is a section heading that indicates that the following statements are configuring a package. As we add more information to this file, we’ll add other sections.
- The next three lines set the configuration information Cargo needs to compile your program: the name, the version, and the edition of Rust to use
- The last line, [dependencies], is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as crates. We won’t need any other crates for this project, but we will in the first project in Chapter 2, so we’ll use this dependencies section then.

### Build Cargo Project
- To create a Cargo project you need to run the following command: 
```cmd
$ cargo build
   Compiling hello_cargo v0.1.0 (file_path)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```
- This command will generate an exe file in debug directory of target directory.

### Running the Generated Executable
- There two ways to run the executable file which are shown below.
  - by accessing the exe file from command-line and executing it like this:
    ```cmd
    $ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
    Hello, world!
    ```
  - by firing following command:
    ```cmd
    $ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
    Hello, world!
    ```


### Checking the Code
- If you want to check your for errors and fail safety without producing the binaries then you can use the following command: 
```cmd
$ cargo check
   Checking hello_cargo v0.1.0 (file_path)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```
- *cargo check* is much faster than *cargo build* because it skips the step of producing an executable. If you’re continually checking your work while writing the code, using *cargo check* will speed up the process of letting you know if your project is still compiling.

### Building for Release
- When your project is finally ready for release, you can use *cargo build --release* to compile it with optimizations. This command will create an executable in *target/release* instead of *target/debug*.