import {
  isPermissionGranted,
  requestPermission,
  sendNotification
} from '@tauri-apps/plugin-notification';

import { NotificationOptions } from './types';

export class NotificationService {
  private static permissionGranted = false;

  constructor() {
    isPermissionGranted().then((granted) => {
      NotificationService.permissionGranted = granted;
    });
  }

  static async checkPermission(): Promise<void> {
    if (!this.permissionGranted) {
      const permission = await requestPermission();
      this.permissionGranted = permission === 'granted';
    }
  }

  async sendNotification({ title, body }: NotificationOptions): Promise<void> {
    await NotificationService.checkPermission();

    if (NotificationService.permissionGranted) {
      sendNotification({
        title,
        body
      });
    }
  }
}
