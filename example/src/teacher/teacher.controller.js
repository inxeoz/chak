import { Router } from 'express';
import multer from 'multer';
import CoursesService from './course/course.service.js'; 
const teacherRouter = Router();


teacherRouter.get('/teacher_details', async (req, res) => {
    const userId = req.query.userId;
    await CoursesService.getTeacherDetails(userId, res);
});





export default teacherRouter;
