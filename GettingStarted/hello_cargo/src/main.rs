fn main() {
    println!("Hello, world!");
}

// cargo build
// cargo run || ./target/debug/hello_cargo
// cargo check = checks your code to make sure it compiles but doesn't run it

/*
Let’s recap what we’ve learned so far about Cargo:

- We can build a project using cargo build or cargo check.
- We can build and run a project in one step using cargo run.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

Building for Release
When your project is finally ready for release, you can use cargo build --release to compile it with optimizations. 
This command will create an executable in target/release instead of target/debug. 
The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile.
This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. If you’re benchmarking your code’s running time, be sure to run cargo build --release and benchmark with the executable in target/release.
*/