from PIL import Image, ImageDraw, ImageFont

# Load all 8 mascots
mascots = [
    ('scanner', 'Scanner\nScanning'),
    ('empty', 'Empty\nNo Content'),
    ('oops', 'Oops\nError'),
    ('lyric', 'Lyric\nLyrics'),
    ('dj', 'DJ\nEqualizer'),
    ('sleepy', 'Sleepy\nSleep Timer'),
    ('search', 'Search\nSearch'),
    ('account', 'Account\nLogin')
]

scale = 6
w, h = 32 * scale, 32 * scale
spacing = 15
label_height = 40

# 4 columns x 2 rows
cols = 4
rows = 2
overview_width = w * cols + spacing * (cols + 1)
overview_height = (h + label_height) * rows + spacing * (rows + 1)

overview = Image.new('RGB', (overview_width, overview_height), (245, 235, 225))
draw = ImageDraw.Draw(overview)

for idx, (name, label) in enumerate(mascots):
    img = Image.open(f'/sessions/vigilant-magical-gates/mnt/ruseme/mascots/{name}/{name}-32.png')
    img_scaled = img.resize((w, h), Image.NEAREST)

    row = idx // cols
    col = idx % cols

    x = spacing + col * (w + spacing)
    y = spacing + row * (h + label_height + spacing)

    overview.paste(img_scaled, (x, y))

    # Draw label
    text_x = x + w // 2
    text_y = y + h + 10
    draw.text((text_x, text_y), label, fill=(43, 33, 30), anchor="ma", font=None)

overview.save('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/overview-all.png')
print("Complete overview exported")
