import { Checkbox, FormControlLabel, Stack } from '@mui/material';

import {
  defaultConfiguration,
  useConfigMenuStore
} from '@/stores/config/configMenu';
import Section from '../Section';

const DiscordSharingForm = () => {
  const configuration = useConfigMenuStore((state) => state.configuration);
  const setConfiguration = useConfigMenuStore(
    (state) => state.setConfiguration
  );

  const sharingConfig = configuration?.sharing ?? defaultConfiguration.sharing;

  const toggleEnableRichPresence = (checked: boolean) => {
    setConfiguration({
      ...configuration,
      sharing: {
        ...sharingConfig,
        enableRichPresence: checked
      }
    });
  };

  const toggleDisplayUsername = (checked: boolean) => {
    setConfiguration({
      ...configuration,
      sharing: {
        ...sharingConfig,
        displayUsernameInPresence: checked
      }
    });
  };

  const toggleDisplayTimeElapsed = (checked: boolean) => {
    setConfiguration({
      ...configuration,
      sharing: {
        ...sharingConfig,
        displayTimeElapsedInPresence: checked
      }
    });
  };

  const togglePreferAnimeTitle = (checked: boolean) => {
    setConfiguration({
      ...configuration,
      sharing: {
        ...sharingConfig,
        preferAnimeTitleInPresence: checked
      }
    });
  };

  return (
    <div className="flex flex-col gap-2">
      <FormControlLabel
        label="Update rich presence"
        checked={sharingConfig.enableRichPresence}
        onChange={(_, checked) => toggleEnableRichPresence(checked)}
        control={<Checkbox size="small" />}
      />

      <Section title="Options">
        <Stack>
          <FormControlLabel
            label="Display username in tooltip"
            checked={sharingConfig.displayUsernameInPresence}
            onChange={(_, checked) => toggleDisplayUsername(checked)}
            control={<Checkbox size="small" />}
          />

          <FormControlLabel
            label="Display elapsed time"
            checked={sharingConfig.displayTimeElapsedInPresence}
            onChange={(_, checked) => toggleDisplayTimeElapsed(checked)}
            control={<Checkbox size="small" />}
          />

          <FormControlLabel
            label="Prefer anime title over app name"
            checked={sharingConfig.preferAnimeTitleInPresence}
            onChange={(_, checked) => togglePreferAnimeTitle(checked)}
            control={<Checkbox size="small" />}
          />
        </Stack>
      </Section>
    </div>
  );
};

export default DiscordSharingForm;
