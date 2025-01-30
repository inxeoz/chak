import dotenv from 'dotenv';
import mongoose from 'mongoose';

dotenv.config();

let gridFsBucket;

const connectDB = async () => {
  try {
    const conn = await mongoose.connect(process.env.MONGO_URI || 'mongodb://localhost:27017/yourdbname', {
      useNewUrlParser: true,
      useUnifiedTopology: true,
    });

    gridFsBucket = new mongoose.mongo.GridFSBucket(conn.connection.db, {
      bucketName: 'uploads',
    });

    console.log('MongoDB connected successfully');
  } catch (error) {
    console.error('MongoDB connection failed:', error.message);
    process.exit(1); // Exit process with failure
  }
};

// Export the database connection and GridFS bucket
export { connectDB, gridFsBucket };
