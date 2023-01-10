import { Button } from '@mui/material';
import { Box } from '@mui/system';
import { observer } from 'mobx-react-lite'
import { useContext, useCallback, useState } from 'react'
import { Navigate, Outlet } from 'react-router-dom'
import { get_csrf_token, test_da_request } from '../../commands/imageSearchCommands';
import { StateContext } from '../../state/context'

function Workspace() {
  const context = useContext(StateContext);
  const [searchField, setSearchField] = useState("");

  const handleSearchSubmit = useCallback(() => {
    
  }, []);

  const handleGetToken = useCallback(() => {
    get_csrf_token().catch((e) => {
      console.log("error caught: ", e);
    })
  }, []);

  const handleTestDa = useCallback(() => {
    test_da_request().catch((e) => {
      console.log("error caught: ", e);
    });
  }, []);

  return (
    <>
      {!!context.currentWorkspace || <Navigate to="create" replace={true} />}
      <p>Workspace</p>
      <p>Location: {context.currentWorkspace?.location}</p>
      <p>Name: {context.currentWorkspace?.name}</p>

      <p>Search for images using ArtStation API:</p>
      <Button onClick={handleGetToken}>get csrf token</Button>
      <Button onClick={handleTestDa}>test da request</Button>
      <Box component="form" onSubmit={handleSearchSubmit}>
      </Box>
    </>
  )
}

export default observer(Workspace);
