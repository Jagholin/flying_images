import { invoke } from "@tauri-apps/api";
import State from "../state/state";
import Workspace, { WorkspaceDescription } from "../state/workspace";

export async function checkDirectory(dirPath: string): Promise<boolean> {
    const result = await invoke("check_directory", {dir: dirPath}) as boolean;
    return result;
}

export async function createWorkspace(dirPath: string, name: string, ctx: State): Promise<void> {
    const result = await invoke("create_workspace", {dir: dirPath, name}) as WorkspaceDescription;

    ctx.setCurrentWorkspace(new Workspace(result.path, result.name));

    return;
}

export async function openWorkspace(dirPath: string, ctx: State): Promise<void> {
    const result = await invoke("open_workspace", {dir: dirPath}) as WorkspaceDescription;

    ctx.setCurrentWorkspace(new Workspace(result.path, result.name));
}
