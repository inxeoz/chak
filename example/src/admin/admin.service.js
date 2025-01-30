import UserModel from "../user/user.schema.js";
import CourseModel from "../teacher/course/course.schema.js";
import DonationModel from "../donation/donation.schema.js";
import TeacherModel from "../teacher/teacher.schema.js";
const AdminService = {

  async getUserDetails(userId, res) {
    try {
      const user = await UserModel.findOne({ userId }).exec();
      res.json(user.toAdminLevelAccess());
    } catch (error) {
      res.status(500).json({ message: 'Failed to get user' });
    }
  },

  async updateUserDetails(value,userId,  res) {
    try {
      // Check if the user exists
      const user = await UserModel.findOne({ userId }).exec();
      if (!user) {
        return res.status(404).json({ message: 'User does not exist' });
      }
  
      // Update the user with the provided value
      Object.assign(user, value); // Use Object.assign to update fields of the existing user
      await user.save(); // Save the updated user
  
      // Return success message and updated user
      res.status(200).json({ message: 'Successfully updated', user });
    } catch (error) {
      console.error(error); // Log the error for debugging purposes
      res.status(500).json({ message: 'Failed to update user details' });
    }
  },

  async getDonationDetails() {
  },
  async updateDonation() {
  },


  async getCourseDetails() {
  },

  async updateCourse() {
  },

  async getWholeUserDetails(skip, limit,  res) {


    try {
      const users =  await UserModel
        .find()
        .skip(skip)
        .sort({ createdAt: -1 })
        .limit(limit)
      res.status(200).json({ users });
    } catch (error) {
      console.error(error); // Log the error for debugging purposes
      res.status(500).json({ message: 'Failed to get users details' });
    }





    
    // return await this.UserModel
    //   .find(filterQuery)
    //   .skip(skip)
    //   .limit(limit)
    //   .exec();
  },
  

};

export default AdminService;
