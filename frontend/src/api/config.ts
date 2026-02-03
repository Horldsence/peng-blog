import { http } from '../utils/request';
import type {
  Config,
  UpdateConfigRequest,
  ApiResponseV2,
} from '../types';

export const configApi = {
  /**
   * Get current configuration
   * Requires admin permission
   */
  getConfig: () => {
    return http.get<ApiResponseV2<Config>>('/config');
  },

  /**
   * Update configuration
   * Requires admin permission
   */
  updateConfig: (data: UpdateConfigRequest) => {
    return http.patch<ApiResponseV2<Config>>('/config', data);
  },
};

export default configApi;