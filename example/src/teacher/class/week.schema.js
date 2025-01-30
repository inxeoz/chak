import mongoose, { model } from 'mongoose';
const { Schema } = mongoose;
import { DaySchema } from './day.schema';

// Schema to represent a week with days
const WeekSchema = new Schema({
  days: { type: [DaySchema], default: [] }
});

const Week = model('Week', WeekSchema);

export default { Week, WeekSchema };