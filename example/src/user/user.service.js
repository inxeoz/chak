import bcrypt from 'bcryptjs';
import jwt from 'jsonwebtoken';
import UserModel from './user.schema.js';
import { sendEmail } from '../service/mail.service.js';

const JWT_SECRET = process.env.JWT_SECRET;

const UserService = {
  async getProfile(req, res) {
    try {
      
     const user = await UserModel.findOne({ userId: req.user.userId }).exec();
        if(  ! user  ) {
        return res.status(409).json({ message: 'User not exists' });
      }
      res.json(user);
    } catch (error) {
      res.status(500).json({ message: 'Failed to get profile' });
    }
  },

  async signUp(value, res) {
    const { email, password } = value;
    console.log(value);
    try {
      if (await UserModel.findOne({ email }).exec()) {
        return res.status(409).json({ message: 'User already exists' });
      }

      const hashedPassword = await bcrypt.hash(password, 10);
      const newUser = new UserModel({ email, password: hashedPassword });
      await newUser.save();
      res.status(201).json({ message: 'User created successfully' });
    } catch (error) {
      res.status(500).json({ message: 'Failed to sign up' });
    }
  },

  async login(req, res) {
    const { email, password,} = req.body;
    try {
      const user = await UserModel.findOne({ email }).exec();
      if (!user || !(await bcrypt.compare(password, user.password)) ) {
        return res.status(401).json({ message: 'Invalid credentials' });
      }


      const token = jwt.sign({ userId: user.userId, userType: user.userType}, JWT_SECRET, { expiresIn: '1h' });
      user.access_token = token;
      await user.save();

      res.json({ access_token: token , userId: user.userId});
    } catch (error) {
      res.status(500).json({ message: 'Failed to login' });
    }
  },

  async requestPasswordReset(req, res) {
    const { email } = req.body;
    try {
      const user = await UserModel.findOne({ email }).exec();
      if (!user) return res.status(404).json({ message: 'User not found' });

      const token = jwt.sign({ email: user.email }, JWT_SECRET, { expiresIn: '1h' });
      const resetUrl = `${process.env.FRONTEND_URL}?token=${token}`;
      const subject = 'Password Reset Request';
      const htmlContent = `<div><p>Click the link to reset your password:</p><a href="${resetUrl}">Reset Password</a><br>
      copy token  ${token}
      </div>`;

      sendEmail(email, subject, '', htmlContent, 'support@ridbharat.org');
      res.json({ message: 'Password reset email sent' });
    } catch (error) {
      res.status(500).json({ message: 'Failed to request password reset' });
    }
  },

  async resetPassword(req, res) {
    const { token: tokenFromQuery } = req.query;
    const { newPassword } = req.body;
    console.log(tokenFromQuery);

    try {
      const { email } = jwt.verify(tokenFromQuery, JWT_SECRET);
      console.log(email);
      const user = await UserModel.findOne({ email }).exec();
      console.log(user);
      if (!user) return res.status(404).json({ message: 'Invalid token' });
      user.password = await bcrypt.hash(newPassword.trim(), 10);
      console.log(user.password); 
      await user.save();
      res.json({ message: 'Password reset successful' });
    } catch (error) {
      console.log(error); 
      res.status(500).json({ message: 'Failed to reset password' });
    }
  },

  async editProfile(req, res) {
    const { language, gender, phone, name, dob } = req.body;
    try {
      const user = await UserModel.findOne({ email: req.user.email }).exec();
      if (!user) return res.status(404).json({ message: 'User not found' });

      Object.assign(user, { language, gender, phone, name, dob });
      await user.save();
      res.json({ message: 'Profile updated successfully' });
    } catch (error) {
      res.status(500).json({ message: 'Failed to edit profile' });
    }
  },
};

export default UserService;
