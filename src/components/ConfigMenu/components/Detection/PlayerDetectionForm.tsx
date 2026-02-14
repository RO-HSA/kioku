import {
  Checkbox,
  Divider,
  FormControl,
  FormControlLabel,
  Switch,
  Typography
} from '@mui/material';

import { SupportedPlayer } from '@/services/backend/types';
import { useConfigMenuStore } from '@/stores/config/configMenu';

const PLAYER_OPTIONS: { label: string; value: SupportedPlayer }[] = [
  { label: 'mpv', value: 'mpv' },
  { label: 'MPC-HC', value: 'mpc-hc' },
  { label: 'MPC-BE', value: 'mpc-be' }
];

const PlayerDetectionForm = () => {
  const configuration = useConfigMenuStore((state) => state.configuration);
  const setConfiguration = useConfigMenuStore(
    (state) => state.setConfiguration
  );

  const { playerDetectionEnabled, enabledPlayers } = configuration.detection;

  const setAutoDetectionEnabled = (enabled: boolean) => {
    setConfiguration({
      ...configuration,
      detection: {
        ...configuration.detection,
        playerDetectionEnabled: enabled
      }
    });
  };

  const setPlayerEnabled = (player: SupportedPlayer, enabled: boolean) => {
    const newEnabledPlayers = enabled
      ? [...enabledPlayers, player]
      : enabledPlayers.filter((p) => p !== player);

    setConfiguration({
      ...configuration,
      detection: {
        ...configuration.detection,
        enabledPlayers: newEnabledPlayers
      }
    });
  };

  const toggleAllPlayers = (enabled: boolean) => {
    setConfiguration({
      ...configuration,
      detection: {
        ...configuration.detection,
        enabledPlayers: enabled
          ? PLAYER_OPTIONS.map((option) => option.value)
          : []
      }
    });
  };

  return (
    <div className="flex flex-col gap-2">
      <div className="pl-2">
        <FormControlLabel
          label="Enable player detection"
          control={
            <Switch
              size="small"
              checked={playerDetectionEnabled}
              onChange={(_, checked) => setAutoDetectionEnabled(checked)}
            />
          }
        />
      </div>

      <div className="border border-primary/25 p-2 rounded-sm max-h-86.5 overflow-hidden">
        <FormControl className="w-full">
          <FormControlLabel
            label="Enable all"
            control={
              <Checkbox
                size="small"
                checked={enabledPlayers.length === PLAYER_OPTIONS.length}
                indeterminate={
                  enabledPlayers.length > 0 &&
                  enabledPlayers.length < PLAYER_OPTIONS.length
                }
                onChange={(_, checked) => toggleAllPlayers(checked)}
              />
            }
          />

          <Typography variant="caption" className="text-text-disabled">
            Select only the players you actually use, disable the others.
          </Typography>

          <Divider />

          <div className="flex flex-col max-h-70 overflow-y-auto">
            {PLAYER_OPTIONS.map(({ label, value }) => (
              <div>
                <FormControlLabel
                  key={value}
                  control={
                    <Checkbox
                      size="small"
                      checked={enabledPlayers.includes(value)}
                      onChange={(_, checked) =>
                        setPlayerEnabled(value, checked)
                      }
                    />
                  }
                  label={label}
                />
              </div>
            ))}
          </div>
        </FormControl>
      </div>
    </div>
  );
};

export default PlayerDetectionForm;
