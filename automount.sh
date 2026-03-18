#!/bin/bash

device=$1
mountpoint=$2
is_root=0
suffix_char="$"
main_dir=$(pwd)

if [ "$(whoami)" == "root" ]; then
	echo "root"
	is_root=1
	suffix_char="#"
fi

if [ ! -b "$device" ]; then
	echo "not a valid device file"
	exit 1
elif [ ! -d "$mountpoint" ]; then
	echo "mount point is not valid"
	exit 1
else
	if [ $is_root -eq 1 ]; then
		mount $device $mountpoint
	else
		sudo mount $device $mountpoint
	fi
	cd $mountpoint
	bash --rcfile <(echo "PS1=\"(\e[0;36mautomount session\e[0m) \u \w $suffix_char \"")
	cd $main_dir
	if [ $is_root -eq 1 ]; then
		umount $device
	else
		sudo umount $device
	fi
fi
