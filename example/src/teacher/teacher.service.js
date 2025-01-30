import UserModel from "../user/user.schema.js";
import CoursesService from "./course/course.service.js";

const TeacherService = {

  async getTeacherDetails(userId, res) {
    try {
      const user = await UserModel.findOne({ userId }).exec();
      res.json(user.toTeacherLevelAccess());
    } catch (error) {
      res.status(500).json({ message: 'Failed to get user' });
    }
  },

};

export default TeacherService;
