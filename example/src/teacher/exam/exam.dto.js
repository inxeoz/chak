class ExamDto {
  constructor(examName, date, time) {
    if (typeof examName !== 'string' || !examName.trim()) {
      throw new Error('examName must be a non-empty string');
    }
    if (isNaN(Date.parse(date))) {
      throw new Error('date must be a valid ISO 8601 date string');
    }
    if (typeof time !== 'string' || !time.trim()) {
      throw new Error('time must be a non-empty string in HH:mm format');
    }

    this.examName = examName;
    this.date = date;
    this.time = time;
  }
}

export default ExamDto;