/*
{
    "id": "1211326265043320872",
    "type": 19,
    "content": "<@274937240109711361> <@535445810146574336> 💀",
    "channel_id": "1114555266835431444",
    "author": {
      "id": "309712398317649931",
      "username": "aim_shock",
      "avatar": "174f1ca0a3685c736bee97aaaafae302",
      "discriminator": "0",
      "public_flags": 4194368,
      "premium_type": 0,
      "flags": 4194368,
      "banner": null,
      "accent_color": null,
      "global_name": "Tim",
      "avatar_decoration_data": null,
      "banner_color": null
    },
    "attachments": [],
    "embeds": [],
    "mentions": [
      {
        "id": "535445810146574336",
        "username": "tomjuri",
        "avatar": "d25b7ab11c67b5223edaa40b6ea7cf12",
        "discriminator": "0",
        "public_flags": 64,
        "premium_type": 0,
        "flags": 64,
        "banner": null,
        "accent_color": null,
        "global_name": "Tom",
        "avatar_decoration_data": null,
        "banner_color": null
      },
      {
        "id": "274937240109711361",
        "username": "norisk",
        "avatar": "e13b91a7dbecd6852f96d3d47068556c",
        "discriminator": "0",
        "public_flags": 0,
        "premium_type": 2,
        "flags": 0,
        "banner": null,
        "accent_color": null,
        "global_name": "NoRisk",
        "avatar_decoration_data": null,
        "banner_color": null
      },
      {
        "id": "1096124677434376233",
        "username": "survivalmodee",
        "avatar": "d70e99210a2c2d8f16169ba46c3ba243",
        "discriminator": "0",
        "public_flags": 4194368,
        "premium_type": 0,
        "flags": 4194368,
        "banner": null,
        "accent_color": null,
        "global_name": "SurvivalMode",
        "avatar_decoration_data": null,
        "banner_color": null
      }
    ],
    "mention_roles": [],
    "pinned": false,
    "mention_everyone": false,
    "tts": false,
    "timestamp": "2024-02-25T14:58:06.940000+00:00",
    "edited_timestamp": null,
    "flags": 0,
    "components": [],
    "message_reference": {
      "channel_id": "1114555266835431444",
      "message_id": "1211324992483106897",
      "guild_id": "774271756549619722"
    },
    "referenced_message": {
      "id": "1211324992483106897",
      "type": 0,
      "content": "was ist das?",
      "channel_id": "1114555266835431444",
      "author": {
        "id": "1096124677434376233",
        "username": "survivalmodee",
        "avatar": "d70e99210a2c2d8f16169ba46c3ba243",
        "discriminator": "0",
        "public_flags": 4194368,
        "premium_type": 0,
        "flags": 4194368,
        "banner": null,
        "accent_color": null,
        "global_name": "SurvivalMode",
        "avatar_decoration_data": null,
        "banner_color": null
      },
      "attachments": [
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
      ],
      "embeds": [],
      "mentions": [],
      "mention_roles": [],
      "pinned": false,
      "mention_everyone": false,
      "tts": false,
      "timestamp": "2024-02-25T14:53:03.538000+00:00",
      "edited_timestamp": null,
      "flags": 0,
      "components": []
    }
  }
 */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub r#type: i64,
    pub content: String,
    pub channel_id: String,
    pub author: User,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub mentions: Vec<User>,
    pub mention_roles: Vec<String>,
    pub pinned: bool,
    pub mention_everyone: bool,
    pub tts: bool,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub flags: i64,
    pub components: Vec<serde_json::Value>,
    pub message_reference: Option<MessageReference>,
    pub referenced_message: Option<Box<Message>>,
}

/*
{
{
    "id": "1211324992483106897",
    "type": 0,
    "content": "was ist das?",
    "channel_id": "1114555266835431444",
    "author": {
        "id": "1096124677434376233",
        "username": "survivalmodee",
        "avatar": "d70e99210a2c2d8f16169ba46c3ba243",
        "discriminator": "0",
        "public_flags": 4194368,
        "premium_type": 0,
        "flags": 4194368,
        "banner": null,
        "accent_color": null,
        "global_name": "SurvivalMode",
        "avatar_decoration_data": null,
        "banner_color": null
    },
    "attachments": [
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
    ],
    "embeds": [],
    "mentions": [],
    "mention_roles": [],
    "pinned": false,
    "mention_everyone": false,
    "tts": false,
    "timestamp": "2024-02-25T14:53:03.538000+00:00",
    "edited_timestamp": null,
    "flags": 0,
    "components": []
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReference {
    id: String,
    r#type: i32,
    content: String,
    channel_id: String,
    author: User,
    attachments: Vec<Attachment>,
    embeds: Vec<Embed>,
    mentions: Vec<User>,
    mention_roles: Vec<Role>,
    
}