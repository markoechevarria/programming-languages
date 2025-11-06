import os
import time
import random
import keyboard

last_key="l"

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

def get_last_key(event):
    global last_key
    last_key = event.name

def detect_key():
    move(last_key, player)
    """
    if last_key == "h":
        move("h", player)
    if last_key == "j":
        move("j", player)
    if last_key == "k":
        move("k", player)
    if last_key == "l":
        move("l", player)
    else: move(last_key, player)
    """

def move(key, player):
    if key == "h":
        for i in range(len(player), 0, -1):
            if i == 1: player[i][1] = player[i][1]-1
            else: player[i] = player[i-1].copy()

    if key == "j":
        for i in range(len(player), 0, -1):
            if i == 1: player[i][0] = player[i][0]+1
            else: player[i] = player[i-1].copy()

    if key == "k":
        for i in range(len(player), 0, -1):
            if i == 1: player[i][0] = player[i][0]-1
            else: player[i] = player[i-1].copy()

    if key == "l":
        for i in range(len(player), 0, -1):
            if i == 1: player[i][1] = player[i][1]+1
            else: player[i] = player[i-1].copy()

def award():
    i = len(player)
    while ( i<len(player)):
        prize = [ random.randint(1, len(table)), random.randint(1, len(table)) ]
        if prize[1] == prize[i][1] and prize[0] == player[i][0]: i=i-1
    return prize

table = create_table(length=30)
player = create_player(table)
apple = award()

for i in range(20000):
    os.system("clear")
    print( f"Iteration {i}")
    print_table_with_player(table, player)
    detect_key()
    keyboard.on_press(get_last_key)
    time.sleep(0.05)
