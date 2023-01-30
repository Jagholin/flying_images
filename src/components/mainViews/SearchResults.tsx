import { Card, CardContent, CardMedia, Grid, Typography } from '@mui/material';
import { Container } from '@mui/system';
import React, { useContext } from 'react'
import { Navigate } from 'react-router-dom';
import { StateContext } from '../../state/context'

export default function SearchResults() {
  const context = useContext(StateContext);

  return (
    <Container maxWidth='lg'>
      {!context.searchResults && <Navigate to='/workspace' />}
      <Grid container spacing={2}>
        {context.searchResults?.map((res) => (
          <Grid item xs={12} sm={6} md={4} lg={3} key={res.id}>
            <Card variant='elevation'>
              <CardMedia component='img' image={res.preview_url} alt={res.title} height={200} />
              <CardContent>
                <Typography gutterBottom variant='h6' component='p' color='text.primary'>
                  {res.title}
                </Typography>
                <Typography gutterBottom variant='body2' component='p' color='text.secondary'>
                  by {res.author_name}
                </Typography>
              </CardContent>
            </Card>
          </Grid>
        ))}
      </Grid>
    </Container>
  )
}
