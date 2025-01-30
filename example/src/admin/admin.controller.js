import { Router } from 'express';
import AdminService from './admin.service.js';
import { JwtAuthGuard } from '../service/auth.js';
import multer from 'multer';
import {  AdminLevelAccessDTO } from '../user/user.dto.js';
const upload = multer();
const adminRouter = Router();

// Health check
adminRouter.get('/testing', (req, res) => {
  res.send('admin testing working');
});


// User signup
adminRouter.get('/user_details', upload.none(), async (req, res) => {
  const userId = req.query.userId
  console.log(userId);
 await AdminService.getUserDetails(userId, res);
});

adminRouter.post('/update_user_details', upload.none(), async (req, res) => {

  const userId = req.query.userId

  const { error, value } = AdminLevelAccessDTO.validate(req.body);
  if (error) {
    return res.status(400).json({ message: error.details[0].message });
  }
  await AdminService.updateUserDetails(value, userId, res);
});


// User login
adminRouter.get('/donation_details', async (req, res) => {
});
// User login
adminRouter.post('/update_donation', async (req, res) => {

});

adminRouter.get('/course_details', async (req, res) => {
});

adminRouter.post('/update_course', async (req, res) => {
});






adminRouter.get('/whole_user_details', upload.none(), async (req, res) => {

  const page = parseInt(req.query.page) || 1;
  const limit = parseInt(req.query.limit) || 10;
  const skip = (page - 1) * limit;
  await AdminService.getWholeUserDetails(skip, limit, res);
});

adminRouter.post('/whole_update_user_details', upload.none(), async (req, res) => {
  await AdminService.updateWholeUserDetails(req.body, res);
});


// User login
adminRouter.get('/whole_donation_details', async (req, res) => {
  await AdminService.getWholeDonationDetails(res);
});
// User login
adminRouter.post('/whole_update_donation', async (req, res) => {
  await AdminService.updateWholeDonationDetails(req.body, res);

});

adminRouter.get('/whole_course_details', async (req, res) => {
  await AdminService.getWholeCourseDetails(res);
});

adminRouter.post('whole_/update_course', async (req, res) => {
  await AdminService.updateWholeCourseDetails(req.body, res);
});



// Get user profile
adminRouter.get('/profile', JwtAuthGuard, async (req, res) => {
});
export default adminRouter;
