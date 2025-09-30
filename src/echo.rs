#[cfg(feature = "echo")]
pub fn run(mut args: &[String]) {
    let mut newline = true;

    while let Some(arg) = args.first() {
        if arg == "-n" {
            newline = false;
            args = &args[1..];
        } else {
            // TODO: handle other options like -e, -E
            break;
        }
    }

    let mut first = true;
    for arg in args {
        if !first {
            print!(" ");
        }
        print!("{}", arg);
        first = false;
    }
    
    if newline {
        println!();
    }
}