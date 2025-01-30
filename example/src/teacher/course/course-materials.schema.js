import mongoose, { model } from 'mongoose';
const { Schema } = mongoose;

// Schema for a single course material
const CourseMaterialSchema = new Schema({
  name: { type: String, required: true },
  pdfLink: { type: String, required: true }, // Link to the PDF file
  description: { type: String, required: true }
});

// Main schema for course materials
const CourseMaterialsSchema = new Schema({
  courseMaterials: { type: [CourseMaterialSchema], default: [] }
});

const CourseMaterial = model('CourseMaterial', CourseMaterialSchema);
const CourseMaterials = model('CourseMaterials', CourseMaterialsSchema);

export default {
  CourseMaterial,
  CourseMaterials
};
