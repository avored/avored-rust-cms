import { BrowserRouter, Route, Routes } from "react-router-dom"
import HomePage from "./pages/home/HomePage"
import AppLayout from "./layout/AppLayout"
import "preline/preline"
import { IStaticMethods } from "preline/preline";
import { useEffect } from "react";
import './index.css'

declare global {
  interface Window {
    HSStaticMethods: IStaticMethods;
  }
}

function App() {

  useEffect(() => {
    window.HSStaticMethods.autoInit();
  })
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<AppLayout />}>
          <Route path="/" element={<HomePage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  )
}

export default App
