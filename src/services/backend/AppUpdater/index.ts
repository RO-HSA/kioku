import { getVersion } from '@tauri-apps/api/app';
import { isTauri } from '@tauri-apps/api/core';
import { relaunch } from '@tauri-apps/plugin-process';
import { check, DownloadEvent, Update } from '@tauri-apps/plugin-updater';

export interface AvailableUpdate {
  currentVersion: string;
  version: string;
  date?: string;
  body?: string;
}

export class AppUpdaterService {
  private static pendingUpdate: Update | null = null;

  static isRuntimeSupported(): boolean {
    return isTauri();
  }

  static async getCurrentVersion(): Promise<string | null> {
    if (!this.isRuntimeSupported()) {
      return null;
    }

    return getVersion();
  }

  static async checkForUpdates(): Promise<AvailableUpdate | null> {
    if (!this.isRuntimeSupported()) {
      return null;
    }

    const [currentVersion, update] = await Promise.all([getVersion(), check()]);

    if (!update) {
      await this.clearPendingUpdate();
      return null;
    }

    await this.clearPendingUpdate();
    this.pendingUpdate = update;

    return {
      currentVersion,
      version: update.version,
      date: update.date,
      body: update.body
    };
  }

  static async downloadAndInstall(
    onEvent?: (event: DownloadEvent) => void
  ): Promise<void> {
    if (!this.pendingUpdate) {
      throw new Error('No pending update found. Check for updates first.');
    }

    try {
      await this.pendingUpdate.downloadAndInstall(onEvent);
    } finally {
      await this.clearPendingUpdate();
    }
  }

  static async relaunchApp(): Promise<void> {
    if (!this.isRuntimeSupported()) {
      throw new Error('App relaunch is only available in Tauri runtime.');
    }

    await relaunch();
  }

  private static async clearPendingUpdate(): Promise<void> {
    if (!this.pendingUpdate) {
      return;
    }

    try {
      await this.pendingUpdate.close();
    } catch {
      this.pendingUpdate = null;
      return;
    }

    this.pendingUpdate = null;
  }
}
