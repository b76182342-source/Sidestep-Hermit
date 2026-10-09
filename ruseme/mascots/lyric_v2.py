import sys
sys.path.append('/sessions/vigilant-magical-gates/mnt/ruseme/tools')
from pixel import *

g = new()

# Lyric Crab - Tall singer standing on tiptoes with long eye stalks
# Body - narrow and tall, higher up
ellipse(g, 15.5, 22, 6, 4, 'B')
# Shadow at very bottom
for x in range(10, 22):
    if g[25][x] == 'B': put(g, x, 25, 'D')

# Legs standing on tiptoes - 2px thick, straight down
both(g, 11, 24, 'B')
both(g, 11, 25, 'B')
both(g, 11, 26, 'B')
both(g, 12, 26, 'B')
both(g, 12, 27, 'B')
both(g, 13, 27, 'B')

# Left claw - holding microphone up
# Arm going up
for y in range(18, 22):
    for x in range(6, 9):
        if ((x-7)/2)**2 + ((y-20)/2)**2 <= 1:
            put(g, x, y, 'B')

# Claw palm holding mic
for y in range(14, 19):
    for x in range(5, 9):
        if ((x-6.5)/2)**2 + ((y-16)/2.5)**2 <= 1:
            put(g, x, y, 'B')
put(g, 6, 18, 'D')
put(g, 7, 18, 'D')

# Microphone - vertical, 2px thick handle
put(g, 6, 10, 'K')
put(g, 7, 10, 'K')
put(g, 6, 11, 'K')
put(g, 7, 11, 'K')
put(g, 6, 12, 'K')
put(g, 7, 12, 'K')
put(g, 6, 13, 'K')
put(g, 7, 13, 'K')

# Mic head - rounded, 2px thick
put(g, 5, 8, 'K')
put(g, 6, 8, 'K')
put(g, 7, 8, 'K')
put(g, 8, 8, 'K')
put(g, 5, 9, 'K')
put(g, 6, 9, 'K')
put(g, 7, 9, 'K')
put(g, 8, 9, 'K')
put(g, 6, 9, 'H')
put(g, 7, 9, 'H')

# Right claw - relaxed at side
for y in range(21, 24):
    for x in range(22, 25):
        if ((x-23)/2)**2 + ((y-22.5)/2)**2 <= 1:
            put(g, x, y, 'B')

# Small palm
put(g, 23, 24, 'B')
put(g, 24, 24, 'B')
put(g, 23, 25, 'B')
put(g, 24, 25, 'B')
put(g, 24, 25, 'D')

# Long eye stalks - signature feature
# Left stalk
put(g, 12, 17, 'B')
put(g, 12, 18, 'B')
put(g, 12, 19, 'B')
put(g, 12, 20, 'B')
# Left eyeball
put(g, 11, 16, 'C')
put(g, 12, 16, 'C')
put(g, 13, 16, 'C')
put(g, 11, 17, 'C')
put(g, 12, 17, 'C')
put(g, 13, 17, 'C')
put(g, 12, 17, 'K')

# Right stalk
put(g, 19, 17, 'B')
put(g, 19, 18, 'B')
put(g, 19, 19, 'B')
put(g, 19, 20, 'B')
# Right eyeball
put(g, 18, 16, 'C')
put(g, 19, 16, 'C')
put(g, 20, 16, 'C')
put(g, 18, 17, 'C')
put(g, 19, 17, 'C')
put(g, 20, 17, 'C')
put(g, 19, 17, 'K')

# Mouth - wide open singing, 2px tall
put(g, 13, 23, 'C')
put(g, 14, 23, 'C')
put(g, 15, 23, 'C')
put(g, 16, 23, 'C')
put(g, 17, 23, 'C')
put(g, 18, 23, 'C')
put(g, 14, 24, 'C')
put(g, 15, 24, 'C')
put(g, 16, 24, 'C')
put(g, 17, 24, 'C')
# Tongue
put(g, 15, 24, 'L')
put(g, 16, 24, 'L')

# Blush
put(g, 10, 21, 'L')
put(g, 21, 21, 'L')

# Music notes - standard pattern
# Note 1
put(g, 24, 8, 'L')
put(g, 25, 8, 'L')
put(g, 24, 9, 'L')
put(g, 25, 9, 'L')
put(g, 25, 10, 'L')
put(g, 25, 11, 'L')
put(g, 25, 12, 'L')
put(g, 24, 13, 'L')
put(g, 25, 13, 'L')
put(g, 26, 13, 'L')
put(g, 24, 14, 'L')
put(g, 25, 14, 'L')

# Note 2
put(g, 27, 11, 'L')
put(g, 28, 11, 'L')
put(g, 27, 12, 'L')
put(g, 28, 12, 'L')
put(g, 28, 13, 'L')
put(g, 28, 14, 'L')
put(g, 28, 15, 'L')
put(g, 27, 16, 'L')
put(g, 28, 16, 'L')
put(g, 29, 16, 'L')
put(g, 27, 17, 'L')
put(g, 28, 17, 'L')

export(g, 'lyric-v2', '/sessions/vigilant-magical-gates/mnt/ruseme/mascots/lyric-v2')
print("Lyric v2 exported")
