import { Schema, model } from 'mongoose';

// Schema for research
const ResearchSchema = new Schema({
  title: { type: String, required: true },
  description: { type: String, required: false } // Optional description
});

// Schema for projects
const ProjectSchema = new Schema({
  title: { type: String, required: true },
  description: { type: String, required: false } // Optional description
});

// Schema for conferences
const ConferenceSchema = new Schema({
  title: { type: String, required: true },
  description: { type: String, required: false } // Optional description
});

// Schema for approvals
const ApprovalSchema = new Schema({
  title: { type: String, required: true },
  description: { type: String, required: false } // Optional description
});

// Main schema for teacher information
const TeacherSchema = new Schema({
  teacherId: { type: String, required: true }, // Unique identifier for the teacher
  specialization: { type: [String], required: true }, // Array of specializations
  research: { type: [ResearchSchema], default: [] }, // Array of research objects
  projects: { type: [ProjectSchema], default: [] }, // Array of project objects
  conferences: { type: [ConferenceSchema], default: [] }, // Array of conference objects
  approvals: { type: [ApprovalSchema], default: [] } // Array of approval objects
});

// Create models from schemas
const ResearchModel = model('ResearchModel', ResearchSchema);
const ProjectModel = model('ProjectModel', ProjectSchema);
const ConferenceModel = model('ConferenceModel', ConferenceSchema);
const ApprovalModel = model('ApprovalModel', ApprovalSchema);
const TeacherModel = model('TeacherModel', TeacherSchema);

// Export models for use in other parts of the application
export default {
  ResearchModel,
  ProjectModel,
  ConferenceModel,
  ApprovalModel,
  TeacherModel
};
