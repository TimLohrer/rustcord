use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Discord {
    pub token: String,
    pub settings: Settings,
    pub user: User,
    pub guilds_minimal: Vec<GuildMinimal>,
    pub guilds: Vec<Guild>
}

impl Discord {
    pub fn new() -> Self {
        Discord {
            token: String::new(),
            settings: Settings::default(),
            user: User::default(),
            guilds_minimal: Vec::new(),
            guilds: Vec::new()
        }
    }
}

/*
{
    "id": "309712398317649931",
    "username": "aim_shock",
    "avatar": "174f1ca0a3685c736bee97aaaafae302",
    "discriminator": "0",
    "public_flags": 4194368,
    "premium_type": 0,
    "flags": 4194416,
    "banner": null,
    "accent_color": 2303016,
    "global_name": "Tim",
    "avatar_decoration_data": null,
    "banner_color": "#232428",
    "mfa_enabled": true,
    "locale": "en-US",
    "email": "me@timlohrer.de",
    "verified": true,
    "phone": "+491794571753",
    "nsfw_allowed": true,
    "linked_users": [],
    "purchased_flags": 3,
    "bio": "(\\u256f\\u00b0\\u25a1\\u00b0)\\u256f\\ufe35 \\u253b\\u2501\\u253b",
    "authenticator_types": [1, 2, 3]
}
*/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub discriminator: String,
    pub public_flags: i32,
    pub premium_type: i32,
    pub flags: i32,
    pub purchased_flags: Option<i32>,
    pub bot: Option<bool>,
    pub banner: Option<String>,
    pub accent_color: Option<i32>,
    pub global_name: Option<String>,
    pub avatar_decoration_data: Option<String>,
    pub banner_color: Option<String>,
    pub mfa_enabled: bool,
    pub locale: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub verified: bool,
    pub bio: String,
    pub nsfw_allowed: Option<bool>,
    pub linked_users: Option<Vec<String>>,
    pub authenticator_types: Option<Vec<i32>>
}

impl Default for User {
    fn default() -> Self {
        User {
            id: String::new(),
            username: String::new(),
            avatar: None,
            discriminator: String::new(),
            public_flags: 0,
            premium_type: 0,
            flags: 0,
            purchased_flags: Some(0),
            bot: Some(false),
            banner: None,
            accent_color: None,
            global_name: None,
            avatar_decoration_data: None,
            banner_color: None,
            mfa_enabled: false,
            locale: String::from("en-US"),
            email: None,
            phone: None,
            verified: false,
            bio: String::new(),
            nsfw_allowed: None,
            linked_users: Some(vec![]),
            authenticator_types: Some(vec![])
        }
    }
}

