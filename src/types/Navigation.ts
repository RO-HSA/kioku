import { LucideProps } from 'lucide-react';
import { ForwardRefExoticComponent, RefAttributes } from 'react';

export interface INavLink {
  label: string;
  icon: ForwardRefExoticComponent<
    Omit<LucideProps, 'ref'> & RefAttributes<SVGSVGElement>
  >;
  link: string;
}

export enum ConfigMenuStep {
  INTEGRATIONS = 'INTEGRATIONS',
  DETECTION = 'DETECTION',
  UPDATES = 'UPDATES'
}

export interface MenuItem {
  label: string;
  step: ConfigMenuStep;
  icon: ForwardRefExoticComponent<
    Omit<LucideProps, 'ref'> & RefAttributes<SVGSVGElement>
  >;
}
