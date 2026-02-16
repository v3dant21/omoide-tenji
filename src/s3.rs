use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;

/// Upload bytes to S3 and return the public URL.
pub async fn upload_to_s3(
    client: &Client,
    bucket: &str,
    key: &str,
    data: Vec<u8>,
    content_type: &str,
) -> Result<String, String> {
    println!("  🚀 S3 upload starting: bucket={bucket}, key={key}, size={} bytes, content_type={content_type}", data.len());

    match client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(data))
        .content_type(content_type)
        .send()
        .await
    {
        Ok(_) => {
            println!("  ✅ S3 upload succeeded: {key}");
        }
        Err(e) => {
            let err_msg = format!("S3 upload error for key={key}: {e:?}");
            println!("  ❌ {err_msg}");
            return Err(err_msg);
        }
    }

    let url = format!("https://{bucket}.s3.{}.amazonaws.com/{key}", std::env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string()));
    Ok(url)
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