/*
{
    "locale": "en-US",
    "show_current_game": true,
    "restricted_guilds": [
      "938696154760228864",
      "584403238032244973",
      "231471142685245440",
      "834044558622982164",
      "1169341422919499826",
      "904704620956381205",
      "994621970152968212",
      "1019307799148245082",
      "289047350020669440",
      "777148268341493770",
      "679875946597056683",
      "1042825510994063360",
      "862301606124453910",
      "627206929823039499",
      "992824218712539326",
      "222078108977594368",
      "701742321884069999",
      "993565767041814578",
      "1126158447260479498",
      "851044488389722113",
      "823999598099431475",
      "1008696016318513243",
      "902967284560195605",
      "949840246680469504",
      "874949449091268648",
      "616186924390023171",
      "934528898144817242",
      "777536797601300490",
      "913695746002419743",
      "716379024250765363",
      "860208780670009374",
      "916400005072429117",
      "774271756549619722",
      "315497731206414338",
      "1016890939245088861"
    ],
    "default_guilds_restricted": true,
    "inline_attachment_media": true,
    "inline_embed_media": true,
    "gif_auto_play": true,
    "render_embeds": true,
    "render_reactions": true,
    "animate_emoji": true,
    "enable_tts_command": false,
    "message_display_compact": false,
    "convert_emoticons": false,
    "explicit_content_filter": 0,
    "disable_games_tab": false,
    "theme": "dark",
    "developer_mode": true,
    "detect_platform_accounts": true,
    "status": "idle",
    "afk_timeout": 60,
    "timezone_offset": -60,
    "stream_notifications_enabled": false,
    "allow_accessibility_detection": true,
    "contact_sync_enabled": true,
    "native_phone_integration_enabled": true,
    "animate_stickers": 0,
    "friend_discovery_flags": 6,
    "view_nsfw_guilds": true,
    "view_nsfw_commands": false,
    "passwordless": true,
    "friend_source_flags": {},
    "guild_folders": [
      {
        "guild_ids": [
          "1019307799148245082"
        ],
        "id": null,
        "name": null,
        "color": null
      },
      {
        "guild_ids": [
          "774271756549619722",
          "777536797601300490",
          "862301606124453910",
          "860208780670009374",
          "1169341422919499826",
          "289047350020669440",
          "716379024250765363",
          "992824218712539326"
        ],
        "id": 2977447460,
        "name": null,
        "color": null
      },
      {
        "guild_ids": [
          "994621970152968212",
          "993565767041814578",
          "701742321884069999",
          "938696154760228864",
          "627206929823039499",
          "934528898144817242",
          "584403238032244973",
          "913695746002419743",
          "916400005072429117"
        ],
        "id": 2395588322,
        "name": null,
        "color": null
      },
      {
        "guild_ids": [
          "823999598099431475",
          "851044488389722113",
          "1042825510994063360",
          "904704620956381205",
          "902967284560195605",
          "949840246680469504",
          "1126158447260479498",
          "777148268341493770"
        ],
        "id": 3564490370,
        "name": "Moderator",
        "color": null
      },
      {
        "guild_ids": [
          "679875946597056683",
          "1008696016318513243",
          "231471142685245440",
          "222078108977594368",
          "616186924390023171",
          "1016890939245088861"
        ],
        "id": 988795182,
        "name": "discord.js",
        "color": 5793266
      },
      {
        "guild_ids": [
          "834044558622982164",
          "874949449091268648"
        ],
        "id": "6090953195309516000",
        "name": null,
        "color": null
      }
    ],
    "custom_status": {
      "text": "galaxybot.app | norisk.gg",
      "expires_at": null,
      "emoji_id": null,
      "emoji_name": null
    },
    "activity_restricted_guild_ids": [],
    "activity_joining_restricted_guild_ids": [],
    "broadcast_allow_friends": false,
    "broadcast_allowed_guild_ids": [],
    "broadcast_allowed_user_ids": []
}
*/

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    pub locale: String,
    pub show_current_game: bool,
    pub restricted_guilds: Vec<String>,
    pub default_guilds_restricted: bool,
    pub inline_attachment_media: bool,
    pub inline_embed_media: bool,
    pub gif_auto_play: bool,
    pub render_embeds: bool,
    pub render_reactions: bool,
    pub animate_emoji: bool,
    pub enable_tts_command: bool,
    pub message_display_compact: bool,
    pub convert_emoticons: bool,
    pub explicit_content_filter: i32,
    pub disable_games_tab: bool,
    pub theme: String,
    pub developer_mode: bool,
    pub detect_platform_accounts: bool,
    pub status: String,
    pub afk_timeout: i32,
    pub timezone_offset: i32,
    pub stream_notifications_enabled: bool,
    pub allow_accessibility_detection: bool,
    pub contact_sync_enabled: bool,
    pub native_phone_integration_enabled: bool,
    pub animate_stickers: i32,
    pub friend_discovery_flags: i32,
    pub view_nsfw_guilds: bool,
    pub view_nsfw_commands: bool,
    pub passwordless: bool,
    pub friend_source_flags: HashMap<String, bool>,
    pub guild_folders: Vec<GuildFolder>,
    pub custom_status: CustomStatus,
    pub activity_restricted_guild_ids: Vec<String>,
    pub activity_joining_restricted_guild_ids: Vec<String>,
    pub broadcast_allow_friends: bool,
    pub broadcast_allowed_guild_ids: Vec<String>,
    pub broadcast_allowed_user_ids: Vec<String>
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            locale: String::from("en-US"),
            show_current_game: true,
            restricted_guilds: Vec::new(),
            default_guilds_restricted: false,
            inline_attachment_media: true,
            inline_embed_media: true,
            gif_auto_play: true,
            render_embeds: true,
            render_reactions: true,
            animate_emoji: true,
            enable_tts_command: false,
            message_display_compact: false,
            convert_emoticons: false,
            explicit_content_filter: 0,
            disable_games_tab: false,
            theme: String::from("dark"),
            developer_mode: true,
            detect_platform_accounts: true,
            status: String::from("online"),
            afk_timeout: 60,
            timezone_offset: -60,
            stream_notifications_enabled: false,
            allow_accessibility_detection: true,
            contact_sync_enabled: true,
            native_phone_integration_enabled: true,
            animate_stickers: 0,
            friend_discovery_flags: 6,
            view_nsfw_guilds: true,
            view_nsfw_commands: false,
            passwordless: true,
            friend_source_flags: HashMap::new(),
            guild_folders: Vec::new(),
            custom_status: CustomStatus::default(),
            activity_restricted_guild_ids: Vec::new(),
            activity_joining_restricted_guild_ids: Vec::new(),
            broadcast_allow_friends: false,
            broadcast_allowed_guild_ids: Vec::new(),
            broadcast_allowed_user_ids: Vec::new()
        }
    }
}

