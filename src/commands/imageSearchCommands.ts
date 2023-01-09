import { invoke } from "@tauri-apps/api";
import State from "../state/state";

export async function search_web_images(query: string, ctx: State): Promise<void> {
    const imgs = await invoke("search_web_images", { query });
}

export async function get_csrf_token(): Promise<void> {
    const result = await invoke("get_csrf_token", {});
}
