import { Checkbox, FormControlLabel, Stack } from '@mui/material';
import { disable, enable } from '@tauri-apps/plugin-autostart';

import {
  defaultConfiguration,
  useConfigMenuStore
} from '@/stores/config/configMenu';
import Section from '../Section';

const GeneralConfigsForm = () => {
  const configuration = useConfigMenuStore((state) => state.configuration);
  const setConfiguration = useConfigMenuStore(
    (state) => state.setConfiguration
  );

  const toggleAutoStartup = async (enabled: boolean) => {
    setConfiguration({
      ...configuration,
      application: {
        ...configuration?.application,
        enableAutoStartup: enabled
      }
    });

    if (enabled) {
      await enable();
    } else {
      await disable();
    }
  };

  const toggleStartMinimized = (enabled: boolean) => {
    setConfiguration({
      ...configuration,
      application: {
        ...configuration?.application,
        startMinimized: enabled
      }
    });
  };

  const toggleCheckForUpdates = (enabled: boolean) => {
    setConfiguration({
      ...configuration,
      application: {
        ...configuration?.application,
        checkForUpdates: enabled
      }
    });
  };

  return (
    <Section title="Startup">
      <Stack>
        <FormControlLabel
          label="Start automatically on system startup"
          checked={
            configuration?.application?.enableAutoStartup ??
            defaultConfiguration.application.enableAutoStartup
          }
          onChange={(_, checked) => toggleAutoStartup(checked)}
          control={<Checkbox size="small" />}
        />

        <FormControlLabel
          label="Start minimized"
          checked={
            configuration?.application?.startMinimized ??
            defaultConfiguration.application.startMinimized
          }
          onChange={(_, checked) => toggleStartMinimized(checked)}
          control={<Checkbox size="small" />}
        />

        <FormControlLabel
          label="Check for updates automatically"
          checked={
            configuration?.application?.checkForUpdates ??
            defaultConfiguration.application.checkForUpdates
          }
          onChange={(_, checked) => toggleCheckForUpdates(checked)}
          control={<Checkbox size="small" />}
        />
      </Stack>
    </Section>
  );
};

export default GeneralConfigsForm;
