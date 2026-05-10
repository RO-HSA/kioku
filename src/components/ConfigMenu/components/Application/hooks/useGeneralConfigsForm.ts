import { disable, enable } from '@tauri-apps/plugin-autostart';

import {
  defaultConfiguration,
  useConfigMenuStore
} from '@/stores/config/configMenu';

const useGeneralConfigsForm = () => {
  const configuration = useConfigMenuStore((state) => state.configuration);
  const setConfiguration = useConfigMenuStore(
    (state) => state.setConfiguration
  );

  const toggleAutoStartup = async (enabled: boolean) => {
    if (enabled) {
      await enable();
    } else {
      await disable();
    }

    setConfiguration({
      ...configuration,
      application: {
        ...configuration?.application,
        enableAutoStartup: enabled
      }
    });
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

  const toggleMinimizeToTray = (enabled: boolean) => {
    setConfiguration({
      ...configuration,
      application: {
        ...configuration?.application,
        minimizeToTray: enabled
      }
    });
  };

  const toggleCloseToTray = (enabled: boolean) => {
    setConfiguration({
      ...configuration,
      application: { ...configuration?.application, closeToTray: enabled }
    });
  };

  return {
    enableAutoStartup:
      configuration?.application?.enableAutoStartup ??
      defaultConfiguration.application.enableAutoStartup,
    startMinimized:
      configuration?.application?.startMinimized ??
      defaultConfiguration.application.startMinimized,
    checkForUpdates:
      configuration?.application?.checkForUpdates ??
      defaultConfiguration.application.checkForUpdates,
    minimizeToTray:
      configuration?.application?.minimizeToTray ??
      defaultConfiguration.application.minimizeToTray,
    closeToTray:
      configuration?.application?.closeToTray ??
      defaultConfiguration.application.closeToTray,
    toggleAutoStartup,
    toggleStartMinimized,
    toggleCheckForUpdates,
    toggleMinimizeToTray,
    toggleCloseToTray
  };
};

export default useGeneralConfigsForm;
