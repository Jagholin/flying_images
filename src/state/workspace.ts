import { computed, makeObservable, observable } from "mobx";

/**
 * This class represents a workspace where most all other data is located and managed
 * on disk, the workspace corresponds to a directory with metadata.json in the root and stored images inside
 * 
 */
export default class Workspace {
    /** location of the workspace on disk */
    _fsLocation: string; 

    constructor(location: string) {
        this._fsLocation = location;
        makeObservable(this, {
            location: computed,
            _fsLocation: observable,
        });
    }

    get location(): string {
        return this._fsLocation;
    }
}
