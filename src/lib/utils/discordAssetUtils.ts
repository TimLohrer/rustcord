/**
CDN Endpoints
Type	Path	Supports
Custom Emoji	emojis/emoji_id.png	PNG, JPEG, WebP, GIF
Guild Icon	icons/guild_id/guild_icon.png *	PNG, JPEG, WebP, GIF
Guild Splash	splashes/guild_id/guild_splash.png	PNG, JPEG, WebP
Guild Discovery Splash	discovery-splashes/guild_id/guild_discovery_splash.png	PNG, JPEG, WebP
Guild Banner	banners/guild_id/guild_banner.png *	PNG, JPEG, WebP, GIF
User Banner	banners/user_id/user_banner.png *	PNG, JPEG, WebP, GIF
Default User Avatar	embed/avatars/index.png ** ***	PNG
User Avatar	avatars/user_id/user_avatar.png *	PNG, JPEG, WebP, GIF
Guild Member Avatar	guilds/guild_id/users/user_id/avatars/member_avatar.png *	PNG, JPEG, WebP, GIF
Avatar Decoration	avatar-decoration-presets/avatar_decoration_data_asset.png	PNG
Application Icon	app-icons/application_id/icon.png	PNG, JPEG, WebP
Application Cover	app-icons/application_id/cover_image.png	PNG, JPEG, WebP
Application Asset	app-assets/application_id/asset_id.png	PNG, JPEG, WebP
Achievement Icon	app-assets/application_id/achievements/achievement_id/icons/icon_hash.png	PNG, JPEG, WebP
Store Page Asset	app-assets/application_id/store/asset_id	PNG, JPEG, WebP
Sticker Pack Banner	app-assets/710982414301790216/store/sticker_pack_banner_asset_id.png	PNG, JPEG, WebP
Team Icon	team-icons/team_id/team_icon.png	PNG, JPEG, WebP
Sticker	stickers/sticker_id.png *** ****	PNG, Lottie, GIF
Role Icon	role-icons/role_id/role_icon.png	PNG, JPEG, WebP
Guild Scheduled Event Cover	guild-events/scheduled_event_id/scheduled_event_cover_image.png	PNG, JPEG, WebP
Guild Member Banner	guilds/guild_id/users/user_id/banners/member_banner.png *	PNG, JPEG, WebP, GIF
 */

export class DiscordAssetUtils {
    static getUserAvatarUrl(userId: string, avatarHash: string, size: number = 128, animated: boolean = false, discriminator: string = ''): string {
        if (!avatarHash && discriminator) {
            return this.getDefaultAvatarUrl(discriminator);
        }
        return `https://cdn.discordapp.com/avatars/${userId}/${avatarHash}.webp?size=${size}&animated=${animated}`;
    }

    static getDefaultAvatarUrl(discriminator: string): string {
        return `https://cdn.discordapp.com/embed/avatars/${parseInt(discriminator) % 5}.png`;
    }

    static getGuildIconUrl(guildId: string, iconHash: string, size: number = 128, animated: boolean = false): string {
        return `https://cdn.discordapp.com/icons/${guildId}/${iconHash}.webp?size=${size}&animated=${animated}`;
    }

    static getGuildBannerUrl(guildId: string, bannerHash: string, size: number = 128, animated: boolean = false): string {
        return `https://cdn.discordapp.com/banners/${guildId}/${bannerHash}.webp?size=${size}&animated=${animated}`;
    }

    static getChannelIconUrl(channelId: string, iconHash: string, size: number = 128, animated: boolean = false): string {
        return `https://cdn.discordapp.com/channel-icons/${channelId}/${iconHash}.webp?size=${size}&animated=${animated}`;
    }

    static getEmojiUrl(emojiId: string, size: number = 128, animated: boolean = false): string {
        return `https://cdn.discordapp.com/emojis/${emojiId}.webp?size=${size}&animated=${animated}`;
    }
}