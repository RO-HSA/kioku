import { Alert, Divider, LinearProgress, Typography } from '@mui/material';

import Button from '@/components/ui/Button';
import { useAppUpdaterStore } from '@/stores/appUpdater';
import Section from '../Section';

const formatBytes = (bytes: number): string => {
  if (bytes <= 0) {
    return '0 B';
  }

  const units = ['B', 'KB', 'MB', 'GB'];
  const exponent = Math.min(
    Math.floor(Math.log(bytes) / Math.log(1024)),
    units.length - 1
  );
  const value = bytes / 1024 ** exponent;
  const precision = exponent === 0 ? 0 : 1;

  return `${value.toFixed(precision)} ${units[exponent]}`;
};

const formatDate = (isoDate: string | null): string | null => {
  if (!isoDate) {
    return null;
  }

  const parsedDate = new Date(isoDate);

  if (Number.isNaN(parsedDate.getTime())) {
    return null;
  }

  return parsedDate.toLocaleString();
};

const AppUpdateForm = () => {
  const status = useAppUpdaterStore((state) => state.status);
  const currentVersion = useAppUpdaterStore((state) => state.currentVersion);
  const availableVersion = useAppUpdaterStore(
    (state) => state.availableVersion
  );
  const installedVersion = useAppUpdaterStore(
    (state) => state.installedVersion
  );
  const releaseDate = useAppUpdaterStore((state) => state.releaseDate);
  const releaseNotes = useAppUpdaterStore((state) => state.releaseNotes);
  const restartRequired = useAppUpdaterStore((state) => state.restartRequired);
  const restartPromptVisible = useAppUpdaterStore(
    (state) => state.restartPromptVisible
  );
  const lastCheckedAt = useAppUpdaterStore((state) => state.lastCheckedAt);
  const downloadedBytes = useAppUpdaterStore((state) => state.downloadedBytes);
  const totalBytes = useAppUpdaterStore((state) => state.totalBytes);
  const error = useAppUpdaterStore((state) => state.error);
  const checkForUpdates = useAppUpdaterStore((state) => state.checkForUpdates);
  const downloadAndInstall = useAppUpdaterStore(
    (state) => state.downloadAndInstall
  );
  const restartNow = useAppUpdaterStore((state) => state.restartNow);
  const askRestartLater = useAppUpdaterStore((state) => state.askRestartLater);

  const isChecking = status === 'checking';
  const isDownloading = status === 'downloading' || status === 'installing';
  const isRestarting = status === 'restarting';
  const isBusy = isChecking || isDownloading || isRestarting;
  const canInstall = status === 'available' && !!availableVersion;

  const progressValue =
    totalBytes && totalBytes > 0
      ? Math.min((downloadedBytes / totalBytes) * 100, 100)
      : undefined;

  const progressLabel =
    status === 'installing'
      ? 'Finishing installation...'
      : totalBytes && totalBytes > 0
        ? `${formatBytes(downloadedBytes)} / ${formatBytes(totalBytes)}`
        : `${formatBytes(downloadedBytes)} downloaded`;

  const releaseDateLabel = formatDate(releaseDate);
  const lastCheckedLabel = formatDate(lastCheckedAt);

  return (
    <Section title="Desktop updater">
      <div className="flex flex-col gap-3">
        <div className="flex flex-col gap-0.5">
          <Typography variant="body2">
            Current version: <b>{currentVersion ?? '-'}</b>
          </Typography>
          {lastCheckedLabel && (
            <Typography variant="caption" className="text-text-secondary">
              Last check: {lastCheckedLabel}
            </Typography>
          )}
        </div>

        <div className="flex flex-wrap gap-2">
          <Button
            variant="secondary"
            size="small"
            isLoading={isChecking}
            isDisabled={isBusy}
            onClick={() => checkForUpdates()}>
            Check for updates
          </Button>

          <Button
            variant="primary"
            size="small"
            isLoading={isDownloading}
            isDisabled={!canInstall || isBusy}
            onClick={downloadAndInstall}>
            {status === 'installing' ? 'Installing...' : 'Download and install'}
          </Button>
        </div>

        {isDownloading && (
          <div className="flex flex-col gap-1">
            <LinearProgress
              variant={
                progressValue !== undefined ? 'determinate' : 'indeterminate'
              }
              value={progressValue}
            />
            <Typography variant="caption" className="text-text-secondary">
              {progressLabel}
            </Typography>
          </div>
        )}

        {status === 'available' && availableVersion && (
          <Alert severity="info">
            Version <b>{availableVersion}</b> is available.
            {releaseDateLabel ? ` Released on ${releaseDateLabel}.` : ''}
          </Alert>
        )}

        {status === 'upToDate' && (
          <Alert severity="success">You are on the latest version.</Alert>
        )}

        {status === 'installed' && (
          <Alert severity="success">
            Version <b>{installedVersion ?? currentVersion}</b> was installed.
            Restart the app to finish the update.
          </Alert>
        )}

        {restartRequired && restartPromptVisible && (
          <Alert severity="warning">
            <div className="flex flex-col gap-2">
              <Typography variant="body2">
                Do you want to restart the app now?
              </Typography>
              <div className="flex flex-wrap gap-2">
                <Button
                  variant="primary"
                  size="small"
                  isLoading={isRestarting}
                  isDisabled={isBusy}
                  onClick={restartNow}>
                  Restart now
                </Button>
                <Button
                  variant="ghost"
                  size="small"
                  isDisabled={isBusy}
                  onClick={askRestartLater}>
                  Later
                </Button>
              </div>
            </div>
          </Alert>
        )}

        {restartRequired && !restartPromptVisible && (
          <Alert severity="info">
            <div className="flex items-center justify-between gap-2">
              <Typography variant="body2">
                Update is installed and waiting for restart.
              </Typography>
              <Button
                variant="primary"
                size="small"
                isLoading={isRestarting}
                isDisabled={isBusy}
                onClick={restartNow}>
                Restart now
              </Button>
            </div>
          </Alert>
        )}

        {status === 'unsupported' && (
          <Alert severity="warning">
            Updater is available only when running the desktop Tauri app.
          </Alert>
        )}

        {status === 'error' && error && <Alert severity="error">{error}</Alert>}

        {status === 'available' && releaseNotes && (
          <>
            <Divider />
            <div className="flex flex-col gap-1">
              <Typography variant="overline">Release notes</Typography>
              <pre className="max-h-36 overflow-y-auto whitespace-pre-wrap rounded-sm border border-primary/25 bg-background-default p-2 text-xs">
                {releaseNotes}
              </pre>
            </div>
          </>
        )}
      </div>
    </Section>
  );
};

export default AppUpdateForm;
