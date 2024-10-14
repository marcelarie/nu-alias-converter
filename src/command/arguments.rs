pub struct CliArgs {
    pub file_path: String,
    pub no_comments: bool,
    pub debug_mode: bool,
}

struct GatheredArgs {
    file_path: Option<String>,
    no_comments: bool,
    debug_mode: bool,
    arguments: Vec<String>,
    #[allow(unused)]
    remaining_args: Vec<String>,
}

impl CliArgs {
    fn gather() -> GatheredArgs {
        let mut arguments: Vec<String> = Vec::new();
        let mut script_name = None;
        let mut args = std::env::args();
        let mut no_comments = false;
        let mut debug_mode = false;

        // Skip the program name
        args.next();

        while let Some(arg) = args.next() {
            if !arg.starts_with('-') && script_name.is_none() {
                script_name = Some(arg);
                // TODO: Check if this should be continue or break
                continue;
            };

            let flag_value = match arg.as_ref() {
                "--no-comments" | "-nc" => {
                    no_comments = true;
                    Some(arg.to_string())
                }
                "--debug" | "-d" => {
                    debug_mode = true;
                    Some(arg.to_string())
                }
                "--help" | "-h" => Some(arg.to_string()),
                _ => {
                    let chars = arg.chars().collect::<Vec<char>>();
                    for (index, c) in chars.iter().enumerate() {
                        if c == &'-' {
                            println!("Invalid flag: {}", arg);
                            println!("Use -h for help");
                            std::process::exit(1);
                        }
                        let flag = format!("-{}", c);
                        let value =
                            arg.chars().skip(index + 1).collect::<String>();
                        arguments.push(flag);
                        arguments.push(value);
                        break;
                    }
                    None
                }
            };

            if let Some(flag_value) = flag_value {
                arguments.push(flag_value);
            }
        }

        GatheredArgs {
            arguments,
            file_path: script_name,
            no_comments,
            debug_mode,
            remaining_args: args.collect(),
        }
    }

    fn print_help() {
        println!(
            "Usage: {} [options] <script>",
            std::env::args()
                .next()
                .unwrap_or_else(|| "nu-alias-converter".to_string())
        );
        println!();
        println!("Options:");
        println!("  --no-comments, -nc  Do not print comments");
        println!("  --debug,       -d   Print debug information");
        println!("  --help,        -h   Print this help message");
    }

    pub fn new() -> Result<Self, &'static str> {
        let gathered = Self::gather();
        let is_help_request = gathered
            .arguments
            .iter()
            .any(|arg| arg == "--help" || arg == "-h");

        if is_help_request {
            Self::print_help();
            std::process::exit(0);
        }

        let file_path = gathered.file_path.ok_or("No script name provided")?;

        Ok(Self {
            file_path,
            no_comments: gathered.no_comments,
            debug_mode: gathered.debug_mode,
        })
    }
}
