# Hello Cargo!

- In this section we will learn about cargo.
- Cargo is a package manager and build system for Rust. 
- Cargo handles a lot of things. Such as :
  - handling dependencies
  - building the code
  - etc

## Installation
<!-- TODO --> add installation steps.

## Use
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
  
`#### 2. target 
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