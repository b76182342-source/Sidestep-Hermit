import sys
sys.path.append('/sessions/vigilant-magical-gates/mnt/ruseme/tools')
from pixel import *
from check import check_all_mascots

# Load the three grids
def load_grid_from_image(path):
    """Reconstruct grid from exported image"""
    from PIL import Image
    img = Image.open(path)
    g = new()

    # Reverse mapping
    color_map = {}
    for code, hex_color in PAL.items():
        color_map[tuple(int(hex_color[i:i+2], 16) for i in (1, 3, 5))] = code

    for y in range(32):
        for x in range(32):
            pixel = img.getpixel((x, y))
            if pixel in color_map:
                g[y][x] = color_map[pixel]

    return g

# Load all three
scanner = load_grid_from_image('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/scanner-v2/scanner-v2-32.png')
empty = load_grid_from_image('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/empty-v2/empty-v2-32.png')
oops = load_grid_from_image('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/oops-v2/oops-v2-32.png')

mascots = {
    'scanner': scanner,
    'empty': empty,
    'oops': oops
}

# Run checks
check_all_mascots(mascots)
