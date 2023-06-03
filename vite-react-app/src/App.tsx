import { useState } from 'react'
import CalendarBox from './Gadgets/Calendar/Calendar'
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
