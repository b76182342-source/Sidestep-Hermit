import os
from PIL import Image, ImageDraw

N = 32
PAL = {'B':'#C86B4E','D':'#A4533A','L':'#E08F72','K':'#2B211E',
       'G':'#4A3A33','H':'#7A6A62','C':'#F5EBE1',
       'S':'#C8CFBF','F':'#BFD0D9','P':'#CFC6D8'}  # Sage, Fog blue, Purple

def new(): return [['.'] * N for _ in range(N)]
def put(g, x, y, c):
    if 0 <= x < N and 0 <= y < N: g[y][x] = c
def mir(x): return N - 1 - x
def both(g, x, y, c): put(g, x, y, c); put(g, mir(x), y, c)
def ellipse(g, cx, cy, rx, ry, color):
    for y in range(N):
        for x in range(N):
            if ((x - cx) / rx) ** 2 + ((y - cy) / ry) ** 2 <= 1:
                put(g, x, y, color(x, y) if callable(color) else color)
def rgb(c): return tuple(int(PAL[c][i:i+2], 16) for i in (1, 3, 5))

def _rounded(im, frac):
    w, h = im.size; ss = 4
    m = Image.new('L', (w*ss, h*ss), 0)
    ImageDraw.Draw(m).rounded_rectangle([0, 0, w*ss-1, h*ss-1], radius=int(w*ss*frac), fill=255)
    out = im.convert('RGBA'); out.putalpha(m.resize((w, h), Image.LANCZOS)); return out

def export(g, name, outdir):
    os.makedirs(outdir, exist_ok=True)
    base = Image.new('RGB', (N, N), rgb('C'))
    tr = Image.new('RGBA', (N, N), (0, 0, 0, 0))
    for y in range(N):
        for x in range(N):
            if g[y][x] != '.':
                base.putpixel((x, y), rgb(g[y][x]))
                tr.putpixel((x, y), rgb(g[y][x]) + (255,))
    base.save(f'{outdir}/{name}-32.png')
    tr.resize((256, 256), Image.NEAREST).save(f'{outdir}/{name}.png')
    _rounded(base.resize((1024, 1024), Image.NEAREST), 230/1024).save(f'{outdir}/{name}-icon-1024.png')
    art = _rounded(base.resize((800, 800), Image.NEAREST), 0.2237)
    cv = Image.new('RGBA', (1024, 1024), (0, 0, 0, 0)); cv.paste(art, (112, 112), art)
    cv.save(f'{outdir}/{name}-icon-macos-1024.png')
    rects = []
    for y in range(N):
        x = 0
        while x < N:
            c = g[y][x]
            if c == '.': x += 1; continue
            x2 = x
            while x2 + 1 < N and g[y][x2 + 1] == c: x2 += 1
            rects.append(f'<rect x="{x}" y="{y}" width="{x2-x+1}" height="1" fill="{PAL[c]}"/>')
            x = x2 + 1
    svg = ('<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32" width="1024" height="1024" '
           'shape-rendering="crispEdges" role="img" aria-label="' + name + '">'
           '<rect width="32" height="32" rx="7.2" fill="#F5EBE1" shape-rendering="geometricPrecision"/>'
           + ''.join(rects) + '</svg>')
    open(f'{outdir}/{name}.svg', 'w').write(svg)
