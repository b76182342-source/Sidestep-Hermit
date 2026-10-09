import sys
sys.path.append('/sessions/vigilant-magical-gates/mnt/ruseme/tools')
from pixel import *

g = new()

# Body - centered
ellipse(g, 15.5, 20, 9.5, 5, 'B')
# Shadow at bottom
for x in range(7, 25):
    if g[24][x] == 'B': put(g, x, 24, 'D')
    if g[23][x] == 'B': put(g, x, 23, 'D')

# Left claw - shrugging (open palm up)
# Upper arm
for y in range(15, 19):
    for x in range(4, 8):
        if ((x-5.5)/2.5)**2 + ((y-17)/2.5)**2 <= 1:
            put(g, x, y, 'B')

# Palm - rounded, open upward
for y in range(11, 16):
    for x in range(3, 8):
        if ((x-5)/2.5)**2 + ((y-13)/2.5)**2 <= 1:
            put(g, x, y, 'B')
# Palm shadow
put(g, 4, 15, 'D')
put(g, 5, 15, 'D')

# Pincer tips - open V shape pointing up
put(g, 3, 10, 'B')
put(g, 4, 10, 'B')
put(g, 3, 11, 'B')
put(g, 7, 10, 'B')
put(g, 6, 10, 'B')
put(g, 7, 11, 'B')

# Right claw - mirror
# Upper arm
for y in range(15, 19):
    for x in range(24, 28):
        if ((x-25.5)/2.5)**2 + ((y-17)/2.5)**2 <= 1:
            put(g, x, y, 'B')

# Palm - rounded, open upward
for y in range(11, 16):
    for x in range(24, 29):
        if ((x-26)/2.5)**2 + ((y-13)/2.5)**2 <= 1:
            put(g, x, y, 'B')
# Palm shadow
put(g, 26, 15, 'D')
put(g, 27, 15, 'D')

# Pincer tips - open V shape
put(g, 28, 10, 'B')
put(g, 27, 10, 'B')
put(g, 28, 11, 'B')
put(g, 24, 10, 'B')
put(g, 25, 10, 'B')
put(g, 24, 11, 'B')

# Legs
both(g, 10, 23, 'B')
both(g, 10, 24, 'B')
both(g, 11, 24, 'B')
both(g, 12, 25, 'B')
both(g, 12, 26, 'B')

# Eyes - half closed, calm
both(g, 11, 17, 'C')
both(g, 12, 17, 'C')
both(g, 11, 18, 'C')
both(g, 12, 18, 'C')
# Pupils - centered, looking forward
both(g, 12, 18, 'K')

# Mouth - small, neutral
put(g, 14, 21, 'C')
put(g, 15, 21, 'C')
put(g, 16, 21, 'C')
put(g, 17, 21, 'C')

# Blush - light
both(g, 9, 19, 'L')

export(g, 'empty', '/sessions/vigilant-magical-gates/mnt/ruseme/mascots/empty')
print("Empty Crab exported")
