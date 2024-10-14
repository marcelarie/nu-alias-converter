# Bash alias converter (to Nushell)

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

The cli app will be written in Rust. It will use the `treesitter` library to
parse the bash script and get all the aliases.

The aliases will then be converted to the nushell format. This implicates that it
will check with the nushell treesitter parser that all the aliases are valid.
If not it will add them as a comment in the alias file so the user can fix them
or create the nushell alternative.

The converted aliases will be written to a file using a manual generation or
nushell env. When using fish env the file will be generated on each shell start.
