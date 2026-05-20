mod home;
mod logs;
mod media;
mod upload;
mod compression;

pub use home::hello_page;
pub use logs::{logs_page, log_post_page, edit_log_page, update_log, delete_log};
pub use media::{media_page, edit_media_page, delete_media, rename_media, get_thumbnail, get_video_poster, view_media_page};
pub use upload::{write_log_page, upload_media_page, upload_error_page, upload_media, create_log};
pub use compression::{compress_videos, get_compress_progress};
