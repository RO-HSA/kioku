import { Alert, Checkbox, FormControlLabel, Stack } from '@mui/material';

import Section from '../Section';
import useGeneralConfigsForm from './hooks/useGeneralConfigsForm';

const GeneralConfigsForm = () => {
  const {
    enableAutoStartup,
    autoStartupError,
    isAutoStartupPending,
    startMinimized,
    checkForUpdates,
    minimizeToTray,
    closeToTray,
    toggleAutoStartup,
    toggleStartMinimized,
    toggleCheckForUpdates,
    toggleMinimizeToTray,
    toggleCloseToTray
  } = useGeneralConfigsForm();

  return (
    <>
      <Section title="Startup">
        <Stack>
          <FormControlLabel
            label="Start automatically on system startup"
            checked={enableAutoStartup}
            disabled={isAutoStartupPending}
            onChange={(_, checked) => {
              toggleAutoStartup(checked);
            }}
            control={<Checkbox size="small" />}
          />

          {autoStartupError && (
            <Alert severity="error">{autoStartupError}</Alert>
          )}

          <FormControlLabel
            label="Start minimized"
            checked={startMinimized}
            onChange={(_, checked) => toggleStartMinimized(checked)}
            control={<Checkbox size="small" />}
          />

          <FormControlLabel
            label="Check for updates automatically"
            checked={checkForUpdates}
            onChange={(_, checked) => toggleCheckForUpdates(checked)}
            control={<Checkbox size="small" />}
          />
        </Stack>
      </Section>

      <Section title="System tray">
        <Stack>
          <FormControlLabel
            label="Minimize to tray"
            checked={minimizeToTray}
            onChange={(_, checked) => toggleMinimizeToTray(checked)}
            control={<Checkbox size="small" />}
          />

          <FormControlLabel
            label="Close to tray"
            checked={closeToTray}
            onChange={(_, checked) => toggleCloseToTray(checked)}
            control={<Checkbox size="small" />}
          />
        </Stack>
      </Section>
    </>
  );
};

export default GeneralConfigsForm;
