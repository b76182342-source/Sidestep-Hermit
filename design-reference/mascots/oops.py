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

# Left claw - raised up covering face (embarrassed)
# Upper arm going up
for y in range(14, 18):
    for x in range(5, 9):
        if ((x-6.5)/2.5)**2 + ((y-16)/2.5)**2 <= 1:
            put(g, x, y, 'B')

# Claw palm - rounded, in front of face
for y in range(10, 15):
    for x in range(5, 10):
        if ((x-7)/2.5)**2 + ((y-12)/2.5)**2 <= 1:
            put(g, x, y, 'B')
# Shadow on palm
put(g, 6, 14, 'D')
put(g, 7, 14, 'D')

# Pincer tips
put(g, 6, 9, 'B')
put(g, 7, 9, 'B')
put(g, 8, 10, 'B')
put(g, 9, 10, 'B')

# Right claw - hanging down
# Upper arm
for y in range(19, 23):
    for x in range(23, 27):
        if ((x-24.5)/2.5)**2 + ((y-21)/2.5)**2 <= 1:
            put(g, x, y, 'B')

# Claw palm drooping
for y in range(23, 27):
    for x in range(23, 27):
        if ((x-24.5)/2)**2 + ((y-25)/2)**2 <= 1:
            put(g, x, y, 'B')
put(g, 24, 26, 'D')
put(g, 25, 26, 'D')

# Legs
both(g, 10, 24, 'B')
both(g, 10, 25, 'B')
both(g, 11, 25, 'B')
both(g, 12, 26, 'B')
both(g, 12, 27, 'B')

# Eyes - wide, worried
both(g, 11, 18, 'C')
both(g, 12, 18, 'C')
both(g, 13, 18, 'C')
both(g, 11, 19, 'C')
both(g, 12, 19, 'C')
both(g, 13, 19, 'C')
# Pupils - large, centered
both(g, 12, 19, 'K')
both(g, 13, 19, 'K')

# Mouth - O shape (surprised/worried)
put(g, 14, 22, 'C')
put(g, 15, 22, 'C')
put(g, 16, 22, 'C')
put(g, 17, 22, 'C')
put(g, 14, 23, 'C')
put(g, 17, 23, 'C')

# Blush - strong
both(g, 9, 20, 'L')
both(g, 10, 20, 'L')

# Sweat drops - above head
put(g, 12, 14, 'L')
put(g, 12, 15, 'L')
put(g, 19, 14, 'L')
put(g, 19, 15, 'L')

export(g, 'oops', '/sessions/vigilant-magical-gates/mnt/ruseme/mascots/oops')
print("Oops Crab exported")
