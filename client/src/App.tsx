import { useState } from 'react'
import CalendarBox from './Gadgets/Calendar/Calendar'
import Notes from './Gadgets/Notes/Notes'
import './App.css'

function App() {
  const [count, setCount] = useState(0)

  return (
    <>
      <h1>Hello you Sexy beast</h1>
      <div className="gadget-container">
        <Notes />
        <CalendarBox />
      </div>
    </>
  )
}

export default App
