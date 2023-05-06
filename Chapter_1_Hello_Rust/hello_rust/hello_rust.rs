// here is the simple program to print "Hello, Rust!" string into the console.
// main execution of the program will start from [main] method.

fn main() {
    // ! [println!] is a Rust macro. (you'll learn more about Rust macro later. So chill!)
    // The following statement will simply print "Hello, Rust!".
    // ! The line will end with ";". (Otherwise it gives error if you write any other code after that.)
    println!("Hello, Rust!");
}

/*
! Now to run the code:
(For windows)
1. Go to terminal.
2. Enter the command "rustc .\hello_rust.rs".
3. It will generate following files if is executed successfully.
    i. hello_rust.exe
    ii. hello_rust.pdb

* Here [hello_rust.exe] is an executable file which can be run from terminal and second is hello_rust.pdb which consist all the debugging information.

4. Now to execute the the program enter the command ".\main.exe".
5. And it will prompt the output "Hello, Rust!" on the screen.
*/
