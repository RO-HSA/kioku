import { invoke, isTauri } from '@tauri-apps/api/core';

export class AutoStartService {
  static isRuntimeSupported(): boolean {
    return isTauri();
  }

  static async isEnabled(): Promise<boolean> {
    if (!this.isRuntimeSupported()) {
      return false;
    }

    return invoke<boolean>('is_auto_start_enabled');
  }

  static async setEnabled(enabled: boolean): Promise<boolean> {
    if (!this.isRuntimeSupported()) {
      return enabled;
    }

    return invoke<boolean>('set_auto_start_enabled', { enabled });
  }
}
