import { PlayerDetectionService } from '@/services/backend/PlayerDetection';
import { usePlayerDetectionStore } from '@/stores/playerDetection';
import { UnlistenFn } from '@tauri-apps/api/event';
import { useEffect } from 'react';

const usePlaybackObserverEvents = () => {
  const setEpisodeDetected = usePlayerDetectionStore(
    (state) => state.setEpisodeDetected
  );
  const setEpisodeClosed = usePlayerDetectionStore(
    (state) => state.setEpisodeClosed
  );

  useEffect(() => {
    let unlistenDetected: UnlistenFn | null = null;
    let unlistenClosed: UnlistenFn | null = null;

    const setupListeners = async () => {
      unlistenDetected = await PlayerDetectionService.listenEpisodeDetected(
        (detection) => {
          setEpisodeDetected(detection);
        }
      );

      unlistenClosed = await PlayerDetectionService.listenEpisodeClosed(
        (detection) => {
          setEpisodeClosed(detection);
        }
      );
    };

    setupListeners();

    return () => {
      if (unlistenDetected) {
        unlistenDetected();
      }

      if (unlistenClosed) {
        unlistenClosed();
      }
    };
  }, [setEpisodeClosed, setEpisodeDetected]);
};

export default usePlaybackObserverEvents;
