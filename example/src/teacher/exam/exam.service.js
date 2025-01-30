import mongoose from 'mongoose';
import Exam from './exam.schema'; // Assuming exam.schema.js exports the Exam model

class ExamsService {
    constructor() {
        this.examModel = Exam;
    }

    async addExam(examDto) {
        // Create an Exam document from the DTO
        const exam = new this.examModel(examDto);
        // Save the new exam document to the database
        return await exam.save();
    }
}

export default ExamsService;
