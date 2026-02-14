import { LucideProps } from 'lucide-react';
import { FC, ForwardRefExoticComponent, RefAttributes } from 'react';

import { Tooltip } from '@mui/material';
import { NavLink } from 'react-router';
import InternalButton from './InternalButton';

interface NavButtonProps {
  isSidebarOpen?: boolean;
  Icon: ForwardRefExoticComponent<
    Omit<LucideProps, 'ref'> & RefAttributes<SVGSVGElement>
  >;
  className?: string;
  label: string;
  link?: string;
  isActive: boolean;
  isDisabled: boolean;
  onClick?: () => void;
}

const NavButton: FC<NavButtonProps> = ({
  isSidebarOpen,
  Icon,
  label,
  className,
  link,
  isActive,
  isDisabled,
  onClick
}) => {
  return (
    <Tooltip
      title={label}
      placement="right"
      disableHoverListener={isSidebarOpen}>
      {link ? (
        <NavLink to={link}>
          <InternalButton
            isSidebarOpen={isSidebarOpen}
            className={className}
            Icon={Icon}
            label={label}
            isActive={isActive}
            isDisabled={isDisabled}
            onClick={onClick}
          />
        </NavLink>
      ) : (
        <InternalButton
          isSidebarOpen={isSidebarOpen}
          className={className}
          Icon={Icon}
          label={label}
          isActive={isActive}
          isDisabled={isDisabled}
          onClick={onClick}
        />
      )}
    </Tooltip>
  );
};

export default NavButton;
