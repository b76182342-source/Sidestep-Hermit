# Sidestep Hermit Mascots Family

A family of 8 pixel art crab mascots for the Sidestep Hermit music player.

**Design Philosophy:** "Carry your shell. Walk sideways."  
All mascots share the warm, minimalist aesthetic with rounded shapes, no outlines, and a consistent color palette.

---

## 🎨 Color Palette

- **B** `#C86B4E` - Brick red (main body color)
- **D** `#A4533A` - Deep brick red (shadows)
- **L** `#E08F72` - Light orange (blush, highlights, motion lines)
- **K** `#2B211E` - Dark brown (pupils, props, accessories)
- **G** `#4A3A33` / **H** `#7A6A62` - Medium browns (details, highlights)
- **C** `#F5EBE1` - Cream (eyes, teeth, highlights, background)

---

## 👥 The Family

### 1. **Scanner Crab** 扫描蟹
- **Species:** Hermit Crab
- **Personality:** Diligent, curious, always exploring
- **Scene:** Scanning music library (loading state)
- **Props:** Magnifying glass in left claw
- **Action:** Forward-leaning posture, examining with magnifying glass
- **Visual ID:** Motion lines behind, focused downward gaze
- **Usage:** Show during library scan, file indexing, metadata loading

---

### 2. **Empty Crab** 空空蟹
- **Species:** Hermit Crab
- **Personality:** Calm, zen, accepting of emptiness
- **Scene:** Empty states (no songs / no search results)
- **Props:** None (that's the point)
- **Action:** Both claws open in shrugging gesture
- **Visual ID:** Neutral expression, symmetrical pose
- **Usage:** Empty playlist, no search results, cleared queue

---

### 3. **Oops Crab** 哎呀蟹
- **Species:** Hermit Crab
- **Personality:** Apologetic, gentle, slightly flustered
- **Scene:** Error states (network failure / permission issues)
- **Props:** Sweat drops above head
- **Action:** One claw covering face (embarrassed), one drooping
- **Visual ID:** Wide worried eyes, O-shaped mouth, prominent blush
- **Usage:** Network errors, file not found, permission denied (keep it friendly, not scary)

---

### 4. **Lyric Crab** 歌词蟹
- **Species:** Hermit Crab
- **Personality:** Poetic, expressive, loves words and melody
- **Scene:** Lyrics display
- **Props:** Microphone in left claw
- **Action:** Singing with closed eyes, mouth open
- **Visual ID:** Blissful expression, music notes floating nearby
- **Usage:** Lyrics panel, karaoke mode, song info with lyrics

---

### 5. **DJ Crab** DJ蟹
- **Species:** Hermit Crab
- **Personality:** Energetic, rhythmic, party animal
- **Scene:** Equalizer & volume control
- **Props:** Headphones over head/ears
- **Action:** Both claws raised high (dancing/DJing)
- **Visual ID:** Excited large eyes, sound waves on sides
- **Usage:** Equalizer settings, volume mixer, audio effects panel

---

### 6. **Sleepy Crab** 困困蟹
- **Species:** Hermit Crab
- **Personality:** Drowsy, slow-paced, loves napping
- **Scene:** Sleep timer
- **Props:** Night cap with drooping tip and pom-pom, ZZZ symbols
- **Action:** Drooping claws, closed eyes, slightly open mouth
- **Visual ID:** Relaxed posture, ZZZ floating above
- **Usage:** Sleep timer settings, auto-shutdown, night mode

---

### 7. **Search Crab** 搜索蟹
- **Species:** Hermit Crab
- **Personality:** Focused, precise, detective-like
- **Scene:** Search function
- **Props:** Binoculars/telescope in left claw, detective cap
- **Action:** Looking through binoculars, right claw pointing
- **Visual ID:** Concentrated gaze, radar waves
- **Usage:** Search bar active, search results loading, filter panel

---

### 8. **Account Crab** 账号蟹
- **Species:** Hermit Crab
- **Personality:** Reliable, protective, guardian
- **Scene:** Login & account management
- **Props:** Key in left claw, shield badge on body
- **Action:** Right claw crossed over chest (guard stance)
- **Visual ID:** Serious eyebrows, steady gaze, upright posture
- **Usage:** Login screen, account settings, profile management, authentication

---

## 📁 File Structure

Each mascot has its own directory with the following exports:

```
mascots/
├── scanner/
│   ├── scanner-32.png           # 32×32 source (cream background)
│   ├── scanner.png               # 256×256 transparent (for UI)
│   ├── scanner-icon-1024.png    # 1024×1024 rounded square icon
│   ├── scanner-icon-macos-1024.png # 1024×1024 macOS style (centered with margin)
│   └── scanner.svg               # Scalable vector (crisp edges)
├── empty/
│   └── ...
├── oops/
│   └── ...
├── lyric/
│   └── ...
├── dj/
│   └── ...
├── sleepy/
│   └── ...
├── search/
│   └── ...
└── account/
    └── ...
```

---

## 🎯 Usage Guidelines

### Size Recommendations
- **UI elements:** Use 256px transparent PNGs, scale down as needed with nearest-neighbor
- **Icons:** Use 1024px versions
- **Scalable:** Use SVG for any size (but test at target size first)

### Animation Ideas
- **Scanner:** Two frames - magnifying glass slightly moving
- **Empty:** Gentle breathing (subtle scale)
- **Oops:** Sweat drops animating
- **Lyric:** Music notes floating up
- **DJ:** Claws bobbing up/down
- **Sleepy:** ZZZ appearing sequentially
- **Search:** Radar waves expanding
- **Account:** Shield glowing

### Consistency Rules
- Always use nearest-neighbor scaling (no blur)
- Maintain cream background for consistency in documentation
- Use transparent PNGs for actual UI implementation
- Each mascot should appear in context appropriate to its role

---

## 🛠 Technical Details

- **Canvas:** 32×32 pixels
- **Character size:** ~24×24 pixels with 3-4px margins
- **Style:** Flat design, no outlines, no gradients
- **Eyes:** 2×2 cream squares with dark pupils (offset inner-bottom for cuteness)
- **Shadows:** Always use color D at bottom of shapes
- **Blush:** Color L, 1-2 pixels per cheek

---

## 🎨 Design Credits

Created with `tools/pixel.py` - a Python script for procedural pixel art generation.

All mascots are original designs for Sidestep Hermit.  
No third-party characters or brand mascots were referenced or imitated.
