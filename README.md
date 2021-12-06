# wmsel
Simple terminal display manager with Xorg and Wayland Session support written in python.

## Installation
To install wmsel `git clone` and `make install` as the root user.

## Enabling wmsel
To enable wmsel on login just execute wmsel at the end of your shells config file if you are in a TTY.
<br />Examples: `if [ $TERM = alacritty ];then wmsel;fi` for POSIX-compliant shells like bash or `if [ $TERM = linux ];wmsel;end` for fish 
