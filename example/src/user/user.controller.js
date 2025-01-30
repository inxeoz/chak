import { Router } from 'express';
import UserService from './user.service.js';
import { UserSignDTO } from './user.dto.js';
import { JwtAuthGuard } from '../service/auth.js';

const userRouter = Router();

// Health check
userRouter.get('/signup', (req, res) => {
  res.send('Signup route is accessible');
});

// User signup
userRouter.post('/signup', async (req, res) => {
  const { error, value } = UserSignDTO.validate(req.body);
  if (error) {
    return res.status(400).json({ message: error.details[0].message });
  }
  await UserService.signUp(value, res);
});

// User login
userRouter.post('/login', async (req, res) => {
  await UserService.login(req, res);
});

// Get user profile
userRouter.get('/profile', JwtAuthGuard, async (req, res) => {
  await UserService.getProfile(req, res);
});

// Request password reset
userRouter.post('/request-reset-password', async (req, res) => {
  await UserService.requestPasswordReset(req, res);
});

// Reset password
userRouter.post('/reset-password', async (req, res) => {
  await UserService.resetPassword(req, res);
});

// Edit user profile
userRouter.patch('/edit-profile', JwtAuthGuard, async (req, res) => {
  await UserService.editProfile(req, res);
});

export default userRouter;
