import { validate } from 'class-validator';

const validationMiddleware = (dtoClass) => async (req, res, next) => {
  const dtoInstance = Object.assign(new dtoClass(), req.body);

  const errors = await validate(dtoInstance);
  if (errors.length > 0) {
    return res.status(400).json({
      message: 'Validation failed',
      errors: errors.map((err) => ({
        field: err.property,
        constraints: err.constraints,
      })),
    });
  }
  next();
};

export default { validationMiddleware };
