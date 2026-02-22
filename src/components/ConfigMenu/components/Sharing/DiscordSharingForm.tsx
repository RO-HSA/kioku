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

  const toggleEnableRichPresence = (checked: boolean) => {
    setConfiguration({
      ...configuration,
      sharing: {
        ...configuration.sharing,
        enableRichPresence: checked
      }
    });
  };

  const toggleDisplayUsername = (checked: boolean) => {
    setConfiguration({
      ...configuration,
      sharing: {
        ...configuration.sharing,
        displayUsernameInPresence: checked
      }
    });
  };

  const toggleDisplayTimeElapsed = (checked: boolean) => {
    setConfiguration({
      ...configuration,
      sharing: {
        ...configuration.sharing,
        displayTimeElapsedInPresence: checked
      }
    });
  };

  const togglePreferAnimeTitle = (checked: boolean) => {
    setConfiguration({
      ...configuration,
      sharing: {
        ...configuration.sharing,
        preferAnimeTitleInPresence: checked
      }
    });
  };

  return (
    <div className="flex flex-col gap-2">
      <FormControlLabel
        label="Update rich presence"
        checked={
          configuration?.sharing?.enableRichPresence ??
          defaultConfiguration.sharing.enableRichPresence
        }
        onChange={(_, checked) => toggleEnableRichPresence(checked)}
        control={<Checkbox size="small" />}
      />

      <Section title="Options">
        <Stack>
          <FormControlLabel
            label="Display username in tooltip"
            checked={
              configuration?.sharing?.displayUsernameInPresence ??
              defaultConfiguration.sharing.displayUsernameInPresence
            }
            onChange={(_, checked) => toggleDisplayUsername(checked)}
            control={<Checkbox size="small" />}
          />

          <FormControlLabel
            label="Display elapsed time"
            checked={
              configuration?.sharing?.displayTimeElapsedInPresence ??
              defaultConfiguration.sharing.displayTimeElapsedInPresence
            }
            onChange={(_, checked) => toggleDisplayTimeElapsed(checked)}
            control={<Checkbox size="small" />}
          />

          <FormControlLabel
            label="Prefer anime title over app name"
            checked={
              configuration?.sharing?.preferAnimeTitleInPresence ??
              defaultConfiguration.sharing.preferAnimeTitleInPresence
            }
            onChange={(_, checked) => togglePreferAnimeTitle(checked)}
            control={<Checkbox size="small" />}
          />
        </Stack>
      </Section>
    </div>
  );
};

export default DiscordSharingForm;
