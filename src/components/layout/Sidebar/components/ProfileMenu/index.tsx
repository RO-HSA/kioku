import {
  Avatar,
  Box,
  Divider,
  ListItemIcon,
  ListItemText,
  MenuItem,
  MenuList,
  Popover,
  Typography
} from '@mui/material';
import { ChevronUp } from 'lucide-react';

import { cn } from '@/lib/utils';
import ListTypeMenu from './ListTypeMenu';
import SwitchAccountMenu from './SwitchAccountMenu';
import useProfileMenu from './useProfileMenu';

const ProfileMenu = () => {
  const {
    isOpen,
    mainPopoverEl,
    mainPopoverOpen,
    switchListEl,
    switchListOpen,
    switchAccountEl,
    switchAccountOpen,
    profileImage,
    providerName,
    username,
    selectedListType,
    menuItems,
    connectedAccounts,
    currentListTypeIcon,
    handleOpenMainPopover,
    handleCloseMainPopover,
    handleSwitchListType,
    handleCloseSwitchListPopover,
    handleSwitchAccount,
    handleCloseSwitchAccountPopover
  } = useProfileMenu();

  return (
    <>
      <div
        className={cn(
          'flex items-center justify-between hover:bg-action-hover rounded-lg transition-all duration-300 ease-in-out cursor-pointer',
          isOpen ? 'p-2' : 'p-1'
        )}
        aria-expanded={mainPopoverOpen ? 'true' : undefined}
        onClick={handleOpenMainPopover}>
        <div
          className={cn(
            'flex gap-3 items-center select-none transition-all duration-300 ease-in-out',
            !isOpen && 'justify-center gap-0'
          )}>
          <Avatar
            className={cn(
              'shrink-0 transition-all duration-300 ease-in-out',
              isOpen ? 'size-10!' : 'size-7.5!'
            )}
            sx={{ borderRadius: '20%' }}
            src={profileImage ?? undefined}
          />

          <Box
            className={cn(
              'truncate w-full opacity-100 transition-opacity duration-300 ease-in-out',
              !isOpen && 'opacity-0 w-0 h-0'
            )}
            gap={2}>
            <Typography variant="body1" fontWeight="bold">
              {username}
            </Typography>
            <Typography variant="body2" color="text.secondary">
              <span className="flex gap-2 items-center">
                <span>{providerName}</span>
                <span>{currentListTypeIcon}</span>
              </span>
            </Typography>
          </Box>
        </div>

        <ChevronUp
          className={cn(
            'text-Chip-defaultBorder transition-all duration-300 ease-in-out opacity-100',
            !isOpen && 'opacity-0'
          )}
          size={19}
        />
      </div>

      <Popover
        anchorEl={mainPopoverEl}
        open={mainPopoverOpen}
        onClose={handleCloseMainPopover}
        anchorOrigin={{ vertical: 'top', horizontal: 'left' }}
        transformOrigin={{ vertical: 'bottom', horizontal: 'left' }}>
        <MenuList>
          {menuItems.flatMap(
            ({ label, icon, disabled, renderDivider, handleClick }) => {
              const items = [
                <MenuItem key={label} onClick={handleClick} disabled={disabled}>
                  <ListItemIcon>{icon}</ListItemIcon>
                  <ListItemText>{label}</ListItemText>
                </MenuItem>
              ];

              if (renderDivider) {
                items.push(<Divider key={`${label}-divider`} />);
              }

              return items;
            }
          )}

          <ListTypeMenu
            anchorEl={switchListEl}
            open={switchListOpen}
            selectedListType={selectedListType}
            onClose={handleCloseSwitchListPopover}
            onSwitchListType={handleSwitchListType}
          />

          <SwitchAccountMenu
            anchorEl={switchAccountEl}
            open={switchAccountOpen}
            connectedAccounts={connectedAccounts}
            onClose={handleCloseSwitchAccountPopover}
            onSwitchAccount={handleSwitchAccount}
          />
        </MenuList>
      </Popover>
    </>
  );
};

export default ProfileMenu;
