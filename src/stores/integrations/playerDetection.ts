import { SupportedPlayer } from '@/services/backend/types';
import { createTauriStore } from '@tauri-store/zustand';
import { create } from 'zustand';

const DEFAULT_PLAYERS: SupportedPlayer[] = ['mpv', 'mpc-hc', 'mpc-be'];

type PlayerDetectionStore = {
  enabledPlayers: SupportedPlayer[];
  setPlayerEnabled: (player: SupportedPlayer, enabled: boolean) => void;
};

export const usePlayerDetectionStore = create<PlayerDetectionStore>((set) => ({
  enabledPlayers: DEFAULT_PLAYERS,
  setPlayerEnabled: (player, enabled) =>
    set((state) => {
      if (enabled && state.enabledPlayers.includes(player)) {
        return {};
      }

      if (!enabled && !state.enabledPlayers.includes(player)) {
        return {};
      }

      if (enabled) {
        return { enabledPlayers: [...state.enabledPlayers, player] };
      }

      return {
        enabledPlayers: state.enabledPlayers.filter(
          (enabledPlayer) => enabledPlayer !== player
        )
      };
    })
}));

export const tauriHandler = createTauriStore(
  'playerDetection',
  usePlayerDetectionStore,
  { autoStart: true }
);
