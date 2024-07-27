import { useState } from 'react'
import logo from './assets/logo_only.svg'
import './App.css'

function App() {
  const [count, setCount] = useState(0)

  return (
    <>
      <div>
        <a href="https://avored.com" target="_blank">
          <img src={logo} className="logo" alt="AvoRed logo" />
        </a>
      </div>
      <h1>AvoRed content management system</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
      </div>
    </>
  )
}

export default App
