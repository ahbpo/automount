#!/bin/bash

device=$1
mountpoint=$2
is_root=0
if [ "$(whoami)" == "root" ]; then
	echo "root"
	is_root=1
fi

if [ ! -b "$device" ]; then
	echo "not a valid device file"
	exit 1
elif [ ! -d "$mountpoint" ]; then
	echo "mount point is not valid"
	exit 1
else
	if [ $is_root -eq 1 ]; then
		mount $device
	else
		sudo mount $device $mountpoint
	fi
fi
