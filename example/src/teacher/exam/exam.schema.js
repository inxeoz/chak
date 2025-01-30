import mongoose, { model } from 'mongoose';
const { Schema } = mongoose;

// Schema for an exam
const examSchema = new Schema({
  examName: {
    type: String,
    required: true
  },
  date: {
    type: Date,
    required: true
  },
  time: {
    type: String,
    required: true
  }
});

const Exam = model('Exam', examSchema);

export default Exam;