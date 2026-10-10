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

# Left claw - raised high (dancing)
# Upper arm going up
for y in range(12, 16):
    for x in range(5, 9):
        if ((x-6.5)/2.5)**2 + ((y-14)/2.5)**2 <= 1:
            put(g, x, y, 'B')

# Claw palm - upward
for y in range(8, 13):
    for x in range(4, 9):
        if ((x-6)/2.5)**2 + ((y-10)/2.5)**2 <= 1:
            put(g, x, y, 'B')
put(g, 5, 12, 'D')
put(g, 6, 12, 'D')

# Pincer tips - V shape
put(g, 4, 7, 'B')
put(g, 5, 7, 'B')
put(g, 4, 8, 'B')
put(g, 7, 7, 'B')
put(g, 8, 7, 'B')
put(g, 8, 8, 'B')

# Right claw - raised high (dancing)
# Upper arm going up
for y in range(12, 16):
    for x in range(23, 27):
        if ((x-24.5)/2.5)**2 + ((y-14)/2.5)**2 <= 1:
            put(g, x, y, 'B')

# Claw palm - upward
for y in range(8, 13):
    for x in range(23, 28):
        if ((x-25)/2.5)**2 + ((y-10)/2.5)**2 <= 1:
            put(g, x, y, 'B')
put(g, 25, 12, 'D')
put(g, 26, 12, 'D')

# Pincer tips - V shape
put(g, 27, 7, 'B')
put(g, 26, 7, 'B')
put(g, 27, 8, 'B')
put(g, 24, 7, 'B')
put(g, 23, 7, 'B')
put(g, 23, 8, 'B')

# Legs
both(g, 10, 24, 'B')
both(g, 10, 25, 'B')
both(g, 11, 25, 'B')
both(g, 12, 26, 'B')
both(g, 12, 27, 'B')

# Headphones - over eyes area
# Headband
put(g, 11, 15, 'K')
put(g, 12, 15, 'K')
put(g, 13, 15, 'K')
put(g, 14, 15, 'K')
put(g, 15, 15, 'K')
put(g, 16, 15, 'K')
put(g, 17, 15, 'K')
put(g, 18, 15, 'K')
put(g, 19, 15, 'K')
put(g, 20, 15, 'K')
put(g, 11, 16, 'K')
put(g, 20, 16, 'K')

# Left ear cup
put(g, 9, 17, 'K')
put(g, 10, 17, 'K')
put(g, 11, 17, 'K')
put(g, 9, 18, 'K')
put(g, 10, 18, 'K')
put(g, 11, 18, 'K')
put(g, 9, 19, 'K')
put(g, 10, 19, 'K')
put(g, 11, 19, 'K')
put(g, 10, 18, 'H')

# Right ear cup
put(g, 20, 17, 'K')
put(g, 21, 17, 'K')
put(g, 22, 17, 'K')
put(g, 20, 18, 'K')
put(g, 21, 18, 'K')
put(g, 22, 18, 'K')
put(g, 20, 19, 'K')
put(g, 21, 19, 'K')
put(g, 22, 19, 'K')
put(g, 21, 18, 'H')

# Eyes - excited, visible through headphones
both(g, 12, 18, 'C')
both(g, 13, 18, 'C')
both(g, 12, 19, 'C')
both(g, 13, 19, 'C')
# Pupils - large excited
both(g, 13, 19, 'K')

# Mouth - big smile
put(g, 14, 22, 'C')
put(g, 15, 22, 'C')
put(g, 16, 22, 'C')
put(g, 17, 22, 'C')

# Sound waves / music notes
put(g, 2, 15, 'L')
put(g, 3, 15, 'L')
put(g, 2, 17, 'L')
put(g, 28, 15, 'L')
put(g, 29, 15, 'L')
put(g, 29, 17, 'L')

export(g, 'dj', '/sessions/vigilant-magical-gates/mnt/ruseme/mascots/dj')
print("DJ Crab exported")
