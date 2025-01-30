import mongoose, { model } from 'mongoose';
const { Schema } = mongoose;
import { WeekSchema } from './week.schema';

// Schema to represent the class schedule
const ClassScheduleSchema = new Schema({
  thisWeek: { type: WeekSchema, required: true },
  nextWeek: { type: WeekSchema, required: true },
  previousWeek: { type: WeekSchema, required: true }
});

const ClassSchedule = model('ClassSchedule', ClassScheduleSchema);

export default { ClassSchedule, ClassScheduleSchema };
