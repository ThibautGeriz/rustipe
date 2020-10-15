use crate::domain::recipes::ports::image_store::ImageStore;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use std::env;
use std::error::Error;
use uuid::Uuid;

pub struct S3ImageStore {
    region: String,
    bucket_name: String,
}

impl ImageStore for S3ImageStore {
    fn get_photo_upload_url(&self, extension: &str) -> Result<String, Box<dyn Error>> {
        let bucket = self.get_bucket()?;
        let uuid = Uuid::new_v4().to_hyphenated();
        let path = format!("/{name}.{extention}", name = uuid, extention = extension);
        print!("extension: {:?}", extension);
        let expiry_secs = 60 * 60;
        let url = bucket
            .presign_put(path, expiry_secs, Some(reqwest::header::HeaderMap::new()))
            .map_err(Box::new)?;
        Ok(url)
    }
}

impl S3ImageStore {
    pub fn new(region: String, bucket_name: String) -> Self {
        S3ImageStore {
            region,
            bucket_name,
        }
    }

    fn get_bucket(&self) -> Result<Bucket, Box<dyn Error>> {
        let region = self.region.parse().map_err(Box::new)?;
        let credentials = Credentials::default().map_err(Box::new)?;
        let bucket = Bucket::new(&self.bucket_name, region, credentials).map_err(Box::new)?;
        Ok(bucket)
    }
}

impl Default for S3ImageStore {
    fn default() -> Self {
        let region = match env::var("REGION") {
            Ok(region) => region,
            Err(_e) => String::from("eu-west-3"),
        };
        let bucket_name = match env::var("BUCKET_NAME") {
            Ok(bucket_name) => bucket_name,
            Err(_e) => String::from("rustipe-photos"),
        };
        S3ImageStore::new(region, bucket_name)
    }
}
