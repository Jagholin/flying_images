import { invoke } from "@tauri-apps/api";
import State from "../state/state";

export async function search_web_images(query: string, ctx: State): Promise<void> {
    console.log("execute image search, query is ", query)
    const imgs = await invoke("search_web_images", { query });
    console.log("search web images finished, result is ", imgs);
}

export async function get_csrf_token(): Promise<void> {
    const result = await invoke("get_csrf_token", {});
}

export async function test_da_request(): Promise<void> {
    await invoke("test_da_request", {});
}
