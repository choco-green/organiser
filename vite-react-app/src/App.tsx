import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import CalendarBox from './Gadgets/Calendar'
import './App.css'

function App() {
  const [count, setCount] = useState(0)

  return (
    <>
      <h1>Hello you Sexy beast</h1>
      <div>
        <CalendarBox />
      </div>
    </>
  )
}

export default App
