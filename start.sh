#check screen install or tmux
if [ -x "$(command -v screen)" ]; then
  screen -DRSq game_cloud ./game_cloud
elif [ -x "$(command -v tmux)" ]; then
  tmux new-session -As ./game_cloud
else
  echo "No screen or tmux installation found, you need to install at least one of them to run GameCloud"
  exit 1
fi

