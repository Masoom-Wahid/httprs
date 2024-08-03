HTTPRS_VERSION="0.1.0"
HTTPRS_PATH="$HOME/.httprs"
EXEC_PATH="$HTTPRS_PATH/bin/httprs"
ALIAS_COMMAND="alias httprs=$EXEC_PATH"



echo "Installing HttpRs....."

if ! [ -d "$FILIO_PATH" ]; then
  mkdir -p "$HTTPRS_PATH/bin"
fi



curl -L https://github.com/Masoom-Wahid/httprs/releases/download/${HTTPRS_VERSION}/httprs > $EXEC_PATH && chmod +x $EXEC_PATH

if [ "$SHELL" = "/usr/bin/zsh" ]; then
    echo "$ALIAS_COMMAND" >> "$HOME/.zshrc" 2>/dev/null
else
    echo "$ALIAS_COMMAND" >> "$HOME/.bashrc" 2>/dev/null
fi


echo "HttRs installed successfully!"
echo "run 'httprs help' for additional help on commands"







