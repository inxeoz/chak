import { Schema, model } from 'mongoose';
import { v4 as uuidv4 } from 'uuid'; // For generating unique courseId


const CourseSchema = new Schema({
  createdBy_userId: { type: String, required: true },
  courseName: { type: String, default: 'New Course' },
  courseId: { type: String, unique: true, default: uuidv4 }, // Correct as function reference
  description: { type: String , default: 'New Course Description' },
  videoUrls:  {type: [String], default: ["example url"]},
  language: { type: String, default: 'English' },
  tags: {type: [String], default: ["new"]},
  duration: { type: String },
  difficulty: { type: String, default: 'Beginner' },
  price: { type: String , default: '0.00' },
  rating: { type: String , default: '0.0' },
  createdAt: { type: Date, default: Date.now }, // Use Date type and correct default
  startDate: { type: Date }, 
  courseMaterials: [String],
});

const CourseModel = model('CourseModel', CourseSchema);

export default CourseModel;

