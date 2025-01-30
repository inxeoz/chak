import mongoose from 'mongoose';
const { Schema, model } = mongoose;

// Schema for an assignment
const AssignmentSchema = new Schema({
  subjectName: { type: String, required: true },
  dueDate: { type: Date, required: true }, // Store as Date
  result: { type: String }, // Optional result field
  shareLink: { type: String, required: true }
});

const Assignment = model('Assignment', AssignmentSchema);

export default {
  Assignment,
  AssignmentSchema
};
