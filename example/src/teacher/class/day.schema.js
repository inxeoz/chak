import mongoose, { model } from 'mongoose';
const { Schema } = mongoose;
import { ScheduleSchema } from './schedule.schema';

// Schema to represent a day with schedules
const DaySchema = new Schema({
  dayName: {
    type: String,
    required: true
  },
  schedules: {
    type: [ScheduleSchema],
    default: []
  }
});

const Day = model('Day', DaySchema);

export default { Day, DaySchema };