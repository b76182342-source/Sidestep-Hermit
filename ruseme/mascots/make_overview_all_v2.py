from PIL import Image

# Load all 6 mascots
scanner = Image.open('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/scanner-v2/scanner-v2-32.png')
empty = Image.open('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/empty-v2/empty-v2-32.png')
oops = Image.open('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/oops-v2/oops-v2-32.png')
lyric = Image.open('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/lyric-v2/lyric-v2-32.png')
dj = Image.open('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/dj-v2/dj-v2-32.png')
sleepy = Image.open('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/sleepy-v2/sleepy-v2-32.png')
search = Image.open('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/search-v2/search-v2-32.png')
account = Image.open('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/account-v2/account-v2-32.png')

# Create overview image: 4 columns x 2 rows, scaled up
scale = 6
w, h = 32 * scale, 32 * scale
spacing = 15

cols = 4
rows = 2
overview = Image.new('RGB', (w * cols + spacing * (cols + 1), h * rows + spacing * (rows + 1)), (245, 235, 225))

mascots = [scanner, empty, oops, lyric, dj, sleepy, search, account]

for idx, img in enumerate(mascots):
    row = idx // cols
    col = idx % cols

    x = spacing + col * (w + spacing)
    y = spacing + row * (h + spacing)

    img_big = img.resize((w, h), Image.NEAREST)
    overview.paste(img_big, (x, y))

overview.save('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/overview-all-v2.png')
print("All 8 mascots v2 overview exported")
