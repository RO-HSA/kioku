import {
  Alert,
  Checkbox,
  FormControlLabel,
  FormGroup,
  Typography
} from '@mui/material';
import { useMemo, useState } from 'react';

import Button from '@/components/ui/Button';
import { PlayerDetectionService } from '@/services/backend/PlayerDetection';
import {
  AnimePlaybackDetection,
  SupportedPlayer
} from '@/services/backend/types';
import { usePlayerDetectionStore } from '@/stores/integrations/playerDetection';

const PLAYER_OPTIONS: { label: string; value: SupportedPlayer }[] = [
  { label: 'mpv', value: 'mpv' },
  { label: 'MPC-HC', value: 'mpc-hc' },
  { label: 'MPC-BE', value: 'mpc-be' }
];

const formatPlayerLabel = (player: SupportedPlayer): string => {
  switch (player) {
    case 'mpv':
      return 'mpv';
    case 'mpc-hc':
      return 'MPC-HC';
    case 'mpc-be':
      return 'MPC-BE';
    default:
      return player;
  }
};

const getErrorMessage = (error: unknown): string => {
  if (typeof error === 'string') {
    return error;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  if (error && typeof error === 'object') {
    const errorRecord = error as Record<string, unknown>;
    const message = errorRecord.message;

    if (typeof message === 'string' && message.trim().length > 0) {
      return message;
    }
  }

  return 'Unexpected error while detecting the current playback.';
};

const PlayerDetectionForm = () => {
  const enabledPlayers = usePlayerDetectionStore(
    (state) => state.enabledPlayers
  );
  const setPlayerEnabled = usePlayerDetectionStore(
    (state) => state.setPlayerEnabled
  );

  const [isDetecting, setIsDetecting] = useState(false);
  const [detection, setDetection] = useState<AnimePlaybackDetection | null>(
    null
  );
  const [status, setStatus] = useState<{
    severity: 'success' | 'info' | 'error';
    message: string;
  } | null>(null);

  const enabledPlayersSummary = useMemo(() => {
    if (!enabledPlayers.length) {
      return 'None selected';
    }

    return enabledPlayers.map(formatPlayerLabel).join(', ');
  }, [enabledPlayers]);

  const handleDetectClick = async () => {
    setStatus(null);
    setIsDetecting(true);

    try {
      const nextDetection = await PlayerDetectionService.detectPlayingAnime({
        players: enabledPlayers
      });
      setDetection(nextDetection);

      if (!nextDetection) {
        setStatus({
          severity: 'info',
          message:
            'No active supported player with a detectable anime filename was found.'
        });
        return;
      }

      setStatus({
        severity: 'success',
        message: `Detected "${nextDetection.animeTitle}"${
          nextDetection.episode ? ` episode ${nextDetection.episode}` : ''
        }.`
      });
    } catch (error) {
      setDetection(null);
      setStatus({
        severity: 'error',
        message: getErrorMessage(error)
      });
    } finally {
      setIsDetecting(false);
    }
  };

  return (
    <div className="flex flex-col gap-2">
      <Typography variant="body2">
        Detect anime title and episode from running players.
      </Typography>

      <FormGroup>
        {PLAYER_OPTIONS.map(({ label, value }) => (
          <FormControlLabel
            key={value}
            control={
              <Checkbox
                size="small"
                checked={enabledPlayers.includes(value)}
                onChange={(_, checked) => setPlayerEnabled(value, checked)}
              />
            }
            label={label}
          />
        ))}
      </FormGroup>

      <div className="flex flex-col gap-1 sm:flex-row sm:items-center sm:gap-2">
        <Button
          variant="secondary"
          size="small"
          isDisabled={!enabledPlayers.length || isDetecting}
          isLoading={isDetecting}
          onClick={handleDetectClick}>
          Detect now
        </Button>

        <Typography variant="caption">
          Enabled players: {enabledPlayersSummary}
        </Typography>
      </div>

      {status && <Alert severity={status.severity}>{status.message}</Alert>}

      {detection && (
        <div className="flex flex-col gap-1 rounded-md border border-primary/25 p-2 text-sm">
          <Typography variant="body2">
            <strong>Anime:</strong> {detection.animeTitle}
          </Typography>
          <Typography variant="body2">
            <strong>Episode:</strong> {detection.episode ?? 'Unknown'}
          </Typography>
          <Typography variant="body2">
            <strong>Player:</strong> {formatPlayerLabel(detection.player)}
          </Typography>
          <Typography variant="body2" className="break-all">
            <strong>Source:</strong> {detection.source}
          </Typography>
        </div>
      )}
    </div>
  );
};

export default PlayerDetectionForm;
