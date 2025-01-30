import mongoose, { model } from 'mongoose';
const { Schema } = mongoose;

// Schema to represent a single schedule
const scheduleSchema = new Schema({
  educator: { type: String, required: true },
  time: { type: String, required: true }, // Time of class in HH:mm format or ISO 8601
  subjectName: { type: String, required: true }
});

const Schedule = model('Schedule', scheduleSchema);

export default Schedule;
