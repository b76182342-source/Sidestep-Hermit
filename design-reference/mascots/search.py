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

# Left claw - holding binoculars/telescope
# Upper arm
for y in range(15, 19):
    for x in range(5, 9):
        if ((x-6.5)/2.5)**2 + ((y-17)/2.5)**2 <= 1:
            put(g, x, y, 'B')

# Claw palm - holding device
for y in range(11, 16):
    for x in range(4, 9):
        if ((x-6)/2.5)**2 + ((y-13)/2.5)**2 <= 1:
            put(g, x, y, 'B')
put(g, 5, 15, 'D')
put(g, 6, 15, 'D')

# Binoculars/telescope
put(g, 5, 8, 'K')
put(g, 6, 8, 'K')
put(g, 7, 8, 'K')
put(g, 8, 8, 'K')
put(g, 5, 9, 'K')
put(g, 6, 9, 'K')
put(g, 7, 9, 'K')
put(g, 8, 9, 'K')
put(g, 5, 10, 'K')
put(g, 8, 10, 'K')
# Lens highlights
put(g, 6, 9, 'H')
put(g, 7, 9, 'H')

# Right claw - pointing/gesturing
# Upper arm
for y in range(17, 21):
    for x in range(23, 27):
        if ((x-24.5)/2.5)**2 + ((y-19)/2.5)**2 <= 1:
            put(g, x, y, 'B')

# Claw pointing upward
for y in range(13, 18):
    for x in range(23, 27):
        if ((x-24.5)/2)**2 + ((y-15)/2.5)**2 <= 1:
            put(g, x, y, 'B')
put(g, 24, 17, 'D')

# Pincer pointing up
put(g, 23, 12, 'B')
put(g, 24, 12, 'B')
put(g, 24, 11, 'B')
put(g, 25, 12, 'B')
put(g, 26, 12, 'B')
put(g, 26, 11, 'B')

# Legs
both(g, 10, 24, 'B')
both(g, 10, 25, 'B')
both(g, 11, 25, 'B')
both(g, 12, 26, 'B')
both(g, 12, 27, 'B')

# Detective/search hat - simple cap with small brim
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
put(g, 12, 14, 'K')
put(g, 13, 14, 'K')
put(g, 14, 14, 'K')
put(g, 15, 14, 'K')
put(g, 16, 14, 'K')
put(g, 17, 14, 'K')
put(g, 18, 14, 'K')
put(g, 19, 14, 'K')
# Hat highlight
put(g, 15, 14, 'H')
put(g, 16, 14, 'H')

# Eyes - focused, squinting through binoculars
both(g, 11, 18, 'C')
both(g, 12, 18, 'C')
both(g, 11, 19, 'C')
both(g, 12, 19, 'C')
# Pupils - small, focused
both(g, 12, 19, 'K')

# Mouth - concentrated
put(g, 14, 22, 'C')
put(g, 15, 22, 'C')
put(g, 16, 22, 'C')
put(g, 17, 22, 'C')

# Search radar circles/waves
put(g, 27, 6, 'L')
put(g, 28, 6, 'L')
put(g, 26, 7, 'L')
put(g, 29, 7, 'L')
put(g, 26, 8, 'L')
put(g, 29, 8, 'L')

export(g, 'search', '/sessions/vigilant-magical-gates/mnt/ruseme/mascots/search')
print("Search Crab exported")
