// install curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
// update rustup update
// desinstall rustup self uninstall
// check version rustc --version
// local doc: rustup doc

fn main()
{
    /*
    main function is always the first code that runs in every executable rust program
    if 'functions' had a !,so its a macro

    compile with rustc 01_hello_world.rs

    rustc is a basic compiler, Cargo is the best
    rust is an ahead-of-time compiled diferent of python,for example.

    Cargo is a compiler for complex programs that uses dependeces
    create a cargo project:
    cargo new project_name

    to build a cargo:

    cargo build

    this make a .cargo-lock with dependencies and a executable in target dir

    to execute:

    cargo run

    to check with:

    cargo check

    for production env:

    cargo build --release
    */
    println!("Hello, world!");
}
