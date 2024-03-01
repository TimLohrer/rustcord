/*
{
    "id": "1211324992441032805",
    "filename": "image.png",
    "size": 20970,
    "url": "https://cdn.discordapp.com/attachments/1114555266835431444/1211324992441032805/image.png?ex=65edc94f&is=65db544f&hm=43603959f9cdd8acc807184be4a0286b2381dd862c25179e8e31cc797f5d986e&",
    "proxy_url": "https://media.discordapp.net/attachments/1114555266835431444/1211324992441032805/image.png?ex=65edc94f&is=65db544f&hm=43603959f9cdd8acc807184be4a0286b2381dd862c25179e8e31cc797f5d986e&",
    "width": 490,
    "height": 391,
    "content_type": "image/png",
    "content_scan_version": 1,
    "placeholder": "+vcFBoBViIhahoeGh/eIaHl6mKwAQ5U=",
    "placeholder_version": 1
}
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attachment {
    pub id: String,
    pub filename: String,
    pub size: u64,
    pub url: String,
    pub proxy_url: String,
    pub width: u64,
    pub height: u64,
    pub content_type: String,
    pub content_scan_version: u64,
    pub placeholder: String,
    pub placeholder_version: u64,
}