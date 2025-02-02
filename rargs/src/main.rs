struct Args {
    number: u32,
    shout: bool,
    positional: Vec<String>
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut args = Args {
        number: 1,
        shout: false,
        positional: Vec::new()
    };

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('n') | Long("number") => {
                args.number = parser.value()?.parse()?;
            }
            Long("shout") => {
                args.shout = true;
            }
            Value(val) => {
                args.positional.push(val.string()?);
            }
            Long("help") => {
                println!("Usage: hello [-n|--number=NUM] [--shout] THING");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(args)
}

fn main() -> Result<(), lexopt::Error> {
    let args = parse_args()?;
    println!("number: {}", args.number);
    println!("shout: {}", args.shout);
    println!("Additional: {}", args.positional.join(", "));
    Ok(())
}