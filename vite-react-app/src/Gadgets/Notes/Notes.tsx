import React from "react";
import { useState } from "react";
import { IconButton, TextField } from "@mui/material";
import { Add, Delete } from "@mui/icons-material";
import './Notes.css';

interface Note {
    id: number;
    content: string;
}

const Notes: React.FC = () => {
    const [notes, setNotes] = useState<Note[]>([]);
    const [newNote, setNewNote] = useState('');
  
    const handleAddNote = () => {
      if (newNote.trim() !== '') {
        const note: Note = {
          id: Date.now(),
          content: newNote,
        };
        setNotes([...notes, note]);
        setNewNote('');
      }
    };

const handleDeleteNote = (id: number) => {
    const updatedNotes = notes.filter((note) => note.id !== id);
    setNotes(updatedNotes);
};

return ( 
    <div className="notes-contrainer">
        <div className="notes-header">
            <TextField
            label="Write a note"
            variant="outlined"
            value={newNote}
            onChange={(e) => setNewNote (e.target.value)}
            />
            <IconButton color="primary" onClick={handleAddNote}>
                <Add />
            </IconButton>
        </div>
        <div className="notes-list">
            {notes.map((note) => (
                <div key={note.id} className="note-item">
                    <span>{note.content}</span>
                    <IconButton color="secondary" onClick={() => handleDeleteNote (note.id)}>
                        <Delete />
                    </IconButton>
                </div>
           ) )}
        </div>
    </div>
);
};

export default Notes;