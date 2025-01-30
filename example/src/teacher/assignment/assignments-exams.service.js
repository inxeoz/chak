import { model } from 'mongoose';
import AssignmentDto from './assignment.dto';
import Assignment from './assignment.schema';
import AssignmentsExams from './assignments-exams.schema';

class AssignmentsService {
  constructor() {
    this.assignmentsModel = model('AssignmentsExams', AssignmentsExams);
    this.assignmentModel = model('Assignment', Assignment);
  }

  // Add Assignment
  async addAssignment(assignmentDto) {
    const assignmentsExams = await this.assignmentsModel.findOne();

    // Create an Assignment document from the DTO
    const assignment = new this.assignmentModel(assignmentDto);

    assignmentsExams.assignments.push(assignment);
    return assignmentsExams.save();
  }

  // async getAll() {
  //   return this.assignmentsModel.findOne().exec();
  // }
}

export default AssignmentsService;
