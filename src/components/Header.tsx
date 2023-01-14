import { AppBar, Toolbar, Typography } from '@mui/material'
import { getVersion } from "@tauri-apps/api/app";
import React, { useEffect, useState } from 'react'

export default function Header() {
  const [version, setVersion] = useState<string>("");

  useEffect(() => {
    getVersion().then(ver => setVersion(ver));
  }, []);

  return (
    <AppBar>
      <Toolbar>
        <div style={{flexGrow: 1}} />
        <Typography variant='subtitle1'>
          Flying images v{version}
        </Typography>
      </Toolbar>
    </AppBar>
  )
}
