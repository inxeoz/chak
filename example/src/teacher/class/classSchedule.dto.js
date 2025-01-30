import { WeekDto } from './week.dto';

// DTO for the class schedule containing weeks
class ClassScheduleDto {
  constructor(thisWeek, nextWeek, previousWeek) {
    if (!(thisWeek instanceof WeekDto) || !(nextWeek instanceof WeekDto) || !(previousWeek instanceof WeekDto)) {
      throw new Error('Invalid type for week properties');
    }
    this.thisWeek = thisWeek;
    this.nextWeek = nextWeek;
    this.previousWeek = previousWeek;
  }
}

export default { ClassScheduleDto };
