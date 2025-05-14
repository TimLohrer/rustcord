export const DISCORD_EPOCH = 1420070400000n; // Discord epoch in milliseconds

export class SnowflakeUtils {
    static snowflakeToDate(snowflake: string): Date {
        const snowflakeBigInt = BigInt(snowflake);
        const timestamp = (snowflakeBigInt >> 22n) + DISCORD_EPOCH; // Extract timestamp from snowflake
        return new Date(Number(timestamp));
    }
}