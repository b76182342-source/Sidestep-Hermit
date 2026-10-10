import sys
sys.path.append('/sessions/vigilant-magical-gates/mnt/ruseme/tools')
from pixel import *

g = new()

# Account Crab - Hermit crab with vinyl record shell, ID badge, key
# Body - small, lower position
ellipse(g, 15.5, 24, 6, 3.5, 'B')
# Shadow
for x in range(10, 22):
    if g[27][x] == 'B': put(g, x, 27, 'D')

# Vinyl record shell on back - large and prominent
# Outer ring - dark
put(g, 11, 14, 'K')
put(g, 12, 14, 'K')
put(g, 13, 14, 'K')
put(g, 14, 14, 'K')
put(g, 15, 14, 'K')
put(g, 16, 14, 'K')
put(g, 17, 14, 'K')
put(g, 18, 14, 'K')
put(g, 19, 14, 'K')
put(g, 20, 14, 'K')
put(g, 9, 15, 'K')
put(g, 10, 15, 'K')
put(g, 21, 15, 'K')
put(g, 22, 15, 'K')
put(g, 8, 16, 'K')
put(g, 9, 16, 'K')
put(g, 22, 16, 'K')
put(g, 23, 16, 'K')
put(g, 7, 17, 'K')
put(g, 8, 17, 'K')
put(g, 23, 17, 'K')
put(g, 24, 17, 'K')
put(g, 7, 18, 'K')
put(g, 8, 18, 'K')
put(g, 23, 18, 'K')
put(g, 24, 18, 'K')
put(g, 7, 19, 'K')
put(g, 8, 19, 'K')
put(g, 23, 19, 'K')
put(g, 24, 19, 'K')
put(g, 8, 20, 'K')
put(g, 9, 20, 'K')
put(g, 22, 20, 'K')
put(g, 23, 20, 'K')
put(g, 9, 21, 'K')
put(g, 10, 21, 'K')
put(g, 21, 21, 'K')
put(g, 22, 21, 'K')
put(g, 11, 22, 'K')
put(g, 12, 22, 'K')
put(g, 19, 22, 'K')
put(g, 20, 22, 'K')

# Vinyl surface - brown tones
for y in range(15, 22):
    for x in range(10, 22):
        if g[y][x] == '.':
            # Inside the outer ring
            dist_sq = (x - 15.5)**2 + (y - 18)**2
            if dist_sq < 36:  # Inside outer circle
                if dist_sq > 4:  # Outside center hole
                    put(g, x, y, 'D')

# Grooves - lighter rings
put(g, 11, 16, 'B')
put(g, 12, 16, 'B')
put(g, 18, 16, 'B')
put(g, 19, 16, 'B')
put(g, 20, 16, 'B')
put(g, 10, 17, 'B')
put(g, 20, 17, 'B')
put(g, 21, 17, 'B')

# Center label - lighter
put(g, 14, 17, 'H')
put(g, 15, 17, 'H')
put(g, 16, 17, 'H')
put(g, 17, 17, 'H')
put(g, 13, 18, 'H')
put(g, 14, 18, 'H')
put(g, 15, 18, 'H')
put(g, 16, 18, 'H')
put(g, 17, 18, 'H')
put(g, 18, 18, 'H')
put(g, 14, 19, 'H')
put(g, 15, 19, 'H')
put(g, 16, 19, 'H')
put(g, 17, 19, 'H')
# Center hole
put(g, 15, 18, 'K')
put(g, 16, 18, 'K')

# Left claw - holding key
# Arm
for y in range(23, 25):
    for x in range(7, 10):
        if ((x-8)/2)**2 + ((y-24)/1.5)**2 <= 1:
            put(g, x, y, 'B')

# Claw palm
put(g, 6, 25, 'B')
put(g, 7, 25, 'B')
put(g, 8, 25, 'B')
put(g, 6, 26, 'B')
put(g, 7, 26, 'B')
put(g, 8, 26, 'B')
put(g, 7, 26, 'D')

# Key - 2px thick
# Key bow (handle)
put(g, 4, 25, 'K')
put(g, 5, 25, 'K')
put(g, 4, 26, 'K')
put(g, 5, 26, 'K')
put(g, 4, 27, 'K')
put(g, 5, 27, 'K')
# Key shaft
put(g, 3, 26, 'K')
put(g, 2, 26, 'K')
put(g, 1, 26, 'K')
# Key teeth
put(g, 1, 25, 'K')
put(g, 2, 27, 'K')

# Right claw - relaxed
for y in range(23, 25):
    for x in range(22, 25):
        if ((x-23)/2)**2 + ((y-24)/1.5)**2 <= 1:
            put(g, x, y, 'B')

put(g, 23, 25, 'B')
put(g, 24, 25, 'B')
put(g, 25, 25, 'B')
put(g, 23, 26, 'B')
put(g, 24, 26, 'B')
put(g, 25, 26, 'B')
put(g, 24, 26, 'D')

# Legs - 2px thick
both(g, 11, 26, 'B')
both(g, 11, 27, 'B')
both(g, 12, 27, 'B')
both(g, 13, 28, 'B')
both(g, 13, 29, 'B')

# Eyes - alert
both(g, 12, 23, 'C')
both(g, 13, 23, 'C')
both(g, 12, 24, 'C')
both(g, 13, 24, 'C')
# Pupils
both(g, 13, 24, 'K')

# Small serious mouth
put(g, 14, 25, 'C')
put(g, 15, 25, 'C')
put(g, 16, 25, 'C')
put(g, 17, 25, 'C')

# ID badge hanging from neck - 2px thick
# Lanyard
put(g, 15, 22, 'K')
put(g, 16, 22, 'K')
put(g, 15, 23, 'K')
put(g, 16, 23, 'K')
# Badge
put(g, 14, 24, 'F')
put(g, 15, 24, 'F')
put(g, 16, 24, 'F')
put(g, 17, 24, 'F')
put(g, 14, 25, 'F')
put(g, 15, 25, 'F')
put(g, 16, 25, 'F')
put(g, 17, 25, 'F')
# Badge text lines
put(g, 15, 24, 'K')
put(g, 16, 24, 'K')

export(g, 'account-v2', '/sessions/vigilant-magical-gates/mnt/ruseme/mascots/account-v2')
print("Account v2 exported")
