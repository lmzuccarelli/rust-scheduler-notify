#!/bin/bash

echo -e "\n"
du -a $1 | sort -n -r | head -n 10
