#!/bin/bash

length=15
s="\u25A0"
c="\u25CB"
dir="l"
declare -A matrix
declare -A snake

create_table() {
    for i in $(seq 0 $(($length-1))); do
        for j in $(seq 0 $(($length-1))); do
            matrix["$i,$j"]="$s"
        done
    done
}

draw_pixel() { 
    matrix["$1,$2"]="$c"
}

draw_snake() {
    for pixel in "${snake[@]}"; do
        read -ra strarr <<< $pixel
        printf "${strarr[0]} ${strarr[1]}\n"
        draw_pixel ${strarr[0]} ${strarr[1]}
    done
}

create_snake() {
    snake[1]="$(($length/2)) 5"
    snake[2]="$(($length/2)) 4"
    snake[3]="$(($length/2)) 3"
    snake[4]="$(($length/2)) 2" 
}

print_table() {
    for i in $(seq 0 $(($length-1))); do
        for j in $(seq 0 $(($length-1))); do
            printf "${matrix["$i,$j"]} "
        done
        printf "\n"
    done
}

get_movement() {
    auxiliar=("${snake[@]}")
    read -rsn 1 key
    if [[ $key == "h" ]]; then
        echo "es la h"     
        # snake[1]=${snake[${#snake[@]}]}
        for i in $(seq 0 "${#snake[@]}" ); do
            if [[ $i == 1 ]]; then
                snake["$1"]=${snake["${#snake[@]}"]}
            elif [[ $i == "${#snake[@]}" ]]; then
                
            else

            fi
        done
        printf "${snake[1]}"
    elif [[ $key == "j" ]]; then
        echo "es la j"
    elif [[ $key == "k" ]]; then
        echo "es la k"
    elif [[ $key == "l" ]]; then
        echo "es la l"
    fi
}

create_table
create_snake
draw_snake

while true; do
    print_table
    get_movement
    sleep 1
done
