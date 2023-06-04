import { useState } from 'react'
import Notes from './Gadgets/Notes/Notes'
import './App.css'
import Calendar from './Gadgets/Calendar/Calendar'

function App() {

  return (
    <>
    <div className="background">
      <div className="title">
      <h1>Hello you Sexy beast</h1>
      </div>
      <div className="gadgets">
      <div className="notes">
        <Notes />
      </div>
      <div className="calendar">
        <Calendar />
      </div>
      </div>
    </div>
    </>
  )
}

export default App
