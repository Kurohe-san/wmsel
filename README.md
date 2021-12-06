# wmsel
Simple terminal display manager with Xorg and Wayland Session support written in python.

## Installation
To install wmsel `git clone` and `make install` as the root user.

## Enabling wmsel
To enable wmsel on startup just add ```if [ $TERM = linux ]\n
	\twmsel\n 
end``` to the end of your shells config.
