import { invoke } from "@tauri-apps/api";
import State from "../state/state";

export interface ImageSearchResult {
    id: string,
    img_url: string,
    title: string,
    author_id: string,
    author_name: string,
    preview_url: string,
    mature_content: boolean,
}

export async function search_web_images(query: string, ctx: State): Promise<ImageSearchResult[]> {
    console.log("execute image search, query is ", query)
    const imgs = await invoke("search_web_images", { query }) as ImageSearchResult[];
    console.log("search web images finished, result is ", imgs);
    ctx.setSearchResults(imgs);
    return imgs;
}

export async function get_search_page(pageNumber: number, ctx: State) {
    console.log(`retrieving page ${pageNumber} from the last search...`);
    const result = await invoke("get_search_page", { page_number: pageNumber });
}

export async function get_csrf_token(): Promise<void> {
    const result = await invoke("get_csrf_token", {});
}

export async function test_da_request(): Promise<void> {
    await invoke("test_da_request", {});
}
