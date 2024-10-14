pub struct CliArgs {
    pub file_path: String,
}

pub struct GatheredArgs {
    #[allow(unused)]
    arguments: Vec<String>,
    file_path: Option<String>,
    #[allow(unused)]
    remaining_args: Vec<String>,
}

impl CliArgs {
    fn gather() -> GatheredArgs {
        let mut arguments: Vec<String> = Vec::new();
        let mut script_name = None;
        let mut args = std::env::args();

        // Skip the program name
        args.next();

        while let Some(arg) = args.next() {
            if !arg.starts_with('-') {
                script_name = Some(arg);
                break;
            };

            let flag_value = match arg.as_ref() {
                "--test-flag" => args.next().map(|x| x.to_string()),
                _ => None,
            };

            if let Some(flag_value) = flag_value {
                arguments.push(flag_value);
            }
        }

        GatheredArgs {
            arguments,
            file_path: script_name,
            remaining_args: args.collect(),
        }
    }

    pub fn new() -> Result<Self, &'static str> {
        let gathered = Self::gather();

        match gathered.file_path {
            Some(file_path) => Ok(Self { file_path }),
            None => Err("No script name provided"),
        }
    }
}
