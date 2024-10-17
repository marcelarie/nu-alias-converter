OS=`echo \`uname\` | tr '[:upper:]' '[:lower:]'`
AURL="https://gist.githubusercontent.com/hightemp/5071909/raw/"
ANAME=".bash_aliases"
TMPAPATH="/tmp/$ANAME"
HOMEAPATH="~/$ANAME"

[ "$OS" = "windowsnt" ] && OS_WIN="yes"
[ "$OS" = "darwin" ]    && OS_MAC="yes"
[ "$OS" = "linux" ]     && OS_LIN="yes"

# Self-update
alias alias_update="rm -f $TMPAPATH;wget $AURL -O $TMPAPATH;mv $TMPAPATH $HOMEAPATH;source $HOMEAPATH"
