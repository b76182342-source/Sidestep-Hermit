import sys
sys.path.append('/sessions/vigilant-magical-gates/mnt/ruseme/tools')
from pixel import *
from PIL import Image
import numpy as np

def check_mascot(g, name):
    """Check mascot against quality rules"""
    issues = []

    # Rule 1: Check for isolated pixels (except sparkles/motion lines)
    for y in range(1, N-1):
        for x in range(1, N-1):
            if g[y][x] != '.' and g[y][x] != 'L':  # L is allowed for effects
                # Check if pixel has no neighbors
                neighbors = [
                    g[y-1][x], g[y+1][x], g[y][x-1], g[y][x+1],
                    g[y-1][x-1], g[y-1][x+1], g[y+1][x-1], g[y+1][x+1]
                ]
                if all(n == '.' for n in neighbors):
                    issues.append(f"Isolated pixel at ({x},{y}) color {g[y][x]}")

    # Rule 2: Check if cream pixels touch background
    for y in range(N):
        for x in range(N):
            if g[y][x] == 'C':
                # Check neighbors for background
                touches_bg = False
                for dy in [-1, 0, 1]:
                    for dx in [-1, 0, 1]:
                        if dy == 0 and dx == 0:
                            continue
                        ny, nx = y + dy, x + dx
                        if 0 <= ny < N and 0 <= nx < N and g[ny][nx] == '.':
                            touches_bg = True
                            break
                    if touches_bg:
                        break
                if touches_bg:
                    issues.append(f"Cream pixel at ({x},{y}) touches background")

    # Rule 3: Check for thin structures (less than 2px wide)
    # This is complex, so we'll do a simplified check for single-pixel lines
    for y in range(1, N-1):
        for x in range(1, N-1):
            c = g[y][x]
            if c != '.' and c != 'L':  # Skip background and effect pixels
                # Check if this forms a thin vertical line
                if g[y-1][x] == c and g[y+1][x] == c:
                    if g[y][x-1] != c and g[y][x+1] != c:
                        # Vertical line without horizontal neighbors
                        if g[y-1][x-1] != c and g[y-1][x+1] != c and g[y+1][x-1] != c and g[y+1][x+1] != c:
                            issues.append(f"Thin vertical line at ({x},{y})")

                # Check if this forms a thin horizontal line
                if g[y][x-1] == c and g[y][x+1] == c:
                    if g[y-1][x] != c and g[y+1][x] != c:
                        # Horizontal line without vertical neighbors
                        if g[y-1][x-1] != c and g[y+1][x-1] != c and g[y-1][x+1] != c and g[y+1][x+1] != c:
                            issues.append(f"Thin horizontal line at ({x},{y})")

    return issues

def compare_silhouettes(g1, g2, name1, name2):
    """Compare two mascots' silhouettes and return IoU"""
    # Create binary masks
    mask1 = np.zeros((N, N), dtype=bool)
    mask2 = np.zeros((N, N), dtype=bool)

    for y in range(N):
        for x in range(N):
            mask1[y, x] = (g1[y][x] != '.')
            mask2[y, x] = (g2[y][x] != '.')

    # Calculate intersection and union
    intersection = np.logical_and(mask1, mask2).sum()
    union = np.logical_or(mask1, mask2).sum()

    if union == 0:
        return 0.0

    iou = intersection / union
    return iou

def check_all_mascots(mascots_dict):
    """Check all mascots and compare silhouettes"""
    print("\n=== Mascot Quality Check ===\n")

    all_passed = True

    # Individual checks
    for name, g in mascots_dict.items():
        print(f"Checking {name}...")
        issues = check_mascot(g, name)
        if issues:
            all_passed = False
            print(f"  ❌ FAILED:")
            for issue in issues:
                print(f"     - {issue}")
        else:
            print(f"  ✓ Passed individual checks")

    # Silhouette comparison
    print("\n=== Silhouette Similarity Check ===\n")
    names = list(mascots_dict.keys())
    for i in range(len(names)):
        for j in range(i+1, len(names)):
            name1, name2 = names[i], names[j]
            iou = compare_silhouettes(mascots_dict[name1], mascots_dict[name2], name1, name2)
            if iou >= 0.6:
                all_passed = False
                print(f"❌ {name1} and {name2} are TOO SIMILAR (IoU: {iou:.2f})")
                print(f"   Must be < 0.6. These need to be redesigned with different body shapes.")
            else:
                print(f"✓ {name1} vs {name2}: {iou:.2f}")

    print("\n" + "="*40)
    if all_passed:
        print("✅ ALL CHECKS PASSED")
    else:
        print("❌ SOME CHECKS FAILED - Review and fix before export")
    print("="*40 + "\n")

    return all_passed

if __name__ == "__main__":
    print("Mascot quality checker ready.")
    print("Import this module and call check_all_mascots(dict_of_grids)")
