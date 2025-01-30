import mongoose, { model } from 'mongoose';
const { Schema } = mongoose;

const liveClassSchema = new Schema({
  educator: { type: String, required: true },
  language: { type: String, required: true },
  level: { type: String, required: true },
  liveClassName: { type: String, required: true },
  startingTime: { type: Date, required: true }
});

const LiveClass = model('LiveClass', liveClassSchema);

export default LiveClass;
