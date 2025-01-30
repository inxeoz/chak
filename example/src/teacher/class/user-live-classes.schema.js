import mongoose, { model } from 'mongoose';
const { Schema } = mongoose;
import { LiveClassSchema } from './live-class.schema';

const UserLiveClassesSchema = new Schema({
  previous: { type: [LiveClassSchema], default: [] },
  current: { type: [LiveClassSchema], default: [] },
  upcoming: { type: [LiveClassSchema], default: [] }
});

const UserLiveClasses = model('UserLiveClasses', UserLiveClassesSchema);

export default { UserLiveClasses, UserLiveClassesSchema };
