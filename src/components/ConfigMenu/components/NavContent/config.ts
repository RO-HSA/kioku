import { AppWindow, Blocks, Share2, TvMinimalPlay } from 'lucide-react';

import { ConfigMenuStep, MenuItem } from '@/types/Navigation';

export const configMenuItems: MenuItem[] = [
  {
    label: 'Integrations',
    step: ConfigMenuStep.INTEGRATIONS,
    icon: Blocks
  },
  {
    label: 'Detection',
    step: ConfigMenuStep.DETECTION,
    icon: TvMinimalPlay
  },
  {
    label: 'Application',
    step: ConfigMenuStep.APPLICATION,
    icon: AppWindow
  },
  {
    label: 'Sharing',
    step: ConfigMenuStep.SHARING,
    icon: Share2
  }
];
