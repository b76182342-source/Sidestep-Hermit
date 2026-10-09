import sys
sys.path.append('/sessions/vigilant-magical-gates/mnt/ruseme/tools')
from pixel import *

g = new()

# Sleepy Crab - Lying on back, legs up, purple sleep cap
# Body - lying flat, wider
ellipse(g, 15.5, 20, 9, 5, 'B')
# Shadow under body
for x in range(7, 25):
    if g[24][x] == 'B': put(g, x, 24, 'D')
    if g[23][x] == 'B': put(g, x, 23, 'D')

# Belly (lighter)
for x in range(10, 22):
    if g[19][x] == 'B': put(g, x, 19, 'L')
    if g[20][x] == 'B': put(g, x, 20, 'L')

# Legs up in air - 2px thick
# Left front leg
put(g, 9, 21, 'B')
put(g, 10, 21, 'B')
put(g, 8, 20, 'B')
put(g, 9, 20, 'B')
put(g, 7, 19, 'B')
put(g, 8, 19, 'B')
put(g, 6, 18, 'B')
put(g, 7, 18, 'B')

# Left back leg
put(g, 11, 22, 'B')
put(g, 12, 22, 'B')
put(g, 10, 21, 'B')
put(g, 11, 21, 'B')
put(g, 9, 20, 'B')
put(g, 10, 20, 'B')

# Right front leg
put(g, 21, 21, 'B')
put(g, 22, 21, 'B')
put(g, 22, 20, 'B')
put(g, 23, 20, 'B')
put(g, 23, 19, 'B')
put(g, 24, 19, 'B')
put(g, 24, 18, 'B')
put(g, 25, 18, 'B')

# Right back leg
put(g, 19, 22, 'B')
put(g, 20, 22, 'B')
put(g, 20, 21, 'B')
put(g, 21, 21, 'B')
put(g, 21, 20, 'B')
put(g, 22, 20, 'B')

# Claws resting on belly
# Left claw
for y in range(18, 21):
    for x in range(11, 14):
        if ((x-12)/2)**2 + ((y-19)/2)**2 <= 1:
            put(g, x, y, 'B')
put(g, 12, 20, 'D')

# Right claw
for y in range(18, 21):
    for x in range(18, 21):
        if ((x-19)/2)**2 + ((y-19)/2)**2 <= 1:
            put(g, x, y, 'B')
put(g, 19, 20, 'D')

# Eyes - closed peacefully
both(g, 13, 17, 'C')
both(g, 14, 17, 'C')
both(g, 15, 17, 'C')
# Closed eyelids - horizontal lines
both(g, 13, 17, 'K')
both(g, 14, 17, 'K')
both(g, 15, 17, 'K')

# Small peaceful mouth
put(g, 15, 19, 'C')
put(g, 16, 19, 'C')
put(g, 17, 19, 'C')

# Purple sleep cap - on head area
put(g, 12, 14, 'P')
put(g, 13, 14, 'P')
put(g, 14, 14, 'P')
put(g, 15, 14, 'P')
put(g, 16, 14, 'P')
put(g, 17, 14, 'P')
put(g, 18, 14, 'P')
put(g, 19, 14, 'P')
put(g, 11, 15, 'P')
put(g, 12, 15, 'P')
put(g, 13, 15, 'P')
put(g, 14, 15, 'P')
put(g, 15, 15, 'P')
put(g, 16, 15, 'P')
put(g, 17, 15, 'P')
put(g, 18, 15, 'P')
put(g, 19, 15, 'P')
put(g, 20, 15, 'P')
put(g, 12, 16, 'P')
put(g, 19, 16, 'P')

# Cap drooping to side
put(g, 20, 14, 'P')
put(g, 21, 14, 'P')
put(g, 21, 15, 'P')
put(g, 22, 15, 'P')
put(g, 23, 15, 'P')
put(g, 24, 16, 'P')
put(g, 25, 16, 'P')
# Pom-pom
put(g, 26, 16, 'C')
put(g, 27, 16, 'C')
put(g, 26, 17, 'C')
put(g, 27, 17, 'C')

# Z symbol - standard pattern (5x5)
# XXXXX
# ...X.
# ..X..
# .X...
# XXXXX
put(g, 5, 8, 'L')
put(g, 6, 8, 'L')
put(g, 7, 8, 'L')
put(g, 8, 8, 'L')
put(g, 9, 8, 'L')
put(g, 8, 9, 'L')
put(g, 7, 10, 'L')
put(g, 6, 11, 'L')
put(g, 5, 12, 'L')
put(g, 6, 12, 'L')
put(g, 7, 12, 'L')
put(g, 8, 12, 'L')
put(g, 9, 12, 'L')

export(g, 'sleepy-v2', '/sessions/vigilant-magical-gates/mnt/ruseme/mascots/sleepy-v2')
print("Sleepy v2 exported")
