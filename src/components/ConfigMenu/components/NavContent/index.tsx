import { PanelLeftClose } from 'lucide-react';

import NavButton from '@/components/NavButton';
import Button from '@/components/ui/Button';
import { useConfigMenuStore } from '@/stores/config/configMenu';
import { ConfigMenuStep } from '@/types/Navigation';
import { Divider } from '@mui/material';
import { FC } from 'react';
import { configMenuItems } from './config';

interface NavContentProps {
  onClickCloseButton?: () => void;
}

const NavContent: FC<NavContentProps> = ({ onClickCloseButton }) => {
  const menuStep = useConfigMenuStore((state) => state.step);
  const setStep = useConfigMenuStore((state) => state.setStep);
  const setSelectedTab = useConfigMenuStore((state) => state.setSelectedTab);

  const handleMenuItemClick = (step: ConfigMenuStep) => {
    setSelectedTab(0);
    setStep(step);
    if (onClickCloseButton) onClickCloseButton();
  };

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

      <div className="flex flex-col gap-1.5">
        {configMenuItems.map(({ step, label, icon }) => (
          <NavButton
            key={step}
            className="rounded-sm"
            label={label}
            isActive={step === menuStep}
            isSidebarOpen
            Icon={icon}
            onClick={() => handleMenuItemClick(step)}
          />
        ))}
      </div>
    </div>
  );
};

export default NavContent;
