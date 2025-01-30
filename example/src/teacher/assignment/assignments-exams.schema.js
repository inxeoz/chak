import mongoose, { model } from 'mongoose';
const { Schema } = mongoose;

import { AssignmentSchema } from './assignment.schema';
import { ExamSchema } from '../exam/exam.schema';
import { ResultSchema } from '../exam/result.schema';

// Schema for results

// Main schema for assignments and exams
const AssignmentsExamsSchema = new Schema({
  assignments: { type: [AssignmentSchema], default: [] },
  exams: { type: [ExamSchema], default: [] },
  results: { type: [ResultSchema], default: [] }
});

const AssignmentsExams = model('AssignmentsExams', AssignmentsExamsSchema);

export default AssignmentsExams;
