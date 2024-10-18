# bash_aliases testing file

shopt -s expand_aliases

# Simple aliases
alias ls='ls --color=auto'                                # Should error with nu::parser::unknown_flag
alias ll='ls -l'                                          # VALID
alias "abc!"='echo String with special characters'        # VALID
alias la= 'ls -A'                                         # This alias is invalid and should be ignored + nu::parser::unknown_flag
alias gitlog='git log --graph --oneline --decorate --all' # VALID
alias invalid_nushell_alias='echo $HOME'                  # Should error with nu::parser::env_var_not_var
alias node15="source /usr/share/nvm/init-nvm.sh"          # Should error
# alias zkn="cd ~/notes; ~/scripts/zk-new"                # TODO: This is a valid alias but it is not being parsed correctly yet

# Cases like this are not handled yet by the Parser
my_function() {
    alias greet='echo Hello, World!'
}
