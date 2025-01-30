import { ScheduleDto } from './schedule.dto';

class DayDto {
  constructor(dayName, schedules) {
    if (typeof dayName !== 'string' || !dayName.trim()) {
      throw new Error('dayName must be a non-empty string');
    }
    if (!Array.isArray(schedules) || !schedules.every(schedule => schedule instanceof ScheduleDto)) {
      throw new Error('schedules must be an array of ScheduleDto instances');
    }
    this.dayName = dayName;
    this.schedules = schedules;
  }
}

export default { DayDto };