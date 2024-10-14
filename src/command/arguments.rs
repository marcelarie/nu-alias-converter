pub struct CliArgs {
    pub file_path: String,
    pub no_comments: bool,
    pub debug_mode: bool,
}

pub struct GatheredArgs {
    file_path: Option<String>,
    no_comments: bool,
    debug_mode: bool,
    #[allow(unused)]
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
                _ => None,
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

    pub fn new() -> Result<Self, &'static str> {
        let arguments_config = Self::gather();

        match arguments_config.file_path {
            Some(file_path) => Ok(Self {
                file_path,
                no_comments: arguments_config.no_comments,
                debug_mode: arguments_config.debug_mode,
            }),
            None => Err("No script name provided"),
        }
    }
}
