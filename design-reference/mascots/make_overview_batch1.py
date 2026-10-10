from PIL import Image

# Load the three 32x32 mascots
scanner = Image.open('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/scanner/scanner-32.png')
empty = Image.open('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/empty/empty-32.png')
oops = Image.open('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/oops/oops-32.png')

# Create overview image: 3 columns, scaled up
scale = 8
w, h = 32 * scale, 32 * scale
spacing = 20

overview = Image.new('RGB', (w * 3 + spacing * 4, h + spacing * 2), (245, 235, 225))

# Paste scaled versions
scanner_big = scanner.resize((w, h), Image.NEAREST)
empty_big = empty.resize((w, h), Image.NEAREST)
oops_big = oops.resize((w, h), Image.NEAREST)

overview.paste(scanner_big, (spacing, spacing))
overview.paste(empty_big, (w + spacing * 2, spacing))
overview.paste(oops_big, (w * 2 + spacing * 3, spacing))

overview.save('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/overview-batch1.png')
print("Batch 1 overview exported")
