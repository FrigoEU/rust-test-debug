use std::env::args;

mod advent1;
mod advent2;

fn main() {
    let cmdargs = args();
    let arg = cmdargs.filter(|arg| arg.contains("--advent")).next();
    println!("hallo");
    match arg.as_deref() {
        None => panic!("No --advent<i> argument found"),
        Some("--advent1") => advent1::main(),
        Some("--advent2") => advent2::main(),
        Some(a) => panic!("Couldn't find function for {}", a),
    }
}
