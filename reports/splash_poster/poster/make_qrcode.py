import qrcode
from qrcode.image.styledpil import StyledPilImage
from qrcode.image.styles.moduledrawers import RoundedModuleDrawer
from qrcode.image.styles.colormasks import SolidFillColorMask
from qrcode.image.pil import PilImage
from PIL import Image, ImageDraw

def make_silhouette_logo(logo_path, color=(255,255,255), mode="dark",
                         alpha_threshold=20, dark_threshold=160):
    """Convert logo into silhouette (dark→white or any→white)."""
    logo = Image.open(logo_path).convert("RGBA")
    px = logo.load()
    w, h = logo.size
    for y in range(h):
        for x in range(w):
            r, g, b, a = px[x, y]
            if a <= alpha_threshold:
                px[x, y] = (255, 255, 255, 0)
                continue
            if mode == "any":                       # VSCode
                px[x, y] = (*color, 255)
            else:                                   # GitHub
                if r < dark_threshold and g < dark_threshold and b < dark_threshold:
                    px[x, y] = (*color, 255)
                else:
                    px[x, y] = (255, 255, 255, 0)
    return logo

def make_qr_image(qr, bg_color, mode="styled"):
    """
    mode:
      - 'styled' -> rounded modules (StyledPilImage)
      - 'plain'  -> square modules (PilImage) — guaranteed correct on white/black
    """
    if mode == "plain":
        return qr.make_image(
            image_factory=PilImage,
            fill_color=(255, 255, 255),     # white modules
            back_color=bg_color             # black background when bg_color=(0,0,0)
        ).convert("RGBA")
    else:
        return qr.make_image(
            image_factory=StyledPilImage,
            module_drawer=RoundedModuleDrawer(),
            color_mask=SolidFillColorMask(front_color=(255,255,255), back_color=bg_color),
        ).convert("RGBA")

def make_styled_qr(data, filename, bg_color, logo_path=None,
                   size=20, silhouette=False, silhouette_mode="dark",
                   box=True, render_mode="styled"):
    """
    render_mode: 'styled' (rounded modules) or 'plain' (square; reliable for black bg)
    """
    qr = qrcode.QRCode(
        version=4,
        error_correction=qrcode.constants.ERROR_CORRECT_H,
        box_size=size,
        border=2,
    )
    qr.add_data(data)
    qr.make(fit=True)

    img = make_qr_image(qr, bg_color, mode=render_mode)

    if logo_path:
        qr_w, qr_h = img.size
        box_size = qr_w // 4

        if box:
            draw = ImageDraw.Draw(img)
            x0 = (qr_w - box_size) // 2
            y0 = (qr_h - box_size) // 2
            x1, y1 = x0 + box_size, y0 + box_size
            radius = box_size // 6
            # center box fill == bg_color (e.g., black for DOI, blue/purple for others)
            draw.rounded_rectangle([x0, y0, x1, y1], radius=radius, fill=bg_color)
            outline_w = max(4 if bg_color == (0,0,0) else 2, box_size // 25)
            draw.rounded_rectangle([x0, y0, x1, y1], radius=radius,
                                   outline=(255,255,255), width=outline_w)

        # logo
        if silhouette:
            logo = make_silhouette_logo(logo_path, color=(255, 255, 255), mode=silhouette_mode)
        else:
            logo = Image.open(logo_path).convert("RGBA")

        logo_size = int(box_size * 0.65)
        logo = logo.resize((logo_size, logo_size), Image.LANCZOS)
        pos = ((qr_w - logo_size) // 2, (qr_h - logo_size) // 2)
        img.alpha_composite(logo, pos)

    img.save(filename)
    print(f"Saved QR to {filename}")

if __name__ == "__main__":
    # GitHub → purple, white Octocat silhouette, rounded modules
    make_styled_qr(
        "https://github.com/RuleBrittonica/rem-cli",
        "poster/images/github.png",
        bg_color=(128, 0, 128),
        logo_path="poster/icons/github_logo.png",
        silhouette=True,
        silhouette_mode="dark",
        box=True,
        render_mode="styled"
    )

    # VSCode → blue, white silhouette, rounded modules
    make_styled_qr(
        "https://marketplace.visualstudio.com/items?itemName=MatthewBritton.remvscode",
        "poster/images/vscode.png",
        bg_color=(0, 102, 204),
        logo_path="poster/icons/vscode_logo.png",
        silhouette=True,
        silhouette_mode="any",
        box=True,
        render_mode="styled"
    )

    # DOI → white on black, **plain renderer** to avoid the black-on-black issue,
    #       rounded black center box with thicker white outline, colored DOI logo
    make_styled_qr(
        "https://doi.org/10.1145/3758316.3765486",
        "poster/images/doi.png",
        bg_color=(0, 0, 0),
        logo_path="poster/icons/doi_logo.png",
        silhouette=False,
        box=True,
        render_mode="plain"   # <- key change
    )
