import { Blocks } from 'lucide-react';

import { ConfigMenuStep, MenuItem } from '@/types/Navigation';

export const configMenuItems: MenuItem[] = [
  {
    label: 'Integrations',
    step: ConfigMenuStep.INTEGRATIONS,
    icon: Blocks
  }
];
