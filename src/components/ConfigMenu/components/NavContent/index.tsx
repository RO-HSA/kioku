import { PanelLeftClose } from 'lucide-react';

import NavButton from '@/components/NavButton';
import Button from '@/components/ui/Button';
import { useConfigMenuStore } from '@/stores/config/configMenu';
import { Divider } from '@mui/material';
import { FC } from 'react';
import { configMenuItems } from './config';

interface NavContentProps {
  onClickCloseButton?: () => void;
}

const NavContent: FC<NavContentProps> = ({ onClickCloseButton }) => {
  const menuStep = useConfigMenuStore((state) => state.step);

  return (
    <div className="flex flex-col gap-2">
      <Button
        variant="ghost"
        className="sm:hidden! max-w-6 self-end"
        size="small"
        onClick={onClickCloseButton}>
        <PanelLeftClose className="size-5" />
      </Button>

      <Divider className="sm:hidden" />

      <div className="flex flex-col">
        {configMenuItems.map(({ step, label, icon }) => (
          <NavButton
            key={step}
            className="rounded-sm"
            label={label}
            isActive={step === menuStep}
            isSidebarOpen
            Icon={icon}
          />
        ))}
      </div>
    </div>
  );
};

export default NavContent;
