import sys
sys.path.append('/sessions/vigilant-magical-gates/mnt/ruseme/tools')
from pixel import *

g = new()

# DJ Crab - Wide, flat, low crab with mixer in front
# Body - very wide and flat, low position
ellipse(g, 15.5, 23, 11, 3.5, 'B')
# Shadow
for x in range(5, 27):
    if g[26][x] == 'B': put(g, x, 26, 'D')
    if g[25][x] == 'B': put(g, x, 25, 'D')

# Headphones - ear cups on sides of body
# Left ear cup
put(g, 5, 21, 'K')
put(g, 6, 21, 'K')
put(g, 7, 21, 'K')
put(g, 5, 22, 'K')
put(g, 6, 22, 'K')
put(g, 7, 22, 'K')
put(g, 5, 23, 'K')
put(g, 6, 23, 'K')
put(g, 7, 23, 'K')
put(g, 5, 24, 'K')
put(g, 6, 24, 'K')
put(g, 7, 24, 'K')
# Highlight
put(g, 6, 22, 'H')
put(g, 6, 23, 'H')

# Right ear cup
put(g, 24, 21, 'K')
put(g, 25, 21, 'K')
put(g, 26, 21, 'K')
put(g, 24, 22, 'K')
put(g, 25, 22, 'K')
put(g, 26, 22, 'K')
put(g, 24, 23, 'K')
put(g, 25, 23, 'K')
put(g, 26, 23, 'K')
put(g, 24, 24, 'K')
put(g, 25, 24, 'K')
put(g, 26, 24, 'K')
# Highlight
put(g, 25, 22, 'H')
put(g, 25, 23, 'H')

# Headband - arc over top
put(g, 8, 20, 'K')
put(g, 9, 20, 'K')
put(g, 10, 19, 'K')
put(g, 11, 19, 'K')
put(g, 12, 18, 'K')
put(g, 13, 18, 'K')
put(g, 14, 18, 'K')
put(g, 15, 18, 'K')
put(g, 16, 18, 'K')
put(g, 17, 18, 'K')
put(g, 18, 18, 'K')
put(g, 19, 19, 'K')
put(g, 20, 19, 'K')
put(g, 21, 20, 'K')
put(g, 22, 20, 'K')
put(g, 23, 20, 'K')

# Small claws on sides
# Left claw
put(g, 3, 22, 'B')
put(g, 4, 22, 'B')
put(g, 3, 23, 'B')
put(g, 4, 23, 'B')
put(g, 2, 21, 'B')
put(g, 3, 21, 'B')

# Right claw
put(g, 27, 22, 'B')
put(g, 28, 22, 'B')
put(g, 27, 23, 'B')
put(g, 28, 23, 'B')
put(g, 28, 21, 'B')
put(g, 29, 21, 'B')

# Short legs - 2px thick
both(g, 9, 25, 'B')
both(g, 9, 26, 'B')
both(g, 10, 26, 'B')
both(g, 11, 27, 'B')
both(g, 11, 28, 'B')

# Eyes - wide and excited
both(g, 11, 21, 'C')
both(g, 12, 21, 'C')
both(g, 13, 21, 'C')
both(g, 11, 22, 'C')
both(g, 12, 22, 'C')
both(g, 13, 22, 'C')
# Large pupils
both(g, 12, 21, 'K')
both(g, 13, 21, 'K')
both(g, 12, 22, 'K')

# Mouth - big smile
put(g, 14, 24, 'C')
put(g, 15, 24, 'C')
put(g, 16, 24, 'C')
put(g, 17, 24, 'C')

# Mixer console in front - fog blue color
# Base
put(g, 11, 27, 'F')
put(g, 12, 27, 'F')
put(g, 13, 27, 'F')
put(g, 14, 27, 'F')
put(g, 15, 27, 'F')
put(g, 16, 27, 'F')
put(g, 17, 27, 'F')
put(g, 18, 27, 'F')
put(g, 19, 27, 'F')
put(g, 20, 27, 'F')
put(g, 11, 28, 'F')
put(g, 12, 28, 'F')
put(g, 13, 28, 'F')
put(g, 14, 28, 'F')
put(g, 15, 28, 'F')
put(g, 16, 28, 'F')
put(g, 17, 28, 'F')
put(g, 18, 28, 'F')
put(g, 19, 28, 'F')
put(g, 20, 28, 'F')

# 4 faders - dark lines
put(g, 12, 25, 'K')
put(g, 12, 26, 'K')
put(g, 15, 24, 'K')
put(g, 15, 25, 'K')
put(g, 15, 26, 'K')
put(g, 18, 25, 'K')
put(g, 18, 26, 'K')
put(g, 21, 23, 'K')
put(g, 21, 24, 'K')
put(g, 21, 25, 'K')
put(g, 21, 26, 'K')

# Fader knobs
put(g, 11, 25, 'H')
put(g, 12, 25, 'H')
put(g, 13, 25, 'H')
put(g, 14, 24, 'H')
put(g, 15, 24, 'H')
put(g, 16, 24, 'H')
put(g, 17, 25, 'H')
put(g, 18, 25, 'H')
put(g, 19, 25, 'H')
put(g, 20, 23, 'H')
put(g, 21, 23, 'H')
put(g, 22, 23, 'H')

export(g, 'dj-v2', '/sessions/vigilant-magical-gates/mnt/ruseme/mascots/dj-v2')
print("DJ v2 exported")
