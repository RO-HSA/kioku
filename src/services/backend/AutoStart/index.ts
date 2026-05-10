import { invoke, isTauri } from '@tauri-apps/api/core';

export type AutoStartSetEnabledResult =
  | {
      ok: true;
      enabled: boolean;
    }
  | {
      ok: false;
      error: string;
    };

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

  static async setEnabled(
    enabled: boolean
  ): Promise<AutoStartSetEnabledResult> {
    if (!this.isRuntimeSupported()) {
      return {
        ok: false,
        error:
          'Auto start is available only when running the desktop Tauri app.'
      };
    }

    try {
      const nextEnabled = await invoke<boolean>('set_auto_start_enabled', {
        enabled
      });

      return {
        ok: true,
        enabled: nextEnabled
      };
    } catch (error) {
      return {
        ok: false,
        error: this.getErrorMessage(error)
      };
    }
  }

  private static getErrorMessage(error: unknown): string {
    if (error instanceof Error) {
      return error.message;
    }

    return String(error);
  }
}
