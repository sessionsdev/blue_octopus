pub struct ParsedArg {
    pub name: Option<String>,
    pub string_value: Option<String>,
}

struct CliOption {
    name: &'static str,
    value_required: bool,
}

pub fn parse_args(args: Vec<String>) -> Vec<ParsedArg> {
    let mut parsed_commands = Vec::new();
    let allowed_options = vec![
        CliOption {name: "--add", value_required: true},
        CliOption {name: "--remove", value_required: false},
    ];
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        if let Some(cli_option) = allowed_options.iter().find(|&opt| opt.name == arg.as_str()) {
            if cli_option.value_required {
                if i + 1 < args.len() && !allowed_options.iter().any(|opt| opt.name == args[i + 1].as_str()) {
                    parsed_commands.push(ParsedArg {
                        name: Some(arg.clone()),
                        string_value: Some(args[i + 1].clone()),
                    });
                    i += 2; // skip the next arg as it's used as a value
                } else {
                    println!("Error: Option '{}' requires a value.", arg);
                    i += 1;
                }
            } else {
                parsed_commands.push(ParsedArg {
                    name: Some(arg.clone()),
                    string_value: None,
                });
                i += 1;
            }
        } else {
            println!("Error: Unknown option '{}'.", arg);
            i += 1;
        }
    }

    parsed_commands
}
