import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { MiscClient } from 'grpc-avored/MiscServiceClientPb'
import { HealthCheckRequest, HealthCheckResponse } from 'grpc-avored/misc_pb'

function App() {
  const [count, setCount] = useState(0)
  var miscClient = new MiscClient('http://localhost:50051');
  var request = new HealthCheckRequest();

  miscClient.healthCheck(request).then((response: HealthCheckResponse) => {
    console.log(response.getStatus());
  })


  return (
    <>
      <div>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

export default App
