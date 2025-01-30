import express from 'express';
import cors from 'cors';
import methodOverride from 'method-override';
import { validationResult } from 'express-validator';
import { config } from 'dotenv';
import morgan from 'morgan';
import { createWriteStream } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import {connectDB, gridFsBucket} from './src/service/db.js';
import userRouter from './src/user/user.controller.js';
import teacherRouter from './src/teacher/teacher.controller.js';
import fileRouter, { setGridFsBucket } from './src/service/fileupload.controller.js';
import adminRouter from './src/admin/admin.controller.js';
import bodyParser from 'body-parser';

import CourseRouter from './src/teacher/course/course.controller.js';





// Initialize app and environment
config();
await connectDB();
setGridFsBucket(gridFsBucket);
const app = express();
const __dirname = dirname(fileURLToPath(import.meta.url));


// Middleware to parse different types of request bodies
app.use(express.urlencoded({ extended: true })); // For URL-encoded
app.use(express.json()); // For JSON payloads


// Middleware
app.use(express.json()); // Parse JSON bodies
app.use(cors({ origin: process.env.CORS_ORIGIN || '*', credentials: process.env.CORS_CREDENTIALS === 'true' })); // CORS setup
app.use(methodOverride('_method')); // Support RESTful APIs
app.use(morgan('combined', { stream: createWriteStream(join(__dirname, 'access.log'), { flags: 'a' }) })); // Log requests

// Validation errors middleware
app.use((req, res, next) => {
	const errors = validationResult(req);
	if (!errors.isEmpty()) {
		return res.status(400).json({ errors: errors.array() });
	}
	next();
});

// Health check
app.get('/', (req, res) => res.send('API is running'));
app.get('/test', (req, res) => {

	console.log(req)

	console.log('URL Params:', req.params);

	// Log query parameters
	console.log('Query Params:', req.query);
  
	// Log form data body
	console.log('Form Body:', req.body);

	res.send("hii");
});
// Routes
app.use('/api/user', userRouter);
app.use('/api/teacher', teacherRouter);
app.use('/api/admin', adminRouter);
app.use('/api/course', CourseRouter);

// Global error handling
app.use((err, req, res, next) => {
	console.error(`[Error]: ${err.message}`);
	res.status(err.status || 500).json({
		message: err.message || 'An unexpected error occurred',
		stack: process.env.NODE_ENV === 'development' ? err.stack : undefined,
	});
});

// Start server
const PORT = process.env.PORT || 5000;
app.listen(PORT, () => console.log(`Server is running on port ${PORT}`));
