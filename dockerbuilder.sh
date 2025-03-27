#!/bin/bash

sudo docker build --no-cache -t jserranojunior/tubedrop:latest . && 
sudo docker push jserranojunior/tubedrop:latest