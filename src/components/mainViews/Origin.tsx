import { Navigate, NavLink } from 'react-router-dom'

export default function Origin() {
  return (
    <div>
      <p>
        <Navigate to="/workspace" />
        <NavLink to="/workspace">
          go to workspace
        </NavLink>
      </p>
    </div>
  )
}
