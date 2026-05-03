import { CustomEventName } from '@/types/Events';
import { useCallback, useEffect } from 'react';

interface UseCustomEventProps<T> {
  listen: boolean;
  onEvent: (payload: CustomEvent<T>) => void;
}

const useCustomEvent = <T>(
  eventName: CustomEventName,
  props?: UseCustomEventProps<T>
) => {
  const { listen, onEvent } = props || {};

  useEffect(() => {
    if (!listen || !onEvent) return;

    window.addEventListener(eventName, onEvent as EventListener);

    return () =>
      window.removeEventListener(eventName, onEvent as EventListener);
  }, [listen, onEvent]);

  const emit = useCallback(
    (payload: T) => {
      const event = new CustomEvent<T>(eventName, { detail: payload });

      window.dispatchEvent(event);
    },
    [eventName]
  );

  return { emit };
};

export default useCustomEvent;
