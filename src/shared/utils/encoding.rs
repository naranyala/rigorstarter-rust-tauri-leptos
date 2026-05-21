use base64::Engine;

pub fn to_image_data_uri(bytes: &[u8], extension: &str) -> String {
    let mime_type = match extension.to_lowercase().as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        _ => "image/jpeg",
    };
    let base64_data = base64::engine::general_purpose::STANDARD.encode(bytes);
    format!("data:{};base64,{}", mime_type, base64_data)
}
