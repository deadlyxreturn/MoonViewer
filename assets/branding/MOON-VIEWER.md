# Moon Viewer artwork

- `moon-viewer-login.png` is the approved 1672 × 941 ImageGen background
  reconstructed from a user-supplied, Sailor Moon-inspired concept.
- `moon-viewer-brand-source.png` is the approved 1536 × 1024 ImageGen logo.
  Its named lockup is used on the login screen and in the project README.
- `moon-viewer-glass-login-frame.png`, `moon-viewer-glossy-login-button.png`
  and `moon-viewer-preferences-frame.png` are the supplied transparent
  ImageGen decorations. Text, fields, and actions are rendered by the
  viewer's native UI.
- `moon-viewer-icon.png` and `moon-viewer.ico` come from the same image.
  The badge is separated from the wordmark for small Windows icon sizes.
  The ICO contains 16, 20, 24, 32, 40, 48, 64, 128 and 256 pixel frames.
  Regenerate the derivatives with `python scripts/draw-moon-brand.py`
  and Pillow.
- The upstream bear and wave remain available as alternate login
  backgrounds. See [NOTICE.md](../../NOTICE.md) for upstream notices.

The supplied concept is used for visual comparison. It is separate from the
runtime assets and is not included in the package. The origin of the art
does not establish a licence to any third-party character or trademark.
