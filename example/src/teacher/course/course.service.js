
import CourseModel from './course.schema.js';
import fileRouter from '../../service/fileupload.controller.js';

const CoursesService = {


  async createCourse(value, userId,  res) {
    const {...CourseDto} = value;
    try {
      const course =await CourseModel({ ...CourseDto, createdBy_userId: userId });
      await course.save();  // Save once
      res.status(201).json(course);
    } catch (error) {
      console.error("Error uploading file:", error);
      res.status(500).json({ message: error.message });
    }
  },
  

  async updateCourse(value,courseId,  res) {
    try {
      const course =await CourseModel.findOne({ courseId }).exec();
      if (!course) {
        return res.status(404).json({ message: 'course does not exist' });
      }
      
      Object.assign(course, value); // Use Object.assign to update fields of the existing user
      await course.save();  // Save once
      res.status(201).json(course);
    } catch (error) {
      console.error("Error uploading file:", error);
      res.status(500).json({ message: error.message });
    }
  },
  

  async getCourses(skip, limit, courseSearchDTO) {
    const filterQuery = { ...courseSearchDTO };

    if (Object.keys(filterQuery).length === 0) {
      return await CourseModel
        .find()
        .sort({ createdAt: -1 })
        .limit(limit)
        .exec();
    }

    return await CourseModel
      .find(filterQuery)
      .skip(skip)
      .limit(limit)
      .exec();
  }
,
  async addCourseMaterial(courseId, files) {
    const course = await CourseModel.findOne({ courseId }).exec();
    if (!course) {
      throw new Error("Course not found");
    }

    try {
      for (const file of files) {
        const res = await this.uploadFile(file);
        course.courseMaterials.push(res.filename);
      }
    } catch (err) {
      console.error("Error uploading file:", err);
    }

    return course.save();
  },

  async downloadFile(filenames, res) {
    try {
      const req = { query: { filenames } };
      await this.fileRouter.handle(req, res);
    } catch (err) {
      console.error("Error downloading files:", err);
      res.status(500).send("Error downloading files");
    }
  },
  

  async uploadFile(file) {
    return new Promise((resolve, reject) => {
      const req = { file };
      const res = {
        status: (code) => ({
          send: (message) => {
            if (code === 200) {
              resolve({ filename: file.filename });
            } else {
              reject(new Error(message || "File upload failed"));
            }
          },
        }),
      };
      this.fileRouter.handle(req, res);
    });
  },
  
}

export default CoursesService;
