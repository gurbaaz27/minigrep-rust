# Minigrep implementation in Rust <img src="assets/rust-logo.png" width="40px" height="40px"></img>

Far inferior to grep command, but still it returns lines from a file containing your query string, both case strictly and flexibly by passing a flag in argument.
## *Requirements* 
* [Rust](https://www.rust-lang.org/) 
* [Cargo](https://doc.rust-lang.org/stable/cargo/)
* [Clap](https://github.com/clap-rs/clap)

Clap dependency has been added in the Cargo.toml file already. It provides neat **C**ommand **L**ine **A**rgument **P**arsing, which is essentially its full-form. :blush:

## *Usage*
```bash
git clone link-to-repo
cd minigrep_rust
```
```bash
cargo run -- --help
My Own Rust CLI MiniGrep! 
Gurbaaz Singh Nandra <gurbaaz27.github.io>
Returns the lines in file containing your query.

USAGE:
    minigrep_rust --case_sensitive <case_sensitive> --filename <filename> --query <query>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --case_sensitive <case_sensitive>    Whether the search be case-insensitive(n) or sensitive(y). [default: y]
    -f, --filename <filename>                String of filename(with extension) you want to search in.
    -q, --query <query>                      String you want to search in the file.
```
A "poem.txt" document has been added as sample file in root directory.

### *Example usage (Ubuntu 18.04)*
```bash
➜  minigrep_rust git:(master) ✗ cargo run -- --query the --filename poem.txt --case_sensitive n
   Compiling minigrep_rust v0.1.0 (/home/gurbaaz/rprojects/minigrep_rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.60s
     Running `target/debug/minigrep_rust --query the --filename poem.txt --case_sensitive n`
Then there’s a pair of us - don’t tell!
They’d banish us, you know.
To tell your name the livelong day
```
Do star if you liked it or learnt something, and please :pray: give suggestions for improvement, as working with Rust is surely rusty :dizzy_face:(*definitely for learners*).

:heart:
