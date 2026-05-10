import { AutoStartService } from '@/services/backend/AutoStart';
import { useState } from 'react';

import {
  defaultConfiguration,
  useConfigMenuStore
} from '@/stores/config/configMenu';

const useGeneralConfigsForm = () => {
  const configuration = useConfigMenuStore((state) => state.configuration);
  const setConfiguration = useConfigMenuStore(
    (state) => state.setConfiguration
  );
  const [autoStartupError, setAutoStartupError] = useState<string | null>(null);
  const [isAutoStartupPending, setIsAutoStartupPending] = useState(false);

  const toggleAutoStartup = async (enabled: boolean) => {
    if (isAutoStartupPending) {
      return;
    }

    setIsAutoStartupPending(true);
    setAutoStartupError(null);

    try {
      const result = await AutoStartService.setEnabled(enabled);

      if (!result.ok) {
        setAutoStartupError(result.error);
        return;
      }

      setConfiguration({
        ...configuration,
        application: {
          ...configuration?.application,
          enableAutoStartup: result.enabled
        }
      });
    } finally {
      setIsAutoStartupPending(false);
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
    autoStartupError,
    isAutoStartupPending,
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
