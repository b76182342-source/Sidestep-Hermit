import sys
sys.path.append('/sessions/vigilant-magical-gates/mnt/ruseme/tools')
from pixel import *

g = new()

# Body - centered
ellipse(g, 15.5, 21, 9.5, 5, 'B')
# Shadow at bottom
for x in range(7, 25):
    if g[25][x] == 'B': put(g, x, 25, 'D')
    if g[24][x] == 'B': put(g, x, 24, 'D')

# Left claw - holding microphone
# Upper arm
for y in range(17, 21):
    for x in range(5, 9):
        if ((x-6.5)/2.5)**2 + ((y-19)/2.5)**2 <= 1:
            put(g, x, y, 'B')

# Claw palm - holding mic
for y in range(13, 18):
    for x in range(4, 9):
        if ((x-6)/2.5)**2 + ((y-15)/2.5)**2 <= 1:
            put(g, x, y, 'B')
put(g, 5, 17, 'D')
put(g, 6, 17, 'D')

# Microphone - vertical
put(g, 6, 8, 'K')
put(g, 6, 9, 'K')
put(g, 6, 10, 'K')
put(g, 6, 11, 'K')
put(g, 6, 12, 'K')
# Mic head - rounded
put(g, 5, 7, 'K')
put(g, 6, 6, 'K')
put(g, 7, 7, 'K')
put(g, 5, 8, 'K')
put(g, 7, 8, 'K')
# Mic highlight
put(g, 7, 7, 'H')

# Right claw - relaxed down
# Upper arm
for y in range(19, 23):
    for x in range(23, 27):
        if ((x-24.5)/2.5)**2 + ((y-21)/2.5)**2 <= 1:
            put(g, x, y, 'B')

# Palm
for y in range(23, 27):
    for x in range(23, 27):
        if ((x-24.5)/2)**2 + ((y-25)/2)**2 <= 1:
            put(g, x, y, 'B')
put(g, 24, 26, 'D')

# Legs
both(g, 10, 24, 'B')
both(g, 10, 25, 'B')
both(g, 11, 25, 'B')
both(g, 12, 26, 'B')
both(g, 12, 27, 'B')

# Eyes - closed, singing blissfully
both(g, 11, 18, 'C')
both(g, 12, 18, 'C')
both(g, 13, 18, 'C')
# Eyelids/lashes - curved lines
both(g, 11, 18, 'K')
both(g, 12, 18, 'K')
both(g, 13, 18, 'K')

# Mouth - open singing
put(g, 14, 21, 'C')
put(g, 15, 21, 'C')
put(g, 16, 21, 'C')
put(g, 17, 21, 'C')
put(g, 14, 22, 'C')
put(g, 15, 22, 'C')
put(g, 16, 22, 'C')
put(g, 17, 22, 'C')
put(g, 15, 22, 'K')
put(g, 16, 22, 'K')

# Blush - prominent
both(g, 9, 19, 'L')
both(g, 10, 19, 'L')

# Music notes floating
put(g, 24, 10, 'L')
put(g, 25, 10, 'L')
put(g, 25, 11, 'L')
put(g, 25, 12, 'L')
put(g, 27, 8, 'L')
put(g, 28, 8, 'L')
put(g, 28, 9, 'L')

export(g, 'lyric', '/sessions/vigilant-magical-gates/mnt/ruseme/mascots/lyric')
print("Lyric Crab exported")
