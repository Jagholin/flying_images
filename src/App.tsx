import "./App.css";
import TestBox from "./components/TestBox";
import { AppBar, CssBaseline, Stack, Toolbar } from "@mui/material";
import Header from "./components/Header";
import { createBrowserRouter, Outlet, RouterProvider } from "react-router-dom";
import Origin from "./components/mainViews/Origin";
import Workspace from "./components/mainViews/Workspace";
import WorkspaceCreate from "./components/mainViews/WorkspaceCreate";
import { StateContext } from "./state/context";
import State from "./state/state";
import { appWindow } from "@tauri-apps/api/window";
import { useEffect } from "react";
import SearchResults from "./components/mainViews/SearchResults";

const programState = new State();
appWindow.listen("task_finished", (event) => {
  console.log("event task_finished caught, payload ", event.payload);
});

function App() {
  /* async function greet() {s
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  } */

  const router = createBrowserRouter([
    {
      path: "/",
      element: <Origin />,
    },
    {
      path: "/workspace",
      element: <Outlet />,
      children: [{
        index: true,
        element: <Workspace />
      }, {
        path: "create",
        element: <WorkspaceCreate />
      }, {
        path: "search",
        element: <SearchResults />
      }]
    }
  ]);

  return (
    <>
      <CssBaseline />
      <Stack height="100vh">
        <Header />
        <Stack direction="row" flexGrow={1} marginTop='64px'>
          <TestBox />
          <TestBox grow>
            <StateContext.Provider value={programState}>
              <RouterProvider router={router} />
            </StateContext.Provider>
          </TestBox>
        </Stack>
        <AppBar position="sticky">
          Hello
        </AppBar>
      </Stack>
    </>
  );
}

export default App;
