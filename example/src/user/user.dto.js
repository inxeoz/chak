import Joi from 'joi';

// Base DTO for user sign-in
const UserSignDTO = Joi.object({
  email: Joi.string().email().required(),
  password: Joi.string().required(),
});

const AdminLevelAccessDTO = Joi.object({
  username: Joi.string().optional(),
  email: Joi.string().email().optional(),
  userId: Joi.string().required(),
  userType: Joi.string().valid('student', 'admin', 'teacher').optional(),
  language: Joi.string().optional(),
  gender: Joi.string().valid('male', 'female', 'other').optional(),
  phone: Joi.string().pattern(/^\+?[1-9]\d{1,14}$/).optional().messages({ 'string.pattern.base': 'Invalid phone number format' }),
  dob: Joi.string().optional(),
  name: Joi.string().optional(),
  createdAt: Joi.date().iso().optional(),
  updatedAt: Joi.date().iso().optional(),
});
// DTO for User-level access
const UserLevelAccessDTO = Joi.object({
  username: Joi.string().optional(),
  email: Joi.string().email().optional(),
  userId: Joi.string().optional(),
  userType: Joi.string().valid('student', 'admin', 'teacher').optional(),
  language: Joi.string().optional(),
  createdAt: Joi.date().iso().optional(),
});

export { UserSignDTO, UserLevelAccessDTO, AdminLevelAccessDTO };



// // Role-based DTO using `.alter()`
// const RoleBasedUserDTO = UserInfoDTO.alter({
//   admin: (schema) => schema.append({
//     createdAt: Joi.date().iso().default(() => new Date().toISOString()),
//     updatedAt: Joi.date().iso().default(() => new Date().toISOString()),
//     userId: Joi.string().default(() => `ADMIN-${Math.floor(Math.random() * 100000)}`),
//   }),
//   user: (schema) => schema, // No additional fields for regular users
// });