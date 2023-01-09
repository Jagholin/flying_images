use serde::{Deserialize, Serialize};
use std::fs::{create_dir, File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

/// serializable struct that contains data
/// that can be used to recreate Workspace instance
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceData {
    name: String,
}

impl WorkspaceData {
    pub fn new(name: impl ToString) -> Self {
        WorkspaceData {
            name: name.to_string(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct WorkspaceUI {
    name: String,
    path: String,
}

#[derive(Debug)]
pub struct Workspace {
    name: String,
    path: PathBuf,
}

impl Workspace {
    fn from_data(data: WorkspaceData, dir: PathBuf) -> Self {
        Workspace {
            name: data.name,
            path: dir,
        }
    }

    fn to_data(&self) -> WorkspaceData {
        WorkspaceData {
            name: self.name.clone(),
        }
    }

    pub fn to_ui_data(&self) -> WorkspaceUI {
        WorkspaceUI { name: self.name.clone(), path: self.path.to_str().unwrap().to_string() }
    }

    pub fn create_workspace(name: impl ToString, path: impl ToString) -> Result<Self, String> {
        let mut work_path: PathBuf = path.to_string().into();

        // let's try to create a new directory in `path` with name `name`
        work_path.push(name.to_string());
        
        let result = Workspace {
            name: name.to_string(),
            path: work_path.clone(),
        };
        create_dir(&work_path).map_err(|e| e.to_string())?;
        let mut meta_path = work_path;
        meta_path.push("workspace.json");
        let file = File::create(&meta_path).map_err(|e| e.to_string())?;
        serde_json::to_writer(BufWriter::new(file), &result.to_data())
            .map_err(|e| e.to_string())?;

        Ok(result)
    }

    pub fn load_workspace(path: impl ToString) -> Result<Self, String> {
        let mut work_path: PathBuf = path.to_string().into();
        work_path.push("workspace.json");
        let file = File::open(&work_path).map_err(|e| e.to_string())?;
        let data: WorkspaceData =
            serde_json::from_reader(BufReader::new(file)).map_err(|e| e.to_string())?;
        let result = Self::from_data(data, path.to_string().into());
        Ok(result)
    }
}
