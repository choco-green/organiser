import { useState } from 'react';
import { AdapterDateFns } from '@mui/x-date-pickers/AdapterDateFns';
import { LocalizationProvider } from '@mui/x-date-pickers/LocalizationProvider';
import { StaticDatePicker } from '@mui/x-date-pickers/StaticDatePicker';
import { format } from 'date-fns';
import './Calendar.css';
import { TextField } from '@mui/material';

const Calendar = () => {
  const [selectedDate, setSelectedDate] = useState<Date | null>(new Date());

  const handleDateChange = (date: Date | null) => {
    setSelectedDate(date);
  };

  return (
    <div className="calendar-container">
    <LocalizationProvider dateAdapter={AdapterDateFns}>
      <StaticDatePicker
        label="Select Date"
        value={selectedDate}
        onChange={handleDateChange}
        disablePast
        openTo="day"
        renderInput={(params: any) => <TextField {...params} />}
        renderDay={(day: number | Date, _value: any) => (
          <>{format(day, 'dd')}</> 
        )}
      />
    </LocalizationProvider>
    </div>
  );
};

export default Calendar;