import { http } from '../utils/request';
import type { ApiResponse } from '../types';

export interface BingDailyImage {
  url: string;
  copyright: string;
  copyright_link?: string;
}

export const bingApi = {
  getDailyImage: () => {
    return http.get<ApiResponse<BingDailyImage>>('/bing/daily-image');
  },
};