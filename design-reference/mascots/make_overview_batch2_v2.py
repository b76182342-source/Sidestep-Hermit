from PIL import Image

# Load the three 32x32 mascots
lyric = Image.open('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/lyric-v2/lyric-v2-32.png')
dj = Image.open('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/dj-v2/dj-v2-32.png')
sleepy = Image.open('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/sleepy-v2/sleepy-v2-32.png')

# Create overview image: 3 columns, scaled up
scale = 8
w, h = 32 * scale, 32 * scale
spacing = 20

overview = Image.new('RGB', (w * 3 + spacing * 4, h + spacing * 2), (245, 235, 225))

# Paste scaled versions
lyric_big = lyric.resize((w, h), Image.NEAREST)
dj_big = dj.resize((w, h), Image.NEAREST)
sleepy_big = sleepy.resize((w, h), Image.NEAREST)

overview.paste(lyric_big, (spacing, spacing))
overview.paste(dj_big, (w + spacing * 2, spacing))
overview.paste(sleepy_big, (w * 2 + spacing * 3, spacing))

overview.save('/sessions/vigilant-magical-gates/mnt/ruseme/mascots/overview-batch2-v2.png')
print("Batch 2 v2 overview exported")
