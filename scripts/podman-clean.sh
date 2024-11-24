#!/bin/bash

echo -e "\n"
podman rmi -f $(podman images | awk '{print $1":"$3}' | grep none | cut -d':' -f2)
