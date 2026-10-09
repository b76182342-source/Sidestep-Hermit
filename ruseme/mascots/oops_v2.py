import sys
sys.path.append('/sessions/vigilant-magical-gates/mnt/ruseme/tools')
from pixel import *

g = new()

# Oops Crab - Spider crab with triangular body and long legs tangled in cable
# Body - triangular, higher up to make room for long legs
for y in range(14, 20):
    for x in range(8, 24):
        # Triangle shape: wider at bottom
        width_at_y = int(8 + (y - 14) * 1.3)
        center = 16
        left = center - width_at_y // 2
        right = center + width_at_y // 2
        if left <= x <= right:
            if ((x-16)/(width_at_y/2))**2 + ((y-17)/3)**2 <= 1:
                put(g, x, y, 'B')

# Shadow
for x in range(10, 22):
    if g[19][x] == 'B': put(g, x, 19, 'D')

# Long legs - 2px thick, tangled
# Front left leg - going down-left, tangled
put(g, 10, 19, 'B')
put(g, 11, 19, 'B')
put(g, 9, 20, 'B')
put(g, 10, 20, 'B')
put(g, 8, 21, 'B')
put(g, 9, 21, 'B')
put(g, 7, 22, 'B')
put(g, 8, 22, 'B')
put(g, 6, 23, 'B')
put(g, 7, 23, 'B')
put(g, 6, 24, 'B')
put(g, 7, 24, 'B')
put(g, 7, 25, 'B')
put(g, 8, 25, 'B')

# Front right leg - going down-right, tangled
put(g, 20, 19, 'B')
put(g, 21, 19, 'B')
put(g, 21, 20, 'B')
put(g, 22, 20, 'B')
put(g, 22, 21, 'B')
put(g, 23, 21, 'B')
put(g, 23, 22, 'B')
put(g, 24, 22, 'B')
put(g, 24, 23, 'B')
put(g, 25, 23, 'B')
put(g, 24, 24, 'B')
put(g, 25, 24, 'B')
put(g, 23, 25, 'B')
put(g, 24, 25, 'B')

# Middle legs - shorter
put(g, 12, 18, 'B')
put(g, 13, 18, 'B')
put(g, 11, 19, 'B')
put(g, 12, 19, 'B')
put(g, 10, 20, 'B')
put(g, 11, 20, 'B')

put(g, 18, 18, 'B')
put(g, 19, 18, 'B')
put(g, 19, 19, 'B')
put(g, 20, 19, 'B')
put(g, 20, 20, 'B')
put(g, 21, 20, 'B')

# Cable tangling around legs - dark brown
put(g, 8, 23, 'K')
put(g, 9, 23, 'K')
put(g, 10, 24, 'K')
put(g, 11, 24, 'K')
put(g, 12, 25, 'K')
put(g, 13, 26, 'K')
put(g, 14, 26, 'K')
put(g, 15, 27, 'K')
put(g, 16, 27, 'K')
put(g, 17, 27, 'K')
put(g, 18, 26, 'K')
put(g, 19, 26, 'K')
put(g, 20, 25, 'K')
put(g, 21, 25, 'K')
put(g, 22, 24, 'K')
put(g, 23, 23, 'K')

# Small claws - not prominent
put(g, 7, 17, 'B')
put(g, 8, 17, 'B')
put(g, 7, 18, 'B')
put(g, 8, 18, 'B')

put(g, 23, 17, 'B')
put(g, 24, 17, 'B')
put(g, 23, 18, 'B')
put(g, 24, 18, 'B')

# Eyes - worried but gentle (inner eyebrows down)
# Left eye
put(g, 12, 15, 'C')
put(g, 13, 15, 'C')
put(g, 12, 16, 'C')
put(g, 13, 16, 'C')
put(g, 13, 16, 'K')

# Right eye
put(g, 18, 15, 'C')
put(g, 19, 15, 'C')
put(g, 18, 16, 'C')
put(g, 19, 16, 'C')
put(g, 18, 16, 'K')

# Gentle worried eyebrows - inner parts lower (sympathetic expression)
put(g, 13, 14, 'K')
put(g, 14, 13, 'K')
put(g, 17, 13, 'K')
put(g, 18, 14, 'K')

# Small wavy mouth (worried but not angry)
put(g, 14, 17, 'C')
put(g, 15, 18, 'C')
put(g, 16, 18, 'C')
put(g, 17, 17, 'C')

# Single sweat drop
put(g, 11, 13, 'L')
put(g, 11, 14, 'L')
put(g, 10, 14, 'L')

# Blush
put(g, 10, 16, 'L')
put(g, 21, 16, 'L')

export(g, 'oops-v2', '/sessions/vigilant-magical-gates/mnt/ruseme/mascots/oops-v2')
print("Oops v2 exported")
