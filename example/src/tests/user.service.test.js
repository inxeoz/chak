import bcrypt from 'bcryptjs';
import jwt from 'jsonwebtoken';
import UserModel from '../src/user.schema.js';
import { sendEmail } from '../src/service/mail.service.js';
import UserService from '../src/service/user.service.js';

// Mock dependencies
jest.mock('bcryptjs');
jest.mock('jsonwebtoken');
jest.mock('../src/user.schema.js');
jest.mock('../src/service/mail.service.js');

const test_email = '1@test.com'
const test_password = '1'

describe('UserService Tests', () => {
  describe('SignUp', () => {
    it('should successfully create a new user', async () => {
      const mockUserData = { email: test_email, password: test_password };
      UserModel.findOne.mockResolvedValue(null); // Simulate that the user doesn't exist

      bcrypt.hash.mockResolvedValue('hashed_password'); // Simulate password hashing
      UserModel.prototype.save.mockResolvedValue(mockUserData); // Simulate saving the user

      const res = {
        status: jest.fn().mockReturnThis(),
        json: jest.fn(),
      };

      await UserService.signUp(mockUserData, res);

      expect(res.status).toHaveBeenCalledWith(201);
      expect(res.json).toHaveBeenCalledWith({ message: 'User created successfully' });
    });

    it('should return error if user already exists', async () => {
      const mockUserData = { email: test_email, password: test_password };
      UserModel.findOne.mockResolvedValue(mockUserData); // Simulate that the user already exists

      const res = {
        status: jest.fn().mockReturnThis(),
        json: jest.fn(),
      };

      await UserService.signUp(mockUserData, res);

      expect(res.status).toHaveBeenCalledWith(409);
      expect(res.json).toHaveBeenCalledWith({ message: 'User already exists' });
    });
  });

  describe('Login', () => {
    it('should successfully log in the user and return a token', async () => {
      const mockUser = { email: test_email, password: 'hashed_password' };
      const mockRequest = { body: { email: test_email, password: test_password } };

      UserModel.findOne.mockResolvedValue(mockUser);
      bcrypt.compare.mockResolvedValue(true); // Simulate correct password comparison
      jwt.sign.mockReturnValue('fake-jwt-token'); // Simulate token generation
      UserModel.prototype.save.mockResolvedValue(mockUser); // Simulate saving the user with the token

      const res = {
        json: jest.fn(),
      };

      await UserService.login(mockRequest, res);

      expect(res.json).toHaveBeenCalledWith({ access_token: 'fake-jwt-token', userId: undefined });
    });

    it('should return error if invalid credentials', async () => {
      const mockRequest = { body: { email: test_email, password: 'wrongpassword' } };

      UserModel.findOne.mockResolvedValue({ email: test_email, password: 'hashed_password' });
      bcrypt.compare.mockResolvedValue(false); // Simulate wrong password comparison

      const res = {
        status: jest.fn().mockReturnThis(),
        json: jest.fn(),
      };

      await UserService.login(mockRequest, res);

      expect(res.status).toHaveBeenCalledWith(401);
      expect(res.json).toHaveBeenCalledWith({ message: 'Invalid credentials' });
    });
  });

//   describe('Request Password Reset', () => {
//     it('should successfully send password reset email', async () => {
//       const mockUser = { email: test_email };
//       const mockRequest = { body: { email: test_email } };

//       UserModel.findOne.mockResolvedValue(mockUser); // Simulate that the user exists
//       jwt.sign.mockReturnValue('reset-token'); // Simulate JWT token generation
//       sendEmail.mockResolvedValue(true); // Simulate email sending

//       const res = {
//         json: jest.fn(),
//       };

//       await UserService.requestPasswordReset(mockRequest, res);

//       expect(res.json).toHaveBeenCalledWith({ message: 'Password reset email sent' });
//     });

//     it('should return error if user not found', async () => {
//       const mockRequest = { body: { email: 'notfound@example.com' } };

//       UserModel.findOne.mockResolvedValue(null); // Simulate user not found

//       const res = {
//         status: jest.fn().mockReturnThis(),
//         json: jest.fn(),
//       };

//       await UserService.requestPasswordReset(mockRequest, res);

//       expect(res.status).toHaveBeenCalledWith(404);
//       expect(res.json).toHaveBeenCalledWith({ message: 'User not found' });
//     });
//   });

//   describe('Reset Password', () => {
//     it('should successfully reset the password', async () => {
//       const mockUser = { email: test_email, password: 'hashed_password' };
//       const mockRequest = { query: { token: 'valid-token' }, body: { newPassword: 'newpassword123' } };

//       jwt.verify.mockReturnValue({ email: test_email }); // Simulate valid token verification
//       UserModel.findOne.mockResolvedValue(mockUser);
//       bcrypt.hash.mockResolvedValue('new-hashed-password'); // Simulate password hashing
//       UserModel.prototype.save.mockResolvedValue(mockUser); // Simulate saving the updated user

//       const res = {
//         json: jest.fn(),
//       };

//       await UserService.resetPassword(mockRequest, res);

//       expect(res.json).toHaveBeenCalledWith({ message: 'Password reset successful' });
//     });

//     it('should return error if token is invalid', async () => {
//       const mockRequest = { query: { token: 'invalid-token' }, body: { newPassword: 'newpassword123' } };

//       jwt.verify.mockRejectedValue(new Error('invalid token')); // Simulate invalid token

//       const res = {
//         status: jest.fn().mockReturnThis(),
//         json: jest.fn(),
//       };

//       await UserService.resetPassword(mockRequest, res);

//       expect(res.status).toHaveBeenCalledWith(500);
//       expect(res.json).toHaveBeenCalledWith({ message: 'Failed to reset password' });
//     });
//   });

  // Add more tests for other methods like editProfile, etc.
});
