import { observer } from 'mobx-react-lite'
import React, { useContext } from 'react'
import { Navigate, Outlet } from 'react-router-dom'
import { StateContext } from '../../state/context'

function Workspace() {
  const context = useContext(StateContext);

  console.log("you opened workspace index");

  return (
    <>
      <p>Workspace</p>
      {!!context.currentWorkspace || <Navigate to="create" replace={true} />}
      <Outlet />
    </>
  )
}

export default observer(Workspace);
