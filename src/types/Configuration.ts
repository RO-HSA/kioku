import { SupportedPlayer } from '@/services/backend/types';

export interface DetectionConfig {
  playerDetectionEnabled: boolean;
  enabledPlayers: SupportedPlayer[];
}

export interface ConfigurationState {
  detection: DetectionConfig;
}
