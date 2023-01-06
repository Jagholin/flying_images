import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import TestBox from "./components/TestBox";
import { CssBaseline, Stack } from "@mui/material";

function App() {

  /* async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  } */

  return (
    <>
      <CssBaseline />
      <Stack height='100vh'>
        <TestBox />
        <Stack direction='row' flexGrow={1}>
          <TestBox />
          <TestBox grow/>
        </Stack>
        <TestBox />
      </Stack>
    </>
  );
}

export default App;