/*
{
    "guild_ids": [
      "834044558622982164",
      "874949449091268648"
    ],
    "id": "6090953195309516000",
    "name": null,
    "color": null
}
*/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildFolder {
  pub guild_ids: Vec<String>,
  pub id: Option<GuildFolderId>,
  pub name: Option<String>,
  pub color: Option<i32>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum GuildFolderId {
    String(String),
    Int(u32)
}

/*
{
    "text": "galaxybot.app | norisk.gg",
    "expires_at": null,
    "emoji_id": null,
    "emoji_name": null
}
*/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomStatus {
    pub text: String,
    pub expires_at: Option<String>,
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>
}

impl Default for CustomStatus {
    fn default() -> Self {
        CustomStatus {
            text: String::new(),
            expires_at: None,
            emoji_id: None,
            emoji_name: None
        }
    }
}

/*
{
    "id": "1042825510994063360",
    "name": "Tim's Developement Hub",
    "icon": "0f68113072bf6f514625690f0794a681",
    "owner": false,
    "permissions": "562949953421311",
    "features": ["NEWS","COMMUNITY"]
}
*/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildMinimal {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub owner: bool,
    pub permissions: String,
    pub features: Vec<String>
}

impl Default for GuildMinimal {
    fn default() -> Self {
        GuildMinimal {
            id: String::new(),
            name: String::new(),
            icon: None,
            owner: false,
            permissions: String::new(),
            features: Vec::new()
        }
    }
}

/*
{
  "id": "1019307799148245082",
  "name": "die hood",
  "icon": "a_c60205cc0a5b86af4f9031d4c7ff022d",
  "description": null,
  "home_header": null,
  "splash": "bf37fe8f42ff2134a80ee8985a3ee247",
  "discovery_splash": null,
  "features": [
    "SEVEN_DAY_THREAD_ARCHIVE",
    "ANIMATED_ICON",
    "PRIVATE_THREADS",
    "BANNER",
    "SOUNDBOARD",
    "MEMBER_PROFILES",
    "THREE_DAY_THREAD_ARCHIVE",
    "INVITE_SPLASH",
    "ROLE_ICONS",
    "CHANNEL_ICON_EMOJIS_GENERATED"
  ],
  "banner": "02e74de69df002e6b91c5a219dc1c523",
  "owner_id": "336516281928908801",
  "application_id": null,
  "region": "singapore",
  "afk_channel_id": null,
  "afk_timeout": 300,
  "system_channel_id": "1019307799936782510",
  "system_channel_flags": 0,
  "widget_enabled": false,
  "widget_channel_id": null,
  "verification_level": 0,
  "roles": [
    {
      "id": "1019307799148245082",
      "name": "@everyone",
      "description": null,
      "permissions": "559623605571137",
      "position": 0,
      "color": 0,
      "hoist": false,
      "managed": false,
      "mentionable": false,
      "icon": null,
      "unicode_emoji": null,
      "flags": 0
    },
    {
      "id": "1020635023445078026",
      "name": "Fortnite",
      "description": null,
      "permissions": "1071698660929",
      "position": 11,
      "color": 15844367,
      "hoist": false,
      "managed": false,
      "mentionable": false,
      "icon": null,
      "unicode_emoji": null,
      "flags": 0
    },
    {
      "id": "1038158962111029280",
      "name": "schwarz",
      "description": null,
      "permissions": "1071698660929",
      "position": 4,
      "color": 0,
      "hoist": false,
      "managed": false,
      "mentionable": false,
      "icon": null,
      "unicode_emoji": null,
      "flags": 0
    },
    {
      "id": "1077133247416385556",
      "name": "Server Booster",
      "description": null,
      "permissions": "1071698660928",
      "position": 8,
      "color": 16023551,
      "hoist": false,
      "managed": true,
      "mentionable": false,
      "icon": null,
      "unicode_emoji": null,
      "tags": {
        "premium_subscriber": null
      },
      "flags": 0
    },
    {
      "id": "1082682661585305621",
      "name": "Green-bot",
      "description": null,
      "permissions": "4331695368",
      "position": 3,
      "color": 0,
      "hoist": false,
      "managed": true,
      "mentionable": false,
      "icon": null,
      "unicode_emoji": null,
      "tags": {
        "bot_id": "783708073390112830"
      },
      "flags": 0
    },
    {
      "id": "1110955189235626099",
      "name": "Reminder",
      "description": null,
      "permissions": "8",
      "position": 1,
      "color": 0,
      "hoist": false,
      "managed": true,
      "mentionable": false,
      "icon": null,
      "unicode_emoji": null,
      "tags": {
        "bot_id": "1009793372220837918"
      },
      "flags": 0
    },
    {
      "id": "1120057277886505131",
      "name": "Valorant",
      "description": null,
      "permissions": "0",
      "position": 6,
      "color": 15158332,
      "hoist": false,
      "managed": false,
      "mentionable": false,
      "icon": null,
      "unicode_emoji": null,
      "flags": 0
    },
    {
      "id": "1180945245551202365",
      "name": "Phasmophobia",
      "description": null,
      "permissions": "31858024238657",
      "position": 7,
      "color": 3066993,
      "hoist": false,
      "managed": false,
      "mentionable": false,
      "icon": null,
      "unicode_emoji": null,
      "flags": 0
    },
    {
      "id": "1180984671002177587",
      "name": "Bot",
      "description": null,
      "permissions": "0",
      "position": 2,
      "color": 197379,
      "hoist": false,
      "managed": false,
      "mentionable": false,
      "icon": null,
      "unicode_emoji": null,
      "flags": 0
    },
    {
      "id": "1181328104619262033",
      "name": "Fleißig",
      "description": null,
      "permissions": "0",
      "position": 5,
      "color": 10038562,
      "hoist": false,
      "managed": false,
      "mentionable": false,
      "icon": null,
      "unicode_emoji": null,
      "flags": 0
    },
    {
      "id": "1194006975835537480",
      "name": "Minecraft",
      "description": null,
      "permissions": "0",
      "position": 9,
      "color": 12745742,
      "hoist": false,
      "managed": false,
      "mentionable": false,
      "icon": null,
      "unicode_emoji": null,
      "flags": 0
    },
    {
      "id": "1200798502888943667",
      "name": "K-Pop Enoyer",
      "description": null,
      "permissions": "0",
      "position": 10,
      "color": 12880839,
      "hoist": false,
      "managed": false,
      "mentionable": false,
      "icon": null,
      "unicode_emoji": null,
      "flags": 0
    }
  ],
  "default_message_notifications": 0,
  "mfa_level": 0,
  "explicit_content_filter": 0,
  "max_presences": null,
  "max_members": 500000,
  "max_stage_video_channel_users": 150,
  "max_video_channel_users": 25,
  "vanity_url_code": null,
  "premium_tier": 2,
  "premium_subscription_count": 7,
  "preferred_locale": "en-US",
  "rules_channel_id": null,
  "safety_alerts_channel_id": null,
  "public_updates_channel_id": null,
  "hub_type": null,
  "premium_progress_bar_enabled": true,
  "latest_onboarding_question_id": null,
  "nsfw": false,
  "nsfw_level": 0,
  "emojis": [
    {
      "id": "1137160245613953126",
      "name": "occtobuddy",
      "roles": [],
      "require_colons": true,
      "managed": false,
      "animated": false,
      "available": true
    },
    {
      "id": "1147651137453969428",
      "name": "Minion",
      "roles": [],
      "require_colons": true,
      "managed": false,
      "animated": false,
      "available": true
    },
    {
      "id": "1155599744123539528",
      "name": "846466811772944emoji",
      "roles": [],
      "require_colons": true,
      "managed": false,
      "animated": true,
      "available": true
    },
    {
      "id": "1155951806023749642",
      "name": "UNO",
      "roles": [],
      "require_colons": true,
      "managed": false,
      "animated": false,
      "available": true
    },
    {
      "id": "1155951874852257863",
      "name": "MaliksMood",
      "roles": [],
      "require_colons": true,
      "managed": false,
      "animated": false,
      "available": true
    },
    {
      "id": "1155952313656152114",
      "name": "Girl",
      "roles": [],
      "require_colons": true,
      "managed": false,
      "animated": false,
      "available": true
    },
    {
      "id": "1155952391737323520",
      "name": "nerd",
      "roles": [],
      "require_colons": true,
      "managed": false,
      "animated": false,
      "available": true
    },
    {
      "id": "1155954378348449892",
      "name": "Trash1",
      "roles": [],
      "require_colons": true,
      "managed": false,
      "animated": false,
      "available": true
    },
    {
      "id": "1155954389849227364",
      "name": "Trash2",
      "roles": [],
      "require_colons": true,
      "managed": false,
      "animated": false,
      "available": true
    },
    {
      "id": "1155955342727974962",
      "name": "squidildotesticles",
      "roles": [],
      "require_colons": true,
      "managed": false,
      "animated": true,
      "available": true
    },
    {
      "id": "1155956008254980197",
      "name": "smurfcat",
      "roles": [],
      "require_colons": true,
      "managed": false,
      "animated": false,
      "available": true
    },
    {
      "id": "1156637171856445533",
      "name": "MontanaBlack",
      "roles": [],
      "require_colons": true,
      "managed": false,
      "animated": false,
      "available": true
    },
    {
      "id": "1176263261356826796",
      "name": "neuronactivation",
      "roles": [],
      "require_colons": true,
      "managed": false,
      "animated": false,
      "available": true
    },
    {
      "id": "1176263998946156604",
      "name": "korner",
      "roles": [],
      "require_colons": true,
      "managed": false,
      "animated": false,
      "available": true
    },
    {
      "id": "1178771560857870476",
      "name": "real",
      "roles": [],
      "require_colons": true,
      "managed": false,
      "animated": false,
      "available": true
    }
  ],
  "stickers": [],
  "incidents_data": null,
  "inventory_settings": null,
  "embed_enabled": false,
  "embed_channel_id": null
}
 */

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub home_header: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub features: Vec<String>,
    pub banner: Option<String>,
    pub owner_id: String,
    pub application_id: Option<String>,
    pub region: String,
    pub afk_channel_id: Option<String>,
    pub afk_timeout: i32,
    pub system_channel_id: Option<String>,
    pub system_channel_flags: i32,
    pub widget_enabled: bool,
    pub widget_channel_id: Option<String>,
    pub verification_level: i32,
    pub roles: Vec<Role>,
    pub default_message_notifications: i32,
    pub mfa_level: i32,
    pub explicit_content_filter: i32,
    pub max_presences: Option<i32>,
    pub max_members: i32,
    pub max_video_channel_users: i32,
    pub vanity_url_code: Option<String>,
    pub premium_tier: i32,
    pub premium_subscription_count: i32,
    pub preferred_locale: String,
    pub rules_channel_id: Option<String>,
    pub safety_alerts_channel_id: Option<String>,
    pub public_updates_channel_id: Option<String>,
    pub hub_type: Option<String>,
    pub premium_progress_bar_enabled: bool,
    pub latest_onboarding_question_id: Option<String>,
    pub nsfw: bool,
    pub nsfw_level: i32,
    pub emojis: Vec<Emoji>,
    pub stickers: Vec<Sticker>,
    pub incidents_data: Option<HashMap<String, String>>,
    pub inventory_settings: Option<String>,
    pub embed_enabled: bool,
    pub embed_channel_id: Option<String>,
    pub channels: Option<Vec<Channel>>
}

