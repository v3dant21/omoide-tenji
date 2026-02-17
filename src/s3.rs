use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;

pub fn get_public_url(bucket: &str, key: &str, region: &str) -> String {
    if let Ok(endpoint) = std::env::var("AWS_ENDPOINT_URL") {
        if endpoint.contains("localhost") || endpoint.contains("localstack") {
            return format!("http://{bucket}.s3.localhost.localstack.cloud:4566/{key}");
        }
    }
    format!("https://{bucket}.s3.{region}.amazonaws.com/{key}")
}

/// Upload bytes to S3 and return the public URL.
pub async fn upload_to_s3(
    client: &Client,
    bucket: &str,
    key: &str,
    data: Vec<u8>,
    content_type: &str,
) -> Result<String, String> {
    match client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(data))
        .content_type(content_type)
        .send()
        .await
    {
        Ok(_) => {}
        Err(e) => {
            let err_msg = format!("S3 upload error for key={key}: {e:?}");
            return Err(err_msg);
        }
    }

    let region = std::env::var("AWS_REGION").unwrap_or_else(|_| "eu-north-1".to_string());
    Ok(get_public_url(bucket, key, &region))
}

/// Download an object from S3 and return its bytes.
pub async fn download_from_s3(
    client: &Client,
    bucket: &str,
    key: &str,
) -> Result<Vec<u8>, String> {
    let resp = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|e| format!("S3 download error: {e}"))?;

    let bytes = resp
        .body
        .collect()
        .await
        .map_err(|e| format!("S3 body read error: {e}"))?
        .into_bytes()
        .to_vec();

    Ok(bytes)
}

/// List all object keys under `galleries/<gallery_id>/`.
pub async fn list_gallery_images(
    client: &Client,
    bucket: &str,
    gallery_id: &str,
) -> Result<Vec<String>, String> {
    let prefix = format!("galleries/{gallery_id}/");

    let resp = client
        .list_objects_v2()
        .bucket(bucket)
        .prefix(&prefix)
        .send()
        .await
        .map_err(|e| format!("S3 list error: {e}"))?;

    let keys: Vec<String> = resp
        .contents()
        .iter()
        .filter_map(|obj| obj.key().map(|k| k.to_string()))
        .collect();

    Ok(keys)
}
