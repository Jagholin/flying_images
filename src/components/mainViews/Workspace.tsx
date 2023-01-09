import { observer } from 'mobx-react-lite'
import { useContext } from 'react'
import { Navigate, Outlet } from 'react-router-dom'
import { StateContext } from '../../state/context'

function Workspace() {
  const context = useContext(StateContext);

  console.log("you opened workspace index");

  return (
    <>
      {!!context.currentWorkspace || <Navigate to="create" replace={true} />}
      <p>Workspace</p>
      <p>Location: {context.currentWorkspace?.location}</p>
      <p>Name: {context.currentWorkspace?.name}</p>
    </>
  )
}

export default observer(Workspace);
