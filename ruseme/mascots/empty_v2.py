import sys
sys.path.append('/sessions/vigilant-magical-gates/mnt/ruseme/tools')
from pixel import *

g = new()

# Empty Crab - Baby crab, round and chubby
# Body - very round, centered
ellipse(g, 15.5, 19, 8, 6, 'B')
# Shadow
for x in range(8, 24):
    if g[24][x] == 'B': put(g, x, 24, 'D')
    if g[23][x] == 'B': put(g, x, 23, 'D')

# Left claw - shrugging, open palm up
# Arm - short and chubby
for y in range(16, 19):
    for x in range(5, 8):
        if ((x-6)/2)**2 + ((y-17.5)/2)**2 <= 1:
            put(g, x, y, 'B')

# Palm - rounded, facing up
for y in range(12, 17):
    for x in range(3, 8):
        if ((x-5)/2.5)**2 + ((y-14)/2.5)**2 <= 1:
            put(g, x, y, 'B')
put(g, 4, 16, 'D')
put(g, 5, 16, 'D')

# Pincers - small V pointing up
put(g, 3, 11, 'B')
put(g, 4, 11, 'B')
put(g, 3, 12, 'B')

put(g, 6, 11, 'B')
put(g, 7, 11, 'B')
put(g, 7, 12, 'B')

# Right claw - mirror
for y in range(16, 19):
    for x in range(24, 27):
        if ((x-25)/2)**2 + ((y-17.5)/2)**2 <= 1:
            put(g, x, y, 'B')

for y in range(12, 17):
    for x in range(24, 29):
        if ((x-26)/2.5)**2 + ((y-14)/2.5)**2 <= 1:
            put(g, x, y, 'B')
put(g, 26, 16, 'D')
put(g, 27, 16, 'D')

put(g, 28, 11, 'B')
put(g, 27, 11, 'B')
put(g, 28, 12, 'B')

put(g, 25, 11, 'B')
put(g, 24, 11, 'B')
put(g, 24, 12, 'B')

# Chubby legs - 2-3px thick
both(g, 9, 23, 'B')
both(g, 9, 24, 'B')
both(g, 10, 24, 'B')
both(g, 11, 25, 'B')
both(g, 11, 26, 'B')
both(g, 12, 26, 'B')
both(g, 13, 26, 'B')

# Big baby eyes
both(g, 11, 17, 'C')
both(g, 12, 17, 'C')
both(g, 13, 17, 'C')
both(g, 11, 18, 'C')
both(g, 12, 18, 'C')
both(g, 13, 18, 'C')
both(g, 11, 19, 'C')
both(g, 12, 19, 'C')
both(g, 13, 19, 'C')
# Pupils - large innocent
both(g, 12, 18, 'K')
both(g, 13, 18, 'K')
both(g, 12, 19, 'K')
both(g, 13, 19, 'K')

# Small mouth
put(g, 14, 21, 'C')
put(g, 15, 21, 'C')
put(g, 16, 21, 'C')
put(g, 17, 21, 'C')

# Blush
both(g, 9, 19, 'L')
both(g, 10, 19, 'L')

# Empty shell nearby - to the right
# Shell spiral shape
put(g, 25, 25, 'D')
put(g, 26, 25, 'D')
put(g, 27, 25, 'D')
put(g, 25, 26, 'D')
put(g, 26, 26, 'D')
put(g, 27, 26, 'D')
put(g, 28, 26, 'D')
put(g, 26, 27, 'D')
put(g, 27, 27, 'D')
put(g, 28, 27, 'D')
put(g, 27, 28, 'D')
put(g, 28, 28, 'D')
# Shell opening
put(g, 26, 26, 'K')
put(g, 27, 27, 'K')

export(g, 'empty-v2', '/sessions/vigilant-magical-gates/mnt/ruseme/mascots/empty-v2')
print("Empty v2 exported")
