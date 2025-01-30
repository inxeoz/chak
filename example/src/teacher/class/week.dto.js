import { DayDto } from './day.dto';

class WeekDto {
  constructor(days) {
    if (!Array.isArray(days)) {
      throw new Error('days must be an array');
    }
    this.days = days.map(day => new DayDto(day));
  }
}

export default { WeekDto };