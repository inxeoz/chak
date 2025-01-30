class ScheduleDto {
  constructor(educator, time, subjectName) {
    if (typeof educator !== 'string' || educator.trim() === '') {
      throw new Error('Educator must be a non-empty string');
    }
    if (typeof time !== 'string' || time.trim() === '') {
      throw new Error('Time must be a non-empty string');
    }
    if (typeof subjectName !== 'string' || subjectName.trim() === '') {
      throw new Error('Subject name must be a non-empty string');
    }

    this.educator = educator;
    this.time = time;
    this.subjectName = subjectName;
  }
}

export default ScheduleDto;