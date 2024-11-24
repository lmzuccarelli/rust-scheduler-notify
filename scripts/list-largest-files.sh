#!/bin/bash

echo -e "\n"
du -ha $1 | sort -n -r | head -n 10
