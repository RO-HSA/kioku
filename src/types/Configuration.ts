import { SupportedPlayer } from '@/services/backend/types';

export interface DetectionConfig {
  playerDetectionEnabled: boolean;
  enabledPlayers: SupportedPlayer[];
}

export interface ApplicationConfig {
  enableAutoStartup: boolean;
  startMinimized: boolean;
}

export interface ConfigurationState {
  detection: DetectionConfig;
  application: ApplicationConfig;
}
