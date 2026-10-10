import sys
sys.path.append('/sessions/vigilant-magical-gates/mnt/ruseme/tools')
from pixel import *

g = new()

# Body - ellipse, forward leaning
ellipse(g, 15.5, 21, 9, 5, 'B')
# Shadow at bottom
for x in range(8, 24):
    if g[25][x] == 'B': put(g, x, 25, 'D')
    if g[24][x] == 'B': put(g, x, 24, 'D')

# Left claw - holding magnifying glass
# Upper arm
for y in range(16, 20):
    for x in range(6, 10):
        if ((x-7.5)/2.5)**2 + ((y-17.5)/3)**2 <= 1:
            put(g, x, y, 'B')

# Claw palm - rounded
for y in range(10, 15):
    for x in range(5, 10):
        if ((x-7)/2.5)**2 + ((y-12)/2.5)**2 <= 1:
            put(g, x, y, 'B')
# Claw shadow
put(g, 6, 14, 'D')
put(g, 7, 14, 'D')

# Magnifying glass - handle
put(g, 6, 8, 'K')
put(g, 6, 9, 'K')
# Glass lens - circle
for y in range(3, 9):
    for x in range(4, 10):
        if ((x-6.5)/2.8)**2 + ((y-6)/2.8)**2 <= 1 and ((x-6.5)/2.8)**2 + ((y-6)/2.8)**2 > 0.5:
            put(g, x, y, 'G')

# Right claw - pointing forward
# Upper arm
for y in range(18, 22):
    for x in range(23, 27):
        if ((x-24.5)/2.5)**2 + ((y-20)/2.5)**2 <= 1:
            put(g, x, y, 'B')

# Claw - pointing shape
for x in range(25, 28):
    put(g, x, 18, 'B')
    put(g, x, 19, 'B')
put(g, 28, 17, 'B')
put(g, 28, 18, 'B')
put(g, 27, 19, 'D')

# Legs
both(g, 10, 24, 'B')
both(g, 10, 25, 'B')
both(g, 11, 25, 'B')
both(g, 12, 26, 'B')
both(g, 12, 27, 'B')

# Eyes - looking down at glass
both(g, 11, 18, 'C')
both(g, 12, 18, 'C')
both(g, 11, 19, 'C')
both(g, 12, 19, 'C')
# Pupils - down and inner
both(g, 12, 19, 'K')

# Mouth
put(g, 14, 22, 'C')
put(g, 15, 22, 'C')
put(g, 16, 22, 'C')
put(g, 17, 22, 'C')

# Blush
both(g, 9, 20, 'L')

# Motion lines (moving forward)
put(g, 3, 20, 'L')
put(g, 4, 20, 'L')
put(g, 3, 22, 'L')
put(g, 4, 22, 'L')

export(g, 'scanner', '/sessions/vigilant-magical-gates/mnt/ruseme/mascots/scanner')
print("Scanner Crab exported")
