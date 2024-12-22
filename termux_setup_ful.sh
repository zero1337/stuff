#!/bin/bash
set -e

pkg update
pkg upgrade

termux-setup-storage

pkg install x11-repo
pkg install xfce4 tigervnc

vncserver -localhost
## set the password now

vncserver -kill :1

echo "xfce4-session &" > ~/.vnc/xstartup

vncserver -localhost

##     Open the VNC viewer app and create a new connection with the following details:
##     Address: 127.0.0.1:5901
##     close the server: vncserver -kill :1

pkg install firefox
