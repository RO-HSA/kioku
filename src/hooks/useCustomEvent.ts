import type { EventCallback } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useCallback, useEffect } from 'react';

import { CustomEventName } from '@/types/Events';

interface UseCustomEventProps<T> {
  listen: boolean;
  onEvent: EventCallback<T>;
}

const useCustomEvent = <T>(
  eventName: CustomEventName,
  props?: UseCustomEventProps<T>
) => {
  const { listen, onEvent } = props || {};

  const emit = useCallback(
    async (payload: T) => {
      await getCurrentWindow().emit(eventName, payload);
    },
    [eventName]
  );

  useEffect(() => {
    if (!listen || !onEvent) return;

    let unlisten: (() => void) | undefined;
    let isActive = true;

    getCurrentWindow()
      .listen(eventName, onEvent)
      .then((cleanup) => {
        if (!isActive) {
          cleanup();
          return;
        }

        unlisten = cleanup;
      })
      .catch((error) => {
        console.error(`Failed to listen to event: ${eventName}`, error);
      });

    return () => {
      isActive = false;
      unlisten?.();
    };
  }, [eventName, listen, onEvent]);

  return { emit };
};

export default useCustomEvent;
