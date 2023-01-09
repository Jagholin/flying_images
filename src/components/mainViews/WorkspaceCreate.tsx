import {
  styled,
} from "@mui/material";
import Button from "@mui/material/Button";
import TextField from "@mui/material/TextField";
import Typography from "@mui/material/Typography";
import { Box } from "@mui/system";
import { observer } from "mobx-react-lite";
import { ChangeEvent, FormEvent, useContext, useState } from "react";
import { StateContext } from "../../state/context";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";
import { createWorkspace, openWorkspace } from "../../commands/workspaceCommands";
import { useNavigate } from "react-router-dom";

const CenteredBox = styled(Box)({
  display: "grid",
  alignContent: "center",
  justifyContent: "center",
  height: "100%",
});

const RowBox = styled(Box)({
  display: "flex",
  alignItems: "baseline",
});

function WorkspaceCreate() {
  const [dir, setDir] = useState<string>("");
  const [directoryValid, setDirectoryValid] = useState(true);
  const [workspaceName, setWorkspaceName] = useState("");
  const [errorState, setErrorState] = useState("");
  const context = useContext(StateContext);
  const navigate = useNavigate();
  console.log("you opened workspace create");

  const handleDirectoryChange = async (e: ChangeEvent<HTMLInputElement>) => {
    const dirPath = e.target.value;
    setDir(dirPath);
    const dirExists = await invoke('check_directory', {dir: dirPath});
    setDirectoryValid(!!dirExists);
  };

  const handleSelectDirectory = async () => {
    const dirPath = await open({
      directory: true,
      title: "Choose workspace folder",
    });
    if (!dirPath) {
      return;
    }
    // dirPath is string, since open() was configured to return a single directory
    setDirectoryValid(true);
    setDir(dirPath as string);
  };

  const handleClearDirectory = () => {
    setDir("");
  };

  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    // try to create the workspace
    createWorkspace(dir, workspaceName, context).then(() => {
      // workspace successfully created, now we can navigate to the workspace index
      navigate("/workspace", {replace: true});
    }).catch((e) => {
      console.error("error caught: ", e);
    });
  }

  const handleSubmitLoad = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    openWorkspace(dir, context).then(() => {
      navigate("/workspace", {replace: true});
    }).catch((e) => {
      console.error("error caught: ", e);
    })
  }

  return (
    <CenteredBox>
      <Box>
        <Typography variant="h2">Load workspace from</Typography>
        <Box component="form" onSubmit={handleSubmitLoad}>
          <RowBox>
            <TextField
              variant="standard"
              label="workspace folder"
              size="small"
              fullWidth={true}
              value={dir}
              onChange={handleDirectoryChange}
              error={!directoryValid}
              helperText={directoryValid ? "" : "enter existing directory"}
              required
            />
            <Button onClick={handleSelectDirectory}>Browse</Button>
            <Button onClick={handleClearDirectory}>Clear</Button>
          </RowBox>
          <RowBox justifyContent={"center"}>
            <Button type="submit" sx={{ mt: 2 }}>
              Load workspace
            </Button>
          </RowBox>
        </Box>
        <Typography variant="h2">Create new workspace</Typography>
        <Box
          component="form"
          onSubmit={handleSubmit}
        >
          <RowBox>
            <TextField
              variant="standard"
              label="workspace folder"
              size="small"
              fullWidth={true}
              value={dir}
              onChange={handleDirectoryChange}
              error={!directoryValid}
              helperText={directoryValid ? "" : "enter existing directory"}
              required
            />
            <Button onClick={handleSelectDirectory}>Browse</Button>
            <Button onClick={handleClearDirectory}>Clear</Button>
          </RowBox>
          <TextField
            variant="standard"
            fullWidth={true}
            label="workspace name"
            inputProps={{pattern: `[a-zA-Z0-9 ]+`}}
            value={workspaceName}
            onChange={(e) => setWorkspaceName(e.target.value)}
            required
          />
          <RowBox justifyContent={"center"}>
            <Button type="submit" sx={{ mt: 2 }}>
              Create workspace
            </Button>
          </RowBox>
        </Box>
      </Box>
    </CenteredBox>
  );
}

export default observer(WorkspaceCreate);
