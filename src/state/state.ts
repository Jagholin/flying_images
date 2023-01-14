import { ImageSearchResult } from './../commands/imageSearchCommands';
import { makeAutoObservable } from 'mobx';
import Workspace from "./workspace";

/**
 * Describes (part of) the state of the application that is 
 * required to display the UI
 */
export default class State {
    /** current workspace that the user has opened */
    currentWorkspace?: Workspace;

    /** search results that are currently being shown in the UI */
    searchResults?: ImageSearchResult[];

    constructor() {
        makeAutoObservable(this);
    }

    /**
     * sets the current workspace, registering it as "opened"
     * @param ws workspace to be opened
     */
    setCurrentWorkspace(ws: Workspace) {
        this.currentWorkspace = ws;
    }

    /**
     * sets the currently visible search results
     * @param res search results that we received from the backend
     */
    setSearchResults(res: ImageSearchResult[]) {
        this.searchResults = res;
    }
}