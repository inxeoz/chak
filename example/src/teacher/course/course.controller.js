import { Router } from 'express';
import multer from 'multer';
import CoursesService from './course.service.js'; 
import fileRouter from '../../service/fileupload.controller.js';
import { JwtAuthGuard } from '../../service/auth.js';
import { CourseDto } from './course.dto.js';

const CourseRouter = Router();

// Set up multer for file uploads
const storage = multer.diskStorage({
  destination: './uploads', 
  filename: (req, file, callback) => {
    const uniqueSuffix = Date.now() + '-' + Math.round(Math.random() * 1e9);
    callback(null, `${uniqueSuffix}-${file.originalname}`);
  },
});

const upload = multer({ storage });


// Create a course
CourseRouter.post('/create_course', JwtAuthGuard, async (req, res) => {
    const {error, value} = CourseDto.validate(req.body);
	const userId = req.user.userId;
	console.log(userId);
    if (error) {
      return res.status(400).json({ message: error.details[0].message });
    }
   await CoursesService.createCourse(value , userId, res);

});

// Get courses with pagination
CourseRouter.get('/get_courses', async (req, res) => {
	try {
	  const page = parseInt(req.query.page) || 1;
	  const limit = parseInt(req.query.limit) || 10;
	  const skip = (page - 1) * limit;
	  const courseSearchDTO = req.body; 
  
	  const result = await CoursesService.getCourses(skip, limit, courseSearchDTO);
	  res.status(200).json(result);
	} catch (error) {
	  res.status(500).json({ message: error.message });
	}
  });


CourseRouter.post('/update_course', JwtAuthGuard, async (req, res) => {

	const courseId = req.query.courseId;
	const {error, value} = CourseDto.validate(req.body);

    if (error) {
      return res.status(400).json({ message: error.details[0].message });
    }
   await CoursesService.updateCourse(value ,  courseId, res);

});



// Add course material
CourseRouter.post('/add-material', JwtAuthGuard, upload.array('courseMaterial', 10), async (req, res) => {
  try {
    const courseId = req.query.courseId;
    const files = req.files;

    const result = await CoursesService.addCourseMaterial(courseId, files);
    res.status(200).json(result);
  } catch (error) {
    res.status(500).json({ message: error.message });
  }
});

// Download a single file
CourseRouter.get('/material', async (req, res) => {
  try {
    req.query = { filenames: [req.query.filename] };
    await fileRouter.handle(req, res);
  } catch (error) {
    res.status(500).json({ message: error.message });
  }
});

// Download multiple files
CourseRouter.get('/materials', async (req, res) => {
  try {
    const filenames = req.query.filenames.split(','); 
    req.query = { filenames }; 
    await fileRouter.handle(req, res);
  } catch (error) {
    res.status(500).json({ message: error.message });
  }
});

export default CourseRouter;
