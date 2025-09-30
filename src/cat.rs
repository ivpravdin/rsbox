use std::io::BufRead;

#[cfg(feature = "cat")]
pub fn run(mut args: &[String]) {
    let mut number_lines = false;
    let mut number_nonblank = false;
    let mut filenames = Vec::new();

    while let Some(arg) = args.first() {
        if arg == "-n" {
            number_lines = true;
            args = &args[1..];
        } else if arg == "-b" {
            number_nonblank = true;
            args = &args[1..];
        } else if arg.starts_with('-') {
            // TODO handle other options
            eprintln!("cat: invalid option -- '{}'", &arg[1..]);
            break;
        } else {
            break;
        }
    }

    if args.is_empty() {
        filenames.push(String::from("-"));
    } else {
        filenames.extend_from_slice(args);
    }

    let mut reader: Box<dyn BufRead>;
    let mut line_number = 1;

    for filename in filenames {
        reader = if filename == "-" {
            Box::new(std::io::BufReader::new(std::io::stdin()))
        } else {
            match std::fs::File::open(&filename) {
                Ok(file) => Box::new(std::io::BufReader::new(file)),
                Err(e) => {
                    eprintln!("cat: {}: {}", filename, e);
                    continue;
                }
            }
        };
        for line in reader.lines() {
            let line = line.unwrap();
            if number_nonblank {
                if !line.trim().is_empty() {
                    println!("{:6}\t{}", line_number, line);
                    line_number += 1;
                } else {
                    println!();
                }
            } else if number_lines {
                println!("{:6}\t{}", line_number, line);
                line_number += 1;
            } else {
                println!("{}", line);
            }
        }
    }
}