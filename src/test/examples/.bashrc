# bashrc testing file

# Simple function
function hello() {
  echo "Hello, $1!"
}

# More complex function
function mkcd() {
  mkdir -p "$1" && cd "$1" || exit
}

# Simple aliases
alias ls='ls --color=auto'
alias ll='ls -l'
alias la='ls -A'
alias gitlog='git log --graph --oneline --decorate --all'

# Alias with arguments
alias echo3='echo $1 $2 $3'

# Alias using a variable
alias mydir='cd $HOME/Documents'

# Alias using a function
alias greet='hello World'

# Alias with multiple commands
alias update='sudo apt update && sudo apt upgrade -y'

# Alias with complex command
alias findfile='find . -name'

# Alias with environment variable
alias envpath='echo $PATH'

# Alias for system information
alias sysinfo='echo "OS: $(uname -s)"; echo "Kernel: $(uname -r)"; echo "Shell: $SHELL"'
