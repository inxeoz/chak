
import mongoose from 'mongoose';
import { ResultDto } from './result.dto';
import { Result } from './result.schema';

export class ResultService {
    constructor() {
        this.ResultModel = mongoose.model(Result.name, Result.schema);
    }

    async addResult(ResultDto) {
        const result = new this.ResultModel(ResultDto);
        return await result.save();
    }
}