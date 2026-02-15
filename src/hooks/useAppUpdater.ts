import { useEffect } from 'react';

import { useAppUpdaterStore } from '@/stores/appUpdater';

const useAppUpdater = () => {
  const bootstrap = useAppUpdaterStore((state) => state.bootstrap);

  useEffect(() => {
    bootstrap();
  }, [bootstrap]);
};

export default useAppUpdater;
