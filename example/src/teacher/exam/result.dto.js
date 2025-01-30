// DTO for results
class ResultDto {
  constructor(examName, result) {
    if (typeof examName !== 'string' || examName.trim() === '') {
      throw new Error('examName must be a non-empty string');
    }
    if (typeof result !== 'string' || result.trim() === '') {
      throw new Error('result must be a non-empty string');
    }
    this.examName = examName;
    this.result = result;
  }
}

export default ResultDto;