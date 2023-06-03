import { useState } from 'react'
import CalendarBox from './Gadgets/Calendar/Calendar'
import './App.css'
import Notes from './Gadgets/Notes/Notes'

function App() {

  return (
    <>
      <h1>Hello you Sexy beast</h1>
      <div className="gadget-contrainer">
        <CalendarBox />
        <Notes />
      </div>
    </>
  )
}

export default App
