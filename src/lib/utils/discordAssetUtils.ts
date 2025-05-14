export class DiscordAssetUtils {
    static getUserAvatarUrl(userId: string, avatarHash: string, size: number = 128, animated: boolean = false): string {
        return `https://cdn.discordapp.com/avatars/${userId}/${avatarHash}.webp?size=${size}&animated=${animated}`;
    }

    static getGuildIconUrl(guildId: string, iconHash: string, size: number = 128, animated: boolean = false): string {
        return `https://cdn.discordapp.com/icons/${guildId}/${iconHash}.webp?size=${size}&animated=${animated}`;
    }

    static getEmojiUrl(emojiId: string, size: number = 128, animated: boolean = false): string {
        return `https://cdn.discordapp.com/emojis/${emojiId}.webp?size=${size}&animated=${animated}`;
    }
}