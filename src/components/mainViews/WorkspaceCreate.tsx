import {
  Autocomplete,
  AutocompleteChangeDetails,
  AutocompleteChangeReason,
  styled,
} from "@mui/material";
import Button from "@mui/material/Button";
import TextField from "@mui/material/TextField";
import Typography from "@mui/material/Typography";
import { Box } from "@mui/system";
import { observer } from "mobx-react-lite";
import React, { ChangeEvent, useContext, useRef, useState } from "react";
import { Form } from "react-router-dom";
import { StateContext } from "../../state/context";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";

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
  const context = useContext(StateContext);
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

  return (
    <CenteredBox>
      <Box>
        <Typography variant="h2">Create new workspace</Typography>
        <Box
          component="form"
          onSubmit={(d) => {
            console.log("submit clicked, data ", d);
            d.preventDefault();
          }}
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
            required
          />
          <Button type="submit" sx={{ mt: 2, float: "right" }}>
            Create workspace
          </Button>
        </Box>
      </Box>
    </CenteredBox>
  );
}

export default observer(WorkspaceCreate);
