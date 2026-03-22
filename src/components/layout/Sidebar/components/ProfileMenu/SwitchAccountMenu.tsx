import { ListItemIcon, ListItemText, Menu, MenuItem } from '@mui/material';
import { ArrowRightToLine } from 'lucide-react';
import { FC } from 'react';

import { Provider } from '@/types/List';
import { mapProviderToName } from '@/utils/provider';

interface SwitchAccountMenuProps {
  anchorEl: HTMLElement | null;
  open: boolean;
  connectedAccounts: Provider[];
  onClose: () => void;
  onSwitchAccount: (provider: Provider) => void;
}

const SwitchAccountMenu: FC<SwitchAccountMenuProps> = ({
  anchorEl,
  open,
  connectedAccounts,
  onClose,
  onSwitchAccount
}) => {
  return (
    <Menu
      anchorEl={anchorEl}
      open={open}
      onClose={onClose}
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
        <MenuItem key={account} onClick={() => onSwitchAccount(account)}>
          <ListItemIcon>
            <ArrowRightToLine />
          </ListItemIcon>
          <ListItemText>{`Switch to ${mapProviderToName(account)}`}</ListItemText>
        </MenuItem>
      ))}
    </Menu>
  );
};

export default SwitchAccountMenu;
