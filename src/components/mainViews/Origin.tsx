import { NavLink } from 'react-router-dom'

export default function Origin() {
  return (
    <div>
      <p>
        Origin
        <NavLink to="/next-page">
          this links to another page
        </NavLink>
      </p>
      <p>
        <NavLink to="/alt-page">
          this links to alternative page
        </NavLink>
      </p>
      <p>
        <NavLink to="/workspace">
          go to workspace
        </NavLink>
      </p>
    </div>
  )
}
