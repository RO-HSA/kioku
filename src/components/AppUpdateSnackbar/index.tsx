import { Alert, Button, Snackbar } from '@mui/material';

import { useAppUpdaterStore } from '@/stores/appUpdater';
import { useConfigMenuStore } from '@/stores/config/configMenu';
import { ConfigMenuStep } from '@/types/Navigation';

const AppUpdateSnackbar = () => {
  const availableVersion = useAppUpdaterStore(
    (state) => state.availableVersion
  );
  const isOpen = useAppUpdaterStore(
    (state) => state.availableUpdateSnackbarOpen
  );
  const closeSnackbar = useAppUpdaterStore(
    (state) => state.closeAvailableUpdateSnackbar
  );
  const openConfigMenu = useConfigMenuStore((state) => state.openConfigMenu);
  const setStep = useConfigMenuStore((state) => state.setStep);
  const setSelectedTab = useConfigMenuStore((state) => state.setSelectedTab);

  const handleOpenUpdates = () => {
    setSelectedTab(0);
    setStep(ConfigMenuStep.UPDATES);
    openConfigMenu();
    closeSnackbar();
  };

  const handleClose = (_?: unknown, reason?: string) => {
    if (reason === 'clickaway') {
      return;
    }

    closeSnackbar();
  };

  return (
    <Snackbar
      open={isOpen && !!availableVersion}
      autoHideDuration={10000}
      anchorOrigin={{ vertical: 'bottom', horizontal: 'right' }}
      onClose={handleClose}>
      <Alert
        variant="filled"
        severity="info"
        onClose={() => closeSnackbar()}
        action={
          <Button color="inherit" size="small" onClick={handleOpenUpdates}>
            Open
          </Button>
        }>
        New version {availableVersion} is available.
      </Alert>
    </Snackbar>
  );
};

export default AppUpdateSnackbar;
