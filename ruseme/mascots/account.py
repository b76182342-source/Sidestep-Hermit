import sys
sys.path.append('/sessions/vigilant-magical-gates/mnt/ruseme/tools')
from pixel import *

g = new()

# Body - centered, upright guard posture
ellipse(g, 15.5, 21, 9.5, 5, 'B')
# Shadow at bottom
for x in range(7, 25):
    if g[25][x] == 'B': put(g, x, 25, 'D')
    if g[24][x] == 'B': put(g, x, 24, 'D')

# Left claw - holding key
# Upper arm
for y in range(16, 20):
    for x in range(5, 9):
        if ((x-6.5)/2.5)**2 + ((y-18)/2.5)**2 <= 1:
            put(g, x, y, 'B')

# Claw palm - holding key
for y in range(12, 17):
    for x in range(4, 9):
        if ((x-6)/2.5)**2 + ((y-14)/2.5)**2 <= 1:
            put(g, x, y, 'B')
put(g, 5, 16, 'D')
put(g, 6, 16, 'D')

# Key - simple design
# Key handle/bow (circle)
put(g, 5, 7, 'K')
put(g, 6, 7, 'K')
put(g, 7, 7, 'K')
put(g, 5, 8, 'K')
put(g, 7, 8, 'K')
put(g, 5, 9, 'K')
put(g, 6, 9, 'K')
put(g, 7, 9, 'K')
# Key shaft
put(g, 6, 10, 'K')
put(g, 6, 11, 'K')
# Key teeth
put(g, 5, 11, 'K')
put(g, 7, 10, 'K')

# Right claw - crossed over chest (guard stance)
# Upper arm
for y in range(18, 22):
    for x in range(23, 27):
        if ((x-24.5)/2.5)**2 + ((y-20)/2.5)**2 <= 1:
            put(g, x, y, 'B')

# Claw crossing chest - angled
for y in range(17, 21):
    for x in range(18, 23):
        if ((x-20)/2.5)**2 + ((y-19)/2)**2 <= 1:
            put(g, x, y, 'B')
put(g, 19, 20, 'D')
put(g, 20, 20, 'D')

# Pincer
put(g, 17, 16, 'B')
put(g, 18, 16, 'B')
put(g, 17, 17, 'B')
put(g, 19, 17, 'B')
put(g, 20, 17, 'B')

# Legs - firm stance
both(g, 10, 24, 'B')
both(g, 10, 25, 'B')
both(g, 11, 25, 'B')
both(g, 12, 26, 'B')
both(g, 12, 27, 'B')

# Eyes - alert, serious
both(g, 11, 18, 'C')
both(g, 12, 18, 'C')
both(g, 11, 19, 'C')
both(g, 12, 19, 'C')
# Pupils - centered, steady gaze
both(g, 12, 19, 'K')

# Eyebrows - slight serious expression
both(g, 11, 17, 'K')
both(g, 12, 17, 'K')

# Mouth - straight line (serious but not angry)
put(g, 14, 22, 'C')
put(g, 15, 22, 'C')
put(g, 16, 22, 'C')
put(g, 17, 22, 'C')

# Shield symbol on body (optional security badge)
put(g, 14, 20, 'H')
put(g, 15, 19, 'H')
put(g, 16, 19, 'H')
put(g, 17, 20, 'H')
put(g, 15, 20, 'H')
put(g, 16, 20, 'H')

export(g, 'account', '/sessions/vigilant-magical-gates/mnt/ruseme/mascots/account')
print("Account Crab exported")
