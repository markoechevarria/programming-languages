#!/bin/bash

command=$(python --version)
output=$?

if [ $output -ne 0 ]; then
	echo "Do you want to install python? [Yes] / No"
	read answer
	if [ "$answer" = "Yes" -o -z "$answer" ]; then
		echo "The answer was Yes"
	fi
else
	python -m venv checkip
	./checkip/Scripts/activate
	pip install -r requirements.txt
fi
