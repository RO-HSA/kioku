import {
  ListItemIcon,
  ListItemText,
  Menu,
  MenuItem,
  Tooltip
} from '@mui/material';
import { BookOpenText, Tv } from 'lucide-react';
import { FC, JSX } from 'react';

import { ListType } from '@/types/List';

interface ListTypeMenuProps {
  anchorEl: HTMLElement | null;
  open: boolean;
  selectedListType: ListType;
  onClose: () => void;
  onSwitchListType: (listType: ListType) => void;
}

const listTypeItems: {
  type: ListType;
  icon: JSX.Element;
  label: string;
}[] = [
  {
    type: 'anime',
    icon: <Tv />,
    label: 'Anime'
  },
  {
    type: 'manga',
    icon: <BookOpenText />,
    label: 'Manga'
  }
];

const ListTypeMenu: FC<ListTypeMenuProps> = ({
  anchorEl,
  open,
  selectedListType,
  onClose,
  onSwitchListType
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
      {listTypeItems.map(({ type, icon, label }) => (
        <Tooltip
          key={type}
          title={selectedListType === type ? 'Current list type' : ''}
          placement="right">
          <div>
            <MenuItem
              selected={selectedListType === type}
              onClick={() => onSwitchListType(type)}>
              <ListItemIcon>{icon}</ListItemIcon>
              <ListItemText>{label}</ListItemText>
            </MenuItem>
          </div>
        </Tooltip>
      ))}
    </Menu>
  );
};

export default ListTypeMenu;
