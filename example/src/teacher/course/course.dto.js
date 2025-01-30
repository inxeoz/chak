import Joi from 'joi';

// Course DTO
const CourseDto = Joi.object({
  courseName: Joi.string().optional(),
  description: Joi.string().optional(),
  startDate: Joi.string().optional(),
  tags: Joi.array().items(Joi.string()).optional(),
  duration: Joi.string().optional(),
  difficulty: Joi.string().optional(),
  language: Joi.string().optional(),
  price: Joi.string().optional(),
  rating: Joi.string().optional()
});

// Course Material DTO
const CourseMaterialDto = Joi.object({
  name: Joi.string().required(),
  pdfLink: Joi.string().uri().required(),
  description: Joi.string().required()
});

// Course Materials DTO
const CourseMaterialsDto = Joi.object({
  courseMaterials: Joi.array().items(CourseMaterialDto).required()
});

export {
  CourseDto,
  CourseMaterialDto,
  CourseMaterialsDto
};
