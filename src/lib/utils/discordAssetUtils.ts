export class DiscordAssetUtils {
    static getUserAvatarUrl(userId: string, avatarHash: string, size: number = 128): string {
        return `https://cdn.discordapp.com/avatars/${userId}/${avatarHash}.png?size=${size}`;
    }

    static getGuildIconUrl(guildId: string, iconHash: string, size: number = 128): string {
        return `https://cdn.discordapp.com/icons/${guildId}/${iconHash}.png?size=${size}`;
    }
}