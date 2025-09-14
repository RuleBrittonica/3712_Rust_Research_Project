from PIL import Image, ImageDraw

def make_hex_bg(width=1000, height=1000, hex_size=60, color=(184, 134, 11, 40)):
    """
    Generate faint hexagon background.
    - width, height: size in px
    - hex_size: diameter of each hex
    - color: RGBA tuple (40 = ~15% opacity)
    """
    img = Image.new("RGBA", (width, height), (255, 255, 255, 0))
    draw = ImageDraw.Draw(img)

    dx = hex_size * 3**0.5 / 2
    dy = hex_size * 0.75

    for y in range(0, height, int(dy)):
        offset = dx if (y // int(dy)) % 2 else 0
        for x in range(0, width, int(2*dx)):
            cx, cy = int(x+offset), y
            # Draw hexagon
            points = [
                (cx + hex_size*0.5, cy),
                (cx + hex_size*0.25, cy + dy),
                (cx - hex_size*0.25, cy + dy),
                (cx - hex_size*0.5, cy),
                (cx - hex_size*0.25, cy - dy),
                (cx + hex_size*0.25, cy - dy),
            ]
            draw.polygon(points, fill=color)

    # Gradient fade top
    fade = Image.new("L", (width, height), 0)
    for y in range(height):
        fade_alpha = int(255 * (1 - min(1, y/400)))  # fade over top 400px
        for x in range(width):
            fade.putpixel((x,y), fade_alpha)

    img.putalpha(fade)
    img.save("poster/images/hex_bg.png")
    print("Hex background saved.")

if __name__ == "__main__":
    make_hex_bg()