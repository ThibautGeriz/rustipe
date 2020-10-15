use std::error::Error;

pub trait ImageStore {
    fn get_photo_upload_url(&self, extension: &str) -> Result<String, Box<dyn Error>>;
}
