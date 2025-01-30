import { Router } from 'express';
import { json } from 'body-parser';
import { ExamDto } from './exam.dto';
import { ExamsService } from './exam.service';

const router = Router();
const examsService = new ExamsService();

router.use(json());

// Add Exam
router.post('/', async (req, res) => {
  const examDto = new ExamDto(req.body);
  const result = await examsService.addExam(examDto);
  res.json(result);
});

export default router;
