import sys
sys.path.append('/sessions/vigilant-magical-gates/mnt/ruseme/tools')
from pixel import *

g = new()

# Body - centered, slightly lower (tired posture)
ellipse(g, 15.5, 22, 9.5, 5, 'B')
# Shadow at bottom
for x in range(7, 25):
    if g[26][x] == 'B': put(g, x, 26, 'D')
    if g[25][x] == 'B': put(g, x, 25, 'D')

# Left claw - drooping down
# Upper arm
for y in range(20, 24):
    for x in range(5, 9):
        if ((x-6.5)/2.5)**2 + ((y-22)/2.5)**2 <= 1:
            put(g, x, y, 'B')

# Palm hanging
for y in range(24, 28):
    for x in range(4, 8):
        if ((x-5.5)/2)**2 + ((y-26)/2)**2 <= 1:
            put(g, x, y, 'B')
put(g, 5, 27, 'D')

# Right claw - drooping down
# Upper arm
for y in range(20, 24):
    for x in range(23, 27):
        if ((x-24.5)/2.5)**2 + ((y-22)/2.5)**2 <= 1:
            put(g, x, y, 'B')

# Palm hanging
for y in range(24, 28):
    for x in range(24, 28):
        if ((x-25.5)/2)**2 + ((y-26)/2)**2 <= 1:
            put(g, x, y, 'B')
put(g, 26, 27, 'D')

# Legs
both(g, 10, 25, 'B')
both(g, 10, 26, 'B')
both(g, 11, 26, 'B')
both(g, 12, 27, 'B')
both(g, 12, 28, 'B')

# Night cap - over head
# Cap body
put(g, 13, 14, 'K')
put(g, 14, 14, 'K')
put(g, 15, 14, 'K')
put(g, 16, 14, 'K')
put(g, 17, 14, 'K')
put(g, 18, 14, 'K')
put(g, 12, 15, 'K')
put(g, 13, 15, 'K')
put(g, 14, 15, 'K')
put(g, 15, 15, 'K')
put(g, 16, 15, 'K')
put(g, 17, 15, 'K')
put(g, 18, 15, 'K')
put(g, 19, 15, 'K')
put(g, 12, 16, 'K')
put(g, 19, 16, 'K')
put(g, 11, 17, 'K')
put(g, 20, 17, 'K')

# Cap tip - drooping to the side
put(g, 19, 14, 'K')
put(g, 20, 14, 'K')
put(g, 21, 15, 'K')
put(g, 22, 15, 'K')
put(g, 23, 16, 'K')
put(g, 24, 16, 'K')
# Pom-pom
put(g, 25, 16, 'C')
put(g, 26, 16, 'C')
put(g, 25, 17, 'C')
put(g, 26, 17, 'C')

# Eyes - closed (sleeping)
both(g, 11, 19, 'C')
both(g, 12, 19, 'C')
both(g, 13, 19, 'C')
# Eyelids - horizontal lines
both(g, 11, 19, 'K')
both(g, 12, 19, 'K')
both(g, 13, 19, 'K')

# Mouth - slightly open (snoring)
put(g, 14, 22, 'C')
put(g, 15, 22, 'C')
put(g, 16, 22, 'C')
put(g, 17, 22, 'C')
put(g, 15, 23, 'C')
put(g, 16, 23, 'C')

# ZZZ symbols floating above
put(g, 24, 8, 'L')
put(g, 25, 8, 'L')
put(g, 26, 8, 'L')
put(g, 24, 9, 'L')
put(g, 26, 9, 'L')
put(g, 24, 10, 'L')
put(g, 25, 10, 'L')
put(g, 26, 10, 'L')

put(g, 21, 11, 'L')
put(g, 22, 11, 'L')
put(g, 21, 12, 'L')
put(g, 22, 12, 'L')

export(g, 'sleepy', '/sessions/vigilant-magical-gates/mnt/ruseme/mascots/sleepy')
print("Sleepy Crab exported")
