import sys
sys.path.append('/sessions/vigilant-magical-gates/mnt/ruseme/tools')
from pixel import *
from check import check_all_mascots

g = new()

# Scanner Crab - Fiddler crab with one huge claw, one tiny claw
# Body - offset to left to make room for giant claw
ellipse(g, 13, 20, 7, 4.5, 'B')
# Shadow
for x in range(7, 20):
    if g[23][x] == 'B': put(g, x, 23, 'D')
    if g[24][x] == 'B': put(g, x, 24, 'D')

# GIANT left claw - signature feature
# Arm connecting to body
for y in range(17, 21):
    for x in range(5, 9):
        if ((x-6.5)/2.5)**2 + ((y-19)/2)**2 <= 1:
            put(g, x, y, 'B')

# Massive claw palm
for y in range(10, 18):
    for x in range(2, 11):
        if ((x-6)/4.5)**2 + ((y-14)/4)**2 <= 1:
            put(g, x, y, 'B')
# Shadow on giant claw
for x in range(2, 11):
    if g[16][x] == 'B': put(g, x, 16, 'D')
    if g[17][x] == 'B': put(g, x, 17, 'D')

# Giant pincer tips - wide V
put(g, 2, 9, 'B')
put(g, 3, 9, 'B')
put(g, 4, 9, 'B')
put(g, 2, 10, 'B')
put(g, 3, 10, 'B')

put(g, 8, 9, 'B')
put(g, 9, 9, 'B')
put(g, 10, 9, 'B')
put(g, 9, 10, 'B')
put(g, 10, 10, 'B')

# Tiny right claw
# Small arm
for y in range(19, 21):
    for x in range(19, 21):
        put(g, x, y, 'B')

# Tiny palm
put(g, 20, 18, 'B')
put(g, 21, 18, 'B')
put(g, 20, 19, 'B')
put(g, 21, 19, 'B')

# Tiny pincers
put(g, 21, 17, 'B')
put(g, 22, 17, 'B')

# Legs - 2px thick
both(g, 9, 23, 'B')
both(g, 9, 24, 'B')
both(g, 10, 24, 'B')
both(g, 11, 25, 'B')
both(g, 11, 26, 'B')
both(g, 12, 26, 'B')

# Eyes on stalks - asymmetric
# Left eye
put(g, 10, 16, 'B')
put(g, 10, 17, 'B')
put(g, 9, 15, 'C')
put(g, 10, 15, 'C')
put(g, 9, 16, 'C')
put(g, 10, 16, 'C')
put(g, 10, 16, 'K')

# Right eye
put(g, 16, 16, 'B')
put(g, 16, 17, 'B')
put(g, 16, 15, 'C')
put(g, 17, 15, 'C')
put(g, 16, 16, 'C')
put(g, 17, 16, 'C')
put(g, 17, 16, 'K')

# Mouth
put(g, 12, 21, 'C')
put(g, 13, 21, 'C')
put(g, 14, 21, 'C')

# Blush
put(g, 8, 19, 'L')
put(g, 15, 19, 'L')

# Scanning arc - fan shape behind
put(g, 22, 12, 'L')
put(g, 23, 12, 'L')
put(g, 23, 13, 'L')
put(g, 24, 13, 'L')
put(g, 24, 14, 'L')
put(g, 25, 14, 'L')
put(g, 25, 15, 'L')
put(g, 26, 16, 'L')
put(g, 26, 17, 'L')
put(g, 27, 18, 'L')

export(g, 'scanner-v2', '/sessions/vigilant-magical-gates/mnt/ruseme/mascots/scanner-v2')
print("Scanner v2 exported")

# Store for checking
scanner_grid = [row[:] for row in g]
