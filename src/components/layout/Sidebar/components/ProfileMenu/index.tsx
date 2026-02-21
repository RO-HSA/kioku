import {
  Avatar,
  Box,
  Divider,
  ListItemIcon,
  ListItemText,
  Menu,
  MenuItem,
  MenuList,
  Popover,
  Typography
} from '@mui/material';
import { ArrowRightToLine, ChevronUp } from 'lucide-react';

import { cn } from '@/lib/utils';
import { useSidebarStore } from '@/stores/sidebar/sidebar';
import { mapProviderToName } from '@/utils/provider';
import useProfileMenu from './useProfileMenu';

const ProfileMenu = () => {
  const {
    mainPopoverEl,
    mainPopoverOpen,
    switchAccountEl,
    switchAccountOpen,
    profileImage,
    providerName,
    username,
    menuItems,
    connectedAccounts,
    handleOpenMainPopover,
    handleCloseMainPopover,
    handleSwitchAccount,
    handleCloseSwitchAccountPopover
  } = useProfileMenu();

  const isOpen = useSidebarStore((state) => state.isOpen);

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
              {providerName}
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

          <Menu
            anchorEl={switchAccountEl}
            open={switchAccountOpen}
            onClose={handleCloseSwitchAccountPopover}
            anchorOrigin={{
              vertical: 'top',
              horizontal: 'right'
            }}
            transformOrigin={{
              vertical: 'top',
              horizontal: 'left'
            }}
            slotProps={{
              paper: {
                sx: {
                  mt: 1.5,
                  '&::before': {
                    content: '""',
                    display: 'block',
                    position: 'absolute',
                    top: 0,
                    right: 14,
                    width: 10,
                    height: 10,
                    bgcolor: 'background.paper',
                    transform: 'translateY(-50%) rotate(45deg)',
                    zIndex: 0
                  }
                }
              }
            }}>
            {connectedAccounts.map((account) => (
              <MenuItem
                key={account}
                onClick={() => handleSwitchAccount(account)}>
                <ListItemIcon>
                  <ArrowRightToLine />
                </ListItemIcon>
                <ListItemText>{`Switch to ${mapProviderToName(account)}`}</ListItemText>
              </MenuItem>
            ))}
          </Menu>
        </MenuList>
      </Popover>
    </>
  );
};

export default ProfileMenu;
