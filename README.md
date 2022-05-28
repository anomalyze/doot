# doot

Small program to create symlinks to all your dotfiles. Configure by creating a config (.doot.yaml), and placing it in your home directory. 

doot does not overwrite any files, so if they exist the program will just skip the symlink and continue.

## Installation
```bash
$ git clone git@github.com:anomalyze/doot.git
$ cd doot
$ cargo build --release
$ cargo install --path .
```

## Example config

The program looks for a .doot.yaml file located in the home directory or the current working directory.

```yaml
dotfiles:
- name: sway
  src_path: $HOME/dotfiles/configs/sway.config # symlink specific file
  dst_path: $HOME/.config/sway/config

- name: kitty
  src_path: $HOME/dotfiles/configs/waybar # symlink a folder
  dst_path: $HOME/.config/waybar
  ```