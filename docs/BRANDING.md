# Antyx-OS Branding

## Included

- Antyx logo in SVG and PNG formats
- 4K default wallpaper
- KDE color scheme
- SDDM login theme
- Plymouth boot theme
- Fastfetch preset
- first-login wallpaper and color setup
- white-purple website

## Testing

After installing the new image:

```bash
antyx-info
fastfetch --config /usr/share/fastfetch/presets/antyx.jsonc
```

Verify:

- the wallpaper is applied
- the Antyx KDE color scheme is active
- the login screen uses Antyx branding
- Brave, Steam, Heroic, and Lutris launch correctly
- `antyx-check` reports the expected security state

## Plymouth note

The Plymouth theme files are included. Atomic systems may require an additional initramfs build step before the boot theme is visible. This must be tested separately.
