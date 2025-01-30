const { IsArray, IsDateString, IsNotEmpty, IsString, IsOptional } = require('class-validator');

class AssignmentDto {
  constructor(subjectName, dueDate, result, shareLink) {
    this.subjectName = subjectName;
    this.dueDate = dueDate;
    this.result = result;
    this.shareLink = shareLink;
  }
}

module.exports = AssignmentDto;
