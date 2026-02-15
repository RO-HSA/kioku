import { DownloadEvent } from '@tauri-apps/plugin-updater';
import { create } from 'zustand';

import {
  AppUpdaterService,
  AvailableUpdate
} from '@/services/backend/AppUpdater';
import { NotificationService } from '@/services/Notification';

type AppUpdateStatus =
  | 'idle'
  | 'checking'
  | 'upToDate'
  | 'available'
  | 'downloading'
  | 'installing'
  | 'installed'
  | 'restarting'
  | 'error'
  | 'unsupported';

interface CheckForUpdatesOptions {
  silent?: boolean;
  notifyWhenAvailable?: boolean;
}

interface AppUpdaterStore {
  hasBootstrapped: boolean;
  status: AppUpdateStatus;
  currentVersion: string | null;
  availableVersion: string | null;
  installedVersion: string | null;
  releaseDate: string | null;
  releaseNotes: string | null;
  restartRequired: boolean;
  restartPromptVisible: boolean;
  lastCheckedAt: string | null;
  downloadedBytes: number;
  totalBytes: number | null;
  error: string | null;
  bootstrap: () => Promise<void>;
  refreshCurrentVersion: () => Promise<void>;
  checkForUpdates: (options?: CheckForUpdatesOptions) => Promise<void>;
  downloadAndInstall: () => Promise<void>;
  restartNow: () => Promise<void>;
  askRestartLater: () => void;
}

const notification = new NotificationService();

const getErrorMessage = (error: unknown): string => {
  if (error instanceof Error) {
    return error.message;
  }

  return String(error);
};

const isUpdaterUnavailableError = (message: string): boolean => {
  const normalized = message.toLowerCase();
  return (
    normalized.includes('plugin:updater') ||
    normalized.includes('plugin not found') ||
    normalized.includes('unknown plugin') ||
    normalized.includes('not allowed')
  );
};

const sendAvailableNotification = (update: AvailableUpdate): void => {
  void notification.sendNotification({
    title: 'Update available',
    body: `Version ${update.version} is ready to install.`
  });
};

const sendInstalledNotification = (version: string): void => {
  void notification.sendNotification({
    title: 'Update installed',
    body: `Version ${version} was installed. Restart the app to finish.`
  });
};

export const useAppUpdaterStore = create<AppUpdaterStore>((set, get) => ({
  hasBootstrapped: false,
  status: 'idle',
  currentVersion: null,
  availableVersion: null,
  installedVersion: null,
  releaseDate: null,
  releaseNotes: null,
  restartRequired: false,
  restartPromptVisible: false,
  lastCheckedAt: null,
  downloadedBytes: 0,
  totalBytes: null,
  error: null,
  bootstrap: async () => {
    if (get().hasBootstrapped) {
      return;
    }

    set({ hasBootstrapped: true });
    await get().refreshCurrentVersion();
    await get().checkForUpdates({ silent: true, notifyWhenAvailable: true });
  },
  refreshCurrentVersion: async () => {
    const currentVersion = await AppUpdaterService.getCurrentVersion();

    if (currentVersion) {
      set({ currentVersion });
    }
  },
  checkForUpdates: async ({
    silent = false,
    notifyWhenAvailable = false
  }: CheckForUpdatesOptions = {}) => {
    if (!AppUpdaterService.isRuntimeSupported()) {
      set({
        status: 'unsupported',
        error: null
      });
      return;
    }

    const currentStatus = get().status;

    if (
      currentStatus === 'checking' ||
      currentStatus === 'downloading' ||
      currentStatus === 'installing' ||
      currentStatus === 'restarting'
    ) {
      return;
    }

    if (!silent) {
      set({
        status: 'checking',
        error: null
      });
    } else {
      set({ error: null });
    }

    try {
      const currentVersion = await AppUpdaterService.getCurrentVersion();
      const update = await AppUpdaterService.checkForUpdates();
      const lastCheckedAt = new Date().toISOString();

      if (!update) {
        set((state) => ({
          currentVersion: currentVersion ?? state.currentVersion,
          availableVersion: null,
          releaseDate: null,
          releaseNotes: null,
          lastCheckedAt,
          downloadedBytes: 0,
          totalBytes: null,
          status: 'upToDate',
          error: null
        }));
        return;
      }

      if (notifyWhenAvailable) {
        sendAvailableNotification(update);
      }

      set({
        currentVersion: update.currentVersion ?? currentVersion,
        availableVersion: update.version,
        releaseDate: update.date ?? null,
        releaseNotes: update.body ?? null,
        lastCheckedAt,
        downloadedBytes: 0,
        totalBytes: null,
        status: 'available',
        error: null
      });
    } catch (error) {
      const message = getErrorMessage(error);

      if (isUpdaterUnavailableError(message)) {
        set({
          status: 'unsupported',
          error: null
        });
        return;
      }

      set({
        status: 'error',
        error: message
      });
    }
  },
  downloadAndInstall: async () => {
    if (!AppUpdaterService.isRuntimeSupported()) {
      set({
        status: 'unsupported',
        error: null
      });
      return;
    }

    const currentStatus = get().status;

    if (
      currentStatus === 'checking' ||
      currentStatus === 'downloading' ||
      currentStatus === 'installing' ||
      currentStatus === 'restarting'
    ) {
      return;
    }

    set({
      status: 'downloading',
      downloadedBytes: 0,
      totalBytes: null,
      error: null
    });

    try {
      await AppUpdaterService.downloadAndInstall((event: DownloadEvent) => {
        if (event.event === 'Started') {
          set({
            status: 'downloading',
            downloadedBytes: 0,
            totalBytes: event.data.contentLength ?? null
          });
          return;
        }

        if (event.event === 'Progress') {
          set((state) => ({
            downloadedBytes: state.downloadedBytes + event.data.chunkLength
          }));
          return;
        }

        if (event.event === 'Finished') {
          set({ status: 'installing' });
        }
      });

      const installedVersion = get().availableVersion;

      if (installedVersion) {
        sendInstalledNotification(installedVersion);
      }

      set((state) => ({
        status: 'installed',
        installedVersion: installedVersion ?? state.installedVersion,
        availableVersion: null,
        restartRequired: true,
        restartPromptVisible: true,
        downloadedBytes: 0,
        totalBytes: null,
        error: null
      }));
    } catch (error) {
      set({
        status: 'error',
        error: getErrorMessage(error)
      });
    }
  },
  restartNow: async () => {
    if (!AppUpdaterService.isRuntimeSupported()) {
      set({
        status: 'unsupported',
        error: null
      });
      return;
    }

    const currentStatus = get().status;
    if (currentStatus === 'restarting') {
      return;
    }

    set({
      status: 'restarting',
      error: null
    });

    try {
      await AppUpdaterService.relaunchApp();
    } catch (error) {
      set((state) => ({
        status: 'installed',
        restartRequired: state.restartRequired,
        restartPromptVisible: true,
        error: getErrorMessage(error)
      }));
    }
  },
  askRestartLater: () => {
    set({
      restartPromptVisible: false
    });
  }
}));
