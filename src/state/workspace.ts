import { computed, makeObservable, observable } from "mobx";

export interface WorkspaceDescription {
    name: string;
    path: string;
}

/**
 * This class represents a workspace where most all other data is located and managed
 * on disk, the workspace corresponds to a directory with metadata.json in the root and stored images inside
 * 
 */
export default class Workspace {
    /** location of the workspace on disk */
    _fsLocation: string;
    name: string;

    constructor(location: string, name: string) {
        this._fsLocation = location;
        this.name = name;
        makeObservable(this, {
            location: computed,
            _fsLocation: observable,
            name: observable,
        });
    }

    get location(): string {
        return this._fsLocation;
    }
}
