import React from "react";
import { Typography } from "@mui/material";
import { StyledCalendarBox, StyledCalendarDay } from "./StyledComponents";
import "./Calendar.css";

//interface CalendarBoxProps {
//}


const CalendarBox: React.FC<CalendarBoxProps> = () => {

    // some fake things to make it look pretty until choco gets going ;) 
    const calendarData = [
        { day: 1, event: 'Meeting with the boss' },
        { day: 2, event: 'Birthday big Choco' },
        { day: 3, event: 'Vacation bitches' },
    ];

    return (
        <StyledCalendarBox>
          <Typography variant="h5" gutterBottom>
            Calendar
          </Typography>
          <div className="calendar">
            {calendarData.map(({ day, event }) => (
              <StyledCalendarDay key={day}>
                <Typography variant="h6" fontWeight="bold">
                  {day}
                </Typography>
                <Typography variant="subtitle1" color="text.secondary">
                  {event}
                </Typography>
              </StyledCalendarDay>
            ))}
          </div>
        </StyledCalendarBox>
      );
    };

export default CalendarBox;
