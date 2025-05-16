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