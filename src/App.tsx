import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import TestBox from "./components/TestBox";
import { CssBaseline, Stack } from "@mui/material";
import Header from "./components/Header";
import { createBrowserRouter, Outlet, RouterProvider } from "react-router-dom";
import Origin from "./components/mainViews/Origin";
import Workspace from "./components/mainViews/Workspace";
import WorkspaceCreate from "./components/mainViews/WorkspaceCreate";
import { StateContext } from "./state/context";
import State from "./state/state";
import { observer } from "mobx-react-lite";
import { Observer } from "mobx-react-lite/dist/ObserverComponent";

const programState = new State();

function App() {
  /* async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  } */

  const router = createBrowserRouter([
    {
      path: "/",
      element: <Origin />,
    },
    {
      path: "/next-page",
      element: <TestBox>
          Next page
        </TestBox>,
      loader: async (args) => {
        console.log("loader for next-page is called with arguments", args);
        const prom = new Promise((resolve, reject) => {
          setTimeout(() => {
            console.log("loader for next-page is finished");
            resolve(42)
          }, 4000);
        });
        return prom;
      }
    },
    {
      path: "/unknown-page",
      element: <TestBox />,
      loader: (args) => {
        console.log("loader for unknown page is called with args", args);
        return null;
      }
    },
    {
      path: "/alt-page",
      element: <TestBox>
          Alternative page
        </TestBox>,
      loader: (args) => {
        console.log("loader for alt-page is called with arguments, ", args);
        return new Promise((resolve, reject) => {
          setTimeout(() => {
            console.log("loader for alt-page is finished");
            resolve(46);
          }, 2000);
        })
      }
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
      }]
    }
  ]);

  return (
    <>
      <CssBaseline />
      <Stack height="100vh">
        <Header />
        <Stack direction="row" flexGrow={1}>
          <TestBox />
          <TestBox grow>
            <StateContext.Provider value={programState}>
              <RouterProvider router={router} />
            </StateContext.Provider>
          </TestBox>
        </Stack>
        <TestBox />
      </Stack>
    </>
  );
}

export default App;
