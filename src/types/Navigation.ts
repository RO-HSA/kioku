import { LucideProps } from 'lucide-react';
import { ForwardRefExoticComponent, RefAttributes } from 'react';

export enum SidebarNavigationStep {
  ANIME_LIST = 'ANIME_LIST',
  SETTINGS = 'SETTINGS'
}

export enum ConfigMenuStep {
  INTEGRATIONS = 'INTEGRATIONS'
}

export interface MenuItem {
  label: string;
  step: ConfigMenuStep;
  icon: ForwardRefExoticComponent<
    Omit<LucideProps, 'ref'> & RefAttributes<SVGSVGElement>
  >;
}
