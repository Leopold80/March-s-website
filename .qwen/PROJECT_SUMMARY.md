# Project Summary

## Overall Goal
Build and maintain a personal website using Rust + Axum with Markdown blog functionality and a media wall for photos/videos, deployed on a Raspberry Pi with tunneling for public access.

## Key Knowledge

### Technology Stack
- **Backend**: Rust + Axum 0.8 (with `multipart` feature)
- **Async Runtime**: Tokio
- **Markdown Parsing**: pulldown-cmark 0.13
- **Static Files**: tower-http 0.6 `ServeDir`
- **Deployment**: Raspberry Pi (user: `cat`) with systemd user services

### Architecture Decisions
- HTML templates use `include_str!()` for compile-time embedding
- Media files served via `ServeDir` at runtime (supports large files)
- File-based storage (no database): `md_notes/` for Markdown, `media/photos/` and `media/videos/` for media
- On-demand scanning: new content appears immediately without recompilation
- **Upload limit**: 30GB (configured via `DefaultBodyLimit::max(30 * 1024 * 1024 * 1024)`)

### Critical Configuration
- **systemd service**: User-level service (`~/.config/systemd/user/marchs-website.service`)
- **Service file**: Uses `%h` placeholder for home directory (no hardcoded paths)
- **No `User=` directive** in user services (causes error 216/GROUP)
- **Gitignore**: `md_notes/` and `media/` are ignored (user content)

### User Preferences
- Concise code without verbose comments or analogies
- Well-encapsulated code with clear separation of concerns
- Graceful degradation for optional features (e.g., ffmpeg for HEIC conversion)
- Mobile-friendly UI design (44px minimum touch targets, responsive layouts)

### Build & Run Commands
```bash
cargo run              # Development mode
cargo build --release  # Production build
```

### Deployment Commands
```bash
# First deploy
python deploy/deploy.py

# Update (preserves data)
python deploy/deploy.py --update

# Uninstall (keep data)
python deploy/uninstall.py --keep-data

# Service management
systemctl --user status marchs-website
systemctl --user restart marchs-website
journalctl --user -u marchs-website -f
```

### Routes
| Path | Method | Description |
|------|--------|-------------|
| `/` | GET | Homepage |
| `/logs` | GET | Log list |
| `/log/{slug}` | GET | Single log view |
| `/edit-log?slug={}` | GET | Edit log page |
| `/media` | GET | Media wall |
| `/upload-media` | GET | Upload page |
| `/api/upload/media` | POST | Upload handler (30GB limit) |
| `/api/logs` | POST | Create log |
| `/api/logs/{slug}` | PUT | Update log |
| `/api/logs/{slug}` | DELETE | Delete log |
| `/api/media/{filename}/{type}` | PUT/DELETE | Rename/Delete media |

## Recent Actions

### v0.1.0 Release
- Created git tag `v0.1.0` marking basic feature completeness
- All core functionality implemented and tested

### Upload Progress Bar
- Added real-time progress bar for media uploads
- Uses `XMLHttpRequest` instead of `fetch` for progress events
- Gradient progress bar with percentage display
- Upload success redirects to media wall

### Cancel Upload Feature
- Added cancel button during upload
- Fixed CSS `pointer-events` issue (cancel button was unclickable)
- `xhr.abort()` cancels the upload gracefully

### Delete Log API
- Added `DELETE /api/logs/{slug}` endpoint
- Created `delete_log` handler in `src/handlers/logs.rs`
- Frontend already had delete button in edit page

### Gitignore Fix
- Changed `logs/` to `md_notes/` (actual directory name)

### Deployment Script Fixes
- **systemd service file extension**: Added `.service` suffix
- **Removed `User=` directive**: Causes error 216/GROUP in user services
- **Use `%h` placeholder**: Auto-expands to user home directory
- **Update mode**: Stop service before overwriting binary
- **Pre-stop check**: Only stop if service is actually running

### Uninstall Script
- Created `deploy/uninstall.py`
- Options: full uninstall or `--keep-data` (preserves `md_notes/` and `media/`)
- Stops service, disables auto-start, removes service file, deletes deployment directory

## Current Plan

1. [DONE] Basic website functionality (homepage, logs, media wall)
2. [DONE] Markdown log support with frontmatter
3. [DONE] Media wall waterfall layout
4. [DONE] HEIC photo to JPEG conversion
5. [DONE] systemd service deployment
6. [DONE] Deploy and uninstall scripts
7. [DONE] Upload progress bar
8. [DONE] Cancel upload functionality
9. [DONE] Delete log API endpoint
10. [DONE] Git tag v0.1.0
11. [TODO] Image thumbnails for faster loading
12. [TODO] Media wall pagination
13. [TODO] Test HEIC conversion on Raspberry Pi

## Open Issues / Notes

- **MOV video format**: May have compatibility issues on some browsers (MP4 preferred)
- **Local testing IP**: Changes per network (check with `hostname -I`)
- **Remote repository**: `git@github.com:Leopold80/March-s-website.git`
- **Deployment directory**: `~/marchs-website` (auto-detected from current user)
- **Data directories preserved during update**: `md_notes/`, `media/photos/`, `media/videos/`

## Summary Metadata
**Update time**: 2026-05-20
**Version**: v0.1.0

---

## Summary Metadata
**Update time**: 2026-05-19T17:16:39.950Z 
