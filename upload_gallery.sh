#!/bin/bash
set -e

SERVER_URL="http://localhost:5000"
IMAGE_DIR="$1"

if [ -z "$IMAGE_DIR" ]; then
    echo "Usage: $0 <directory_with_images>"
    exit 1
fi

# 1. Create a new gallery
echo "Creating new gallery..."
RESPONSE=$(curl -s -X POST "$SERVER_URL/api/gallery")
GALLERY_ID=$(echo "$RESPONSE" | grep -o '"gallery_id":"[^"]*"' | cut -d'"' -f4)

if [ -z "$GALLERY_ID" ]; then
    echo "Failed to create gallery. Response: $RESPONSE"
    exit 1
fi

echo "Created Gallery ID: $GALLERY_ID"

# 2. Construct curl arguments for images
CURL_ARGS=""
count=0
for img in "$IMAGE_DIR"/*.{jpg,jpeg,png,gif,webp}; do
    if [ -e "$img" ]; then
        echo "Found image: $img"
        CURL_ARGS="$CURL_ARGS -F image=@$img"
        count=$((count + 1))
    fi
done

if [ "$count" -eq 0 ]; then
    echo "No images found in $IMAGE_DIR"
    exit 1
fi

# 3. Upload images
echo "Uploading $count images to gallery $GALLERY_ID..."
curl -s $CURL_ARGS "$SERVER_URL/api/gallery/$GALLERY_ID/upload"

echo -e "\nUpload complete!"
echo "Gallery ID: $GALLERY_ID"
echo "To view, open: /g/$GALLERY_ID on your server URL"
