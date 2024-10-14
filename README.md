# Nu alias converter

> [!NOTE]  
> This project is for educational purposes. I aim to learn more about tree-sitter and Rust.

TODO:

- [ ] Bash aliases converter (bash to nushell)
  - [x] Use treesitter to parse bash and get all the aliases
    - [ ] Handle `expand_aliases` and `shopt -s expand_aliases`
      - [ ] Needs to be recursive
  - [x] Convert them to nushell format
    - [x] Validate the content of the alias to check if it is valid nushell
  - [x] Write them to a file
    - [x] Single time
    - [ ] Use nushell env
  - [ ] Add a command to source the file
  - [ ] Handle multiple files in a directory

## How?

The CLI app will be written in Rust, leveraging libraries from Nushell (which is
also written in Rust). It will use
[treesitter](https://github.com/tree-sitter/tree-sitter) to parse the bash
script and get all the aliases.

The aliases will then be converted to the nushell format using the
[nu-parser](https://github.com/nushell/nushell/tree/main/crates/nu-parser)
crate. After converting, the aliases will be validated, if an alias is not
valid it will be generated as a comment with the information of the parsing
error. So the user can check it and fix it manually. It would be nice to auto
generate the rust code from the content of the alias is the parsing fails but
this is not a priority for now.

The converted aliases are written to a file. You can either generate them
manually or use the Nushell environment. If using the environment method, the
file will regenerate at the start of each shell session.

## Usage

current implementation:
```bash
nu-alias-converter .bash_aliases # will generate a alias.nu file in the same directory
```

in the future it will work like this:
```bash
nu-alias-converter .bash_aliases --out /path/to/nushell/nushell_aliases.nu
```
