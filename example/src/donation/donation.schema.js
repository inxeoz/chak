import { Schema, model } from 'mongoose';
import { v4 as uuidv4 } from 'uuid'; // For generating unique courseId

const DonationSchema = new Schema({
	donationId: {
		type: String,
		default: uuidv4,
		unique: true,
	},
	amount: {
		type: Number,
		required: true,
	},
	donorName: {
		type: String,
		required: true,
	},
	donorEmail: {
		type: String,
		required: true,
	},
	donationDate: {
		type: Date,
		default: Date.now,
	}

});


const DonationModel = model('DonationModel', DonationSchema);

export default DonationModel;
