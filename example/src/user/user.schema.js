import { Schema, model } from 'mongoose';
import { v4 as uuidv4 } from 'uuid'; // For generating unique courseId

const userSchema = new Schema({

  username: { type: String },
  email: { type: String, required: true, unique: true },
  password: { type: String, required: true },
  userId: {type: String, unique: true, default: uuidv4},
  userType: { type: String, default: 'student' },
  language: { type: String },
  gender: { type: String },
  phone: { type: String },
  dob: { type: String },
  name: { type: String },
  access_token: { type: String }
});


// Role-Based Transform Functions
userSchema.methods.toAdminLevelAccess = function () {
  const { username, email, userId, userType, language, gender, phone, dob, name, createdAt, updatedAt } = this;
  return { username, email, userId, userType, language, gender, phone, dob, name, createdAt, updatedAt };
};

userSchema.methods.toUserLevelAccess = function () {
  const { username, email, userId, userType, language, createdAt } = this;
  return { username, email, userId, userType, language, createdAt };
};

userSchema.methods.toTeacherLevelAccess = function () {
  const { username, email, userId, userType, language, createdAt } = this;
  return { username, email, userId, userType, language, createdAt };
};


const UserModel = model('UserModel', userSchema);

export default UserModel;
