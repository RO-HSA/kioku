import { LucideProps } from 'lucide-react';
import { FC, ForwardRefExoticComponent, RefAttributes } from 'react';

import { cn } from '@/lib/utils';

interface NavButtonProps {
  isSidebarOpen?: boolean;
  isActive: boolean;
  Icon: ForwardRefExoticComponent<
    Omit<LucideProps, 'ref'> & RefAttributes<SVGSVGElement>
  >;
  className?: string;
  label: string;
  onClick?: () => void;
}

const NavButton: FC<NavButtonProps> = ({
  isSidebarOpen,
  isActive,
  Icon,
  label,
  className,
  onClick
}) => {
  return (
    <button
      type="button"
      onClick={onClick}
      aria-current={isActive ? 'page' : undefined}
      className={cn(
        'flex w-full items-center justify-center gap-3 cursor-pointer rounded-xl px-3 py-2 text-left text-sm font-medium transition-[colors, gap] duration-200 ease-in-out',
        className,
        isActive
          ? 'bg-primary/10 text-primary'
          : 'text-text-secondary hover:bg-primary/5 hover:text-text-primary',
        !isSidebarOpen && 'justify-center gap-0'
      )}>
      <Icon className="size-5 shrink-0 justify-self-center" />

      <span
        className={cn(
          'truncate w-full opacity-100 transition-opacity duration-300 ease-in-out',
          !isSidebarOpen && 'opacity-0'
        )}>
        {label}
      </span>
    </button>
  );
};

export default NavButton;
