# wmsel
Simple terminal display manager with Xorg and Wayland Session support written in python.

## Installation
To install wmsel `git clone` and `make install` as the root user.

## Enabling wmsel
To enable wmsel on startup just execute wmsel at the end of your shells config if you are in a tty.
Examples: if [ $TERM = alacritty ];then wmsel;fi for POSIX-compliant shells like bash or `if [ $TERM = linux ];wmsel;end` for fish 
