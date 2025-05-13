import { ACTIVE_USER, TOKEN } from "$lib/stores/stateStore";
import { invoke } from "@tauri-apps/api/core";

export async function loadDebugToken(): Promise<void> {
    try {
        const token: string = await invoke("get_debug_token");
        TOKEN.set(token);
    } catch (error) {
        console.error("Error loading debug token:", error);
    }
}

export async function login(email: string, password: string): Promise<void> {
    throw new Error("Login function is not implemented yet!");
    try {
        const user: object = await invoke("login", { email, password });
        if (user) {
            ACTIVE_USER.set(user);
            // TOKEN.set(); TODO: retrive this later when we implement this function!
        }
    } catch (error) {
        console.error("Error during login:", error);
    }
}

export async function token_login(token: string): Promise<void> {
    try {
        const user = await invoke("discord_token_login", { token });
        ACTIVE_USER.set(user);
        TOKEN.set(token);
        console.log("User logged in with token:", user);
    } catch (error) {
        console.error("Error during token login:", error);
    }
}