export class PermissionUtils {
    static hasPermission(userPermissions: number, permission: number): boolean {
        return (userPermissions & permission) === permission;
    }

    static hasAnyPermission(userPermissions: number, permissions: number[]): boolean {
        return permissions.some(permission => this.hasPermission(userPermissions, permission));
    }

    static hasAllPermissions(userPermissions: number, permissions: number[]): boolean {
        return permissions.every(permission => this.hasPermission(userPermissions, permission));
    }
}

export class PermissionFlags {
    static readonly CREATE_INSTANT_INVITE = 1 << 0;
    static readonly KICK_MEMBERS = 1 << 1;
    static readonly BAN_MEMBERS = 1 << 2;
    static readonly ADMINISTRATOR = 1 << 3;
    static readonly MANAGE_CHANNELS = 1 << 4;
    static readonly MANAGE_GUILD = 1 << 5;
    static readonly ADD_REACTIONS = 1 << 6;
    static readonly VIEW_AUDIT_LOG = 1 << 7;
    static readonly PRIORITY_SPEAKER = 1 << 8;
    static readonly STREAM = 1 << 9;
    static readonly VIEW_CHANNEL = 1 << 10;
    static readonly SEND_MESSAGES = 1 << 11;
    static readonly SEND_TTS_MESSAGES = 1 << 12;
    static readonly MANAGE_MESSAGES = 1 << 13;
    static readonly EMBED_LINKS = 1 << 14;
    static readonly ATTACH_FILES = 1 << 15;
    static readonly READ_MESSAGE_HISTORY = 1 << 16;
    static readonly MENTION_EVERYONE = 1 << 17;
    static readonly USE_EXTERNAL_EMOJIS = 1 << 18;
    static readonly VIEW_GUILD_INSIGHTS = 1 << 19;
    static readonly CONNECT = 1 << 20;
    static readonly SPEAK = 1 << 21;
    static readonly MUTE_MEMBERS = 1 << 22;
    static readonly DEAFEN_MEMBERS = 1 << 23;
    static readonly MOVE_MEMBERS = 1 << 24;
    static readonly USE_VAD = 1 << 25;
    static readonly CHANGE_NICKNAME = 1 << 26;
    static readonly MANAGE_NICKNAMES = 1 << 27;
    static readonly MANAGE_ROLES = 1 << 28;
    static readonly MANAGE_WEBHOOKS = 1 << 29;
    static readonly MANAGE_EMOJIS_AND_STICKERS = 1 << 30;
    static readonly USE_APPLICATION_COMMANDS = 1 << 31;
    static readonly REQUEST_TO_SPEAK = 1 << 32;
    static readonly MANAGE_EVENTS = 1 << 33;
    static readonly MANAGE_THREADS = 1 << 34;
    static readonly CREATE_PUBLIC_THREADS = 1 << 35;
    static readonly CREATE_PRIVATE_THREADS = 1 << 36;
    static readonly USE_EXTERNAL_STICKERS = 1 << 37;
    static readonly SEND_MESSAGES_IN_THREADS = 1 << 38;
    static readonly START_EMBEDDED_ACTIVITIES = 1 << 39;
    static readonly MODERATE_MEMBERS = 1 << 40;
    static readonly VIEW_CREATOR_MONETIZATION_ANALYTICS = 1 << 41;
    static readonly USE_SOUNDBOARD = 1 << 42;
    static readonly CREATE_GUILD_EXPRESSIONS = 1 << 43;
    static readonly CREATE_EVENTS = 1 << 44;
    static readonly USE_EXTERNAL_SOUNDS = 1 << 45;
    static readonly SEND_VOICE_MESSAGES = 1 << 46;
    static readonly SEND_POLLS = 1 << 49;
    static readonly USE_EXTERNAL_APPS = 1 << 50;
}