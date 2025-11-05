import os
import time
import random
import keyboard

length=20

def create_table(length):
    return [ ["." for j in range(length)] for i in range(length)]

def create_player(table):
    player = {
        1: [int(len(table)/2), 6 ],
        2: [int(len(table)/2), 5 ],
        3: [int(len(table)/2), 4 ],
        4: [int(len(table)/2), 3 ]
    }
    return player

def print_table_with_player(table, player): 
    for i in range(len(table)):
        for j in range(len(table)):
            value=True
            for key in player.keys():
                if player[key] == [i,j]: 
                    value=False
                    print("■", end=" ")
            if value: print( table[i][j], end=" ")
        print("")

def detect_key():
    pressed = keyboard.read_key()
    if pressed == "h":
        for i in range(len(player), 0, -1):
            if i == 1: player[i][1] = player[i][1]+1
            else: player[i] = player[i-1] 
    if pressed == "j":
        for i in range(len(player), 0, -1):
            if i == 1: player[i][0] = player[i][0]+1
            else: player[i] = player[i-1] 
    if pressed == "k":
        for i in range(len(player), 0, -1):
            if i == 1: player[i][0] = player[i][0]-1
            else: player[i] = player[i-1] 
    if pressed == "l":
        for i in range(len(player), 0, -1):
            print("para debugear", i, player[i])
            if i == 1: player[i][1] = player[i][1]+1
            else: player[i] = player[i-1] 

table = create_table(length)
player = create_player(table)

for i in range(20000):
    os.system("clear")
    print( f"Iteration {i}")
    print_table_with_player(table, player)
    detect_key()
    player = dict(sorted(player.items()))
    print(player)
    time.sleep(3.5)
