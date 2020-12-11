use std::sync::Arc;

use bytes::{Bytes, BytesMut};
use rusoto_core::Region;
use rusoto_s3::{GetObjectRequest, S3, S3Client};
use serenity::futures::TryStreamExt;
use serenity::prelude::TypeMapKey;

pub struct AwsS3Client {
    #[allow(dead_code)]
    region: Region,
    s3: S3Client,
    bucket_name: String,
}

impl TypeMapKey for AwsS3Client {
    type Value = Arc<AwsS3Client>;
}

impl AwsS3Client {
    pub fn new() -> AwsS3Client {
        let region = Region::default();

        AwsS3Client {
            region: region.to_owned(),
            s3: S3Client::new(region),
            bucket_name: std::env::var("AWS_S3_BUCKET_NAME").unwrap(),
        }
    }

    pub async fn get_object(&self, key: &str) -> Result<Bytes, String> {
        let get_object = GetObjectRequest {
            bucket: self.bucket_name.to_owned(),
            key: key.to_string(),
            ..Default::default()
        };
        let res = self
            .s3
            .get_object(get_object)
            .await
            .map_err(|e| e.to_string())?;

        let stream = res.body.ok_or("Error on getting object from S3")?;
        let body = stream.map_ok(|b| BytesMut::from(&b[..]))
            .try_concat()
            .await
            .map_err(|e| e.to_string())?
            .freeze();

        Ok(body)
    }
}