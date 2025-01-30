import express from 'express';
import { json } from 'body-parser';
import AssignmentsService from './assignments-exams.service';
import AssignmentDto from './assignment.dto';

const app = express();
app.use(json());

const assignmentsService = new AssignmentsService();

app.post('/assignment', async (req, res) => {
  const assignmentDto = new AssignmentDto(req.body);
  const result = await assignmentsService.addAssignment(assignmentDto);
  res.json(result);
});
