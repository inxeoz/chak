import { ExamDto } from '../exam/exam.dto';
import { AssignmentDto } from './assignment.dto';
import { ResultDto } from '../exam/result.dto';

// Main DTO for assignments and exams
class AssignmentsExamsDto {
  constructor(assignments = [], exams = [], results = []) {
    this.assignments = assignments;
    this.exams = exams;
    this.results = results;
  }
}

export default { AssignmentsExamsDto };
