import { invoke } from '@tauri-apps/api/core';

import { DiscordPresenceRequest } from '../types';

export class DiscordRpcService {
  static async configure(clientId?: string): Promise<boolean> {
    return invoke<boolean>('configure_discord_rpc', { clientId });
  }

  static async setPresence(request: DiscordPresenceRequest): Promise<void> {
    return invoke('set_discord_presence', { request });
  }

  static async clearPresence(): Promise<void> {
    return invoke('clear_discord_presence');
  }
}
