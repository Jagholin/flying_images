import { ImageSearchResult } from './../commands/imageSearchCommands';
import { makeAutoObservable } from 'mobx';
import Workspace from "./workspace";

const SEARCH_PAGE_LENGTH = 20 as const;

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

    getSearchResultPage(pageNumber: number): ImageSearchResult[] {
        if (!this.searchResults) {
            return [];
        }
        const startIndex = pageNumber * SEARCH_PAGE_LENGTH;
        if (this.searchResults.length < startIndex + SEARCH_PAGE_LENGTH) {
            // we will need to retrieve the search results for this page
            // from the backend
            setTimeout(() => {
                
            })
        } 
    }
}