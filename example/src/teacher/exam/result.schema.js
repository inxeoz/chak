import { Schema, model } from 'mongoose';

const resultSchema = new Schema({
  examName: {
    type: String,
    required: true
  },
  result: {
    type: String,
    required: true
  }
});

const Result = model('Result', resultSchema);

export default Result;