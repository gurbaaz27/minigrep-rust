extern crate clap;
use clap::{Arg, App};

use std::process;
use minigrep_rust::Config;

fn main() {
	let matches = App::new("My Own Rust CLI MiniGrep!")
			.author("Gurbaaz Singh Nandra <gurbaaz27.github.io>")
			.about("Returns the lines in file containing your query.")
			.arg(Arg::with_name("query")
					.short("q")
					.long("query")
					.takes_value(true)
					.required(true)
					.help("String you want to search in the file."))
			.arg(Arg::with_name("filename")
					.short("f")
					.long("filename")
					.takes_value(true)
					.required(true)
					.help("String of filename(with extension) you want to search in."))
			.arg(Arg::with_name("case_sensitive")
					.short("c")
					.long("case_sensitive")
					.takes_value(true)
					.required(true)
					.default_value("y")
					.help("Whether the search be case-insensitive(n) or sensitive(y)."))
			.get_matches();
	
	let config = Config::new(matches).unwrap_or_else(|err| {
    	eprintln!("Problem passing case_sensitive argument : {}",err);
    	process::exit(1);
    });

    
    if let Err(e) = minigrep_rust::run(config){
    	eprintln!("Application error : {}",e);
    	process::exit(1);
    }
}
