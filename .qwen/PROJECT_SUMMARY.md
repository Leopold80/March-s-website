# Project Summary

## Overall Goal
Build and maintain a personal website using Rust + Axum with Markdown blog functionality and a media wall for photos/videos, deployed on a Raspberry Pi with tunneling for public access.

## Key Knowledge

### Technology Stack
- **Backend**: Rust + Axum 0.8 (with `multipart` feature)
- **Async Runtime**: Tokio
- **Markdown Parsing**: pulldown-cmark 0.13
- **Static Files**: tower-http 0.6 `ServeDir`
- **Deployment**: Raspberry Pi with 内网穿透 (tunneling)

### Architecture Decisions
- HTML templates use `include_str!()` for compile-time embedding
- Media files served via `ServeDir` at runtime (supports large files)
- File-based storage (no database): `logs/` for Markdown, `media/photos/` and `media/videos/` for media
- On-demand scanning: new content appears immediately without recompilation

### Critical Configuration
- **Upload limit**: 30GB (configured via `DefaultBodyLimit::max(30 * 1024 * 1024 * 1024)`)
- **Default axum multipart limit is 2MB** - must explicitly increase for large file uploads
- Server listens on `0.0.0.0:3000`

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
| `/api/logs` | POST/PUT | Create/Update log |
| `/api/logs/{slug}` | PUT/DELETE | Update/Delete log |
| `/api/media/{filename}/{type}` | PUT/DELETE | Rename/Delete media |

## Recent Actions

### Upload Functionality Fixes
- **Discovered**: PNG files not displaying → root cause was empty test files in Trash
- **Discovered**: Large file uploads failing silently → axum's default 2MB multipart body limit
- **Fixed**: Increased upload limit to 30GB with `DefaultBodyLimit` layer
- **Added**: Error page (`/upload-error`) for upload limit exceeded
- **Added**: Frontend validation with clear error messages

### Media Management Features
- **Added**: Delete button with confirmation dialog for each media item
- **Added**: Rename functionality that auto-preserves file extensions
- **Improved**: UI displays filenames without extensions (e.g., "藤原豆腐店" instead of "藤原豆腐店.jpg")
- **Optimized**: Mobile-responsive layout (2 columns on mobile, 3-4 on desktop)
- **Fixed**: `renameMedia()` parameter order bug (was passing display name as filename)

### Log Editing Features
- **Added**: `/edit-log` page for editing existing logs
- **Added**: `PUT /api/logs/{slug}` endpoint for updates
- **Created**: `assets/edit_log.html` template with mobile-friendly form
- **Fixed**: Route registration and handler exports

### Code Cleanup
- Removed debug `println!` statements from upload handler
- Removed console.log statements from frontend
- Removed unnecessary delay in upload success redirect

### Git Configuration
- Set global git user: `Leopold <1261763982@qq.com>`
- All changes committed and pushed to `github.com:Leopold80/March-s-website.git`

## Current Plan

1. [DONE] Fix large file upload issue (30GB limit)
2. [DONE] Add media delete functionality
3. [DONE] Add media rename with extension preservation
4. [DONE] Make media wall mobile-friendly
5. [DONE] Add log editing functionality
6. [DONE] Clean up debug logging
7. [TODO] Test HEIC conversion on Raspberry Pi
8. [TODO] Add pagination for media wall (if many files)
9. [TODO] Consider adding image thumbnails for faster loading

## Open Issues / Notes

- `MediaService::upload_media()` and `delete_media()` methods are now unused (dead code warning) - consider removal
- Log edit page currently requires slug in query params; could be cleaner with path parameter
- MOV video format may have compatibility issues on some browsers (MP4 preferred)
- Local IP for testing: `192.168.170.131:3000` (changes per network)

---

## Summary Metadata
**Update time**: 2026-05-19T15:11:55.315Z 
