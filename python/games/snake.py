import os
import time
import random
import keyboard

def create_table(length):
    return [ ["." if j==0 or j==length-1 or i==0 or i==length-1 else " " for j in range(length)] for i in range(length)]

def create_player(table):
    player = {
        1: [int(len(table)/2), 6 ],
        2: [int(len(table)/2), 5 ],
        3: [int(len(table)/2), 4 ],
        4: [int(len(table)/2), 3 ]
    }
    return player

def print_table_with_player(table, player, prize):
    for i in range(len(table)):
        for j in range(len(table)):
            value=True
            if prize[0]== i and prize[1]== j :
                print("#", end="")
                value=False
            for key in player.keys():
                if player[key] == [i,j]:
                    value=False
                    print("■", end=" ")
            if value: print( table[i][j], end=" ")
        print("")

def get_last_key(event):
    global last_key
    if last_key == "h" and event.name == "l":
        last_key = "h"
    if last_key == "l" and event.name == "h":
        last_key = "l"
    if last_key == "j" and event.name == "k":
        last_key = "j"
    if last_key == "k" and event.name == "j":
        last_key = "k"
    else:
        last_key = event.name

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

def eat(prize, player, puntaje):
    if player[1] == prize:
        player[len(player)+1] = player[len(player)].copy()
        prize = award(player)
        puntaje=puntaje+1
    return player, prize, puntaje

def award(player):
    i = 1
    while ( i < len(player)):
        prize = [ random.randint(1, len(table)-2), random.randint(1, len(table)-2) ]
        if prize[1] == player[i][1] and prize[0] == player[i][0]: i=i-1
        i = i+1
    return prize

def check_dead(player, table):
    l = len(table)-1
    if player[1][0] == l or player[1][1] == l or player[1][0] == 0 or player[1][1] == 0:
        print("===============================")
        print("========   GAME OVER   ========")
        print("===============================")
        return True
    return False

last_key="l"
table = create_table(length=30)
player = create_player(table)
apple = award(player)
puntaje = 0

for i in range(20000):
    os.system("clear")
    print("===============================")
    print( f"======    SNAKE GAME    =======")
    print("===============================")
    print( f"======    PUNTAJE {puntaje}    =======")
    print("===============================")
    print_table_with_player(table, player, apple)
    move(last_key, player)
    player, apple, puntaje = eat(apple, player, puntaje)
    if check_dead(player, table): break
    keyboard.on_press(get_last_key)
    time.sleep(0.1)