impl Default for Guild {
  fn default() -> Self {
    Guild {
      id: String::new(),
      name: String::new(),
      icon: None,
      description: None,
      home_header: None,
      splash: None,
      discovery_splash: None,
      features: Vec::new(),
      banner: None,
      owner_id: String::new(),
      application_id: None,
      region: String::new(),
      afk_channel_id: None,
      afk_timeout: 0,
      system_channel_id: None,
      system_channel_flags: 0,
      widget_enabled: false,
      widget_channel_id: None,
      verification_level: 0,
      roles: Vec::new(),
      default_message_notifications: 0,
      mfa_level: 0,
      explicit_content_filter: 0,
      max_presences: None,
      max_members: 0,
      max_video_channel_users: 0,
      vanity_url_code: None,
      premium_tier: 0,
      premium_subscription_count: 0,
      preferred_locale: String::new(),
      rules_channel_id: None,
      safety_alerts_channel_id: None,
      public_updates_channel_id: None,
      hub_type: None,
      premium_progress_bar_enabled: false,
      latest_onboarding_question_id: None,
      nsfw: false,
      nsfw_level: 0,
      emojis: Vec::new(),
      stickers: Vec::new(),
      incidents_data: None,
      inventory_settings: None,
      embed_enabled: false,
      embed_channel_id: None,
      channels: None
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Channel {
    pub id: String,
    pub r#type: i32,
    pub name: String,
	  pub flags: i32,
    pub guild_id: String,
    pub parent_id: Option<String>,
    pub children: Option<Vec<Channel>>,
    pub position: i32,
    pub video_quality_mode: Option<i32>,
	  pub bitrate: Option<i32>,
	  pub user_limit: Option<i32>,
	  pub rtc_region: Option<String>,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub last_message_id: Option<String>,
    pub topic: Option<String>,
    pub rate_limit_per_user: Option<i32>,
    pub default_thread_rate_limit_per_user: Option<i32>,
    pub default_auto_archive_duration: Option<i32>,
	  pub available_tags: Option<Vec<AvailableTag>>,
	  pub default_reaction_emoji: Option<DefaultReactionEmoji>,
	  pub default_sort_order: Option<i32>,
	  pub default_forum_layout: Option<i32>,
	  pub icon_emoji: Option<IconEmoji>,
	  pub theme_color: Option<i32>,
	  pub template: Option<String>,
    pub voice_background_display: Option<String>,
    pub nsfw: Option<bool>
}

impl Default for Channel {
  fn default() -> Self {
    Self {
      id: String::new(),
      r#type: 0,
      name: String::new(),
      flags: 0,
      guild_id: String::new(),
      children: None,
      parent_id: None,
      position: 0,
      video_quality_mode: None,
      bitrate: None,
      user_limit: None,
      rtc_region: None,
      permission_overwrites: Vec::new(),
      last_message_id: None,
      topic: None,
      rate_limit_per_user: None,
      default_thread_rate_limit_per_user: None,
      default_auto_archive_duration: None,
      available_tags: None,
      default_reaction_emoji: None,
      default_sort_order: None,
      default_forum_layout: None,
      icon_emoji: None,
      theme_color: None,
      template: None,
      voice_background_display: None,
      nsfw: None
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IconEmoji {
  pub id: Option<String>,
  pub name: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AvailableTag {
	pub id: String,
	pub name: String,
	pub moderated: bool,
	pub emoji_id: Option<String>,
	pub emoji_name: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DefaultReactionEmoji {
	pub emoji_id: Option<String>,
	pub emoji_name: Option<String>
}

/*
{
	"id": "1210897633099382854",
	"type": 0,
	"allow": "1049600",
	"deny": "0"
}
*/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PermissionOverwrite {
	pub id: String,
	pub r#type: i32,
	pub allow: String,
	pub deny: String
}

/*
{
  	"id": "1137160245613953126",
  	"name": "occtobuddy",
  	"roles": [],
  	"require_colons": true,
  	"managed": false,
  	"animated": false,
  	"available": true
}
*/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Emoji {
    pub id: String,
    pub name: String,
    pub roles: Vec<String>,
    pub require_colons: bool,
    pub managed: bool,
    pub animated: bool,
    pub available: bool
}

/*
{
    "id": "859967607808196628",
    "name": "one gateway please",
    "tags": "pleading_face",
    "type": 2,
    "format_type": 1,
    "description": "Don't be lazy and use Intents.ALL! Only provide the intents for the events you need to receive!",
    "asset": "",
    "available": true,
    "guild_id": "222078108977594368"
}
*/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sticker {
	pub id: String,
	pub name: String,
	pub tags: String,
	pub r#type: i32,
	pub format_type: i32,
	pub description: Option<String>,
	pub asset: Option<String>,
	pub available: bool,
	pub guild_id: String
}

/*
{
  	"id": "1019307799148245082",
  	"name": "@everyone",
  	"description": null,
  	"permissions": "559623605571137",
  	"position": 0,
  	"color": 0,
  	"hoist": false,
  	"managed": false,
  	"mentionable": false,
  	"icon": null,
  	"unicode_emoji": null,
  	"flags": 0
}
*/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub permissions: String,
    pub position: i32,
    pub color: i32,
    pub hoist: bool,
    pub managed: bool,
    pub mentionable: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub tags: Option<HashMap<String, Option<String>>>,
    pub flags: i32
}