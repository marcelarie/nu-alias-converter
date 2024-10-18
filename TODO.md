# TODO:

**Parsing Bash Aliases**

- [x] Use tree-sitter to parse Bash and get all aliases
- [ ] Handle `expand_aliases` and `shopt -s expand_aliases`
  - [ ] Implement recursive handling

**Conversion to Nushell Format**

- [x] Convert aliases to Nushell format
- [x] Validate alias content to ensure it is valid Nushell (with nu-parser)

**File Handling**

- [x] Write the converted aliases to a file
  - [x] Write once
  - [x] Use Nushell `env.nu`
- [x] Handle multiple files in a directory
- [ ] Handle non-Bash script files (zsh, fish)
- [x] Add `.alias_ignore` file to skip certain aliases during conversion
  - [x] Ignore aliases by name
  - [x] Ignore aliases by command
  - [x] Check for .alias_ignore in the current directory
  - [ ] Check for .alias_ignore in nu config directory
- [x] Handle cases when no aliases are found in the file
- [x] Handle empty files

**Flags and Modes**

- [x] Add `--help` flag
- [x] Add `--no-comments` flag
- [x] Handle `*.nu` files error
- [x] Add `--output` flag to specify the output file path and name
- [x] Handle missing files
- [x] Add debug mode

**Performance**

- [x] Use rayon to parallelize the conversion process
  - Improved from 25 seconds to 5 seconds with a 28888 line aliases file ✅
