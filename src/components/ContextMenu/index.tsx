import {
  Divider,
  ListItemText,
  MenuItem,
  MenuList,
  Popover
} from '@mui/material';

import AddToList from './components/AddToList';
import useContextMenu from './useContextMenu';

const ContextMenu = () => {
  const {
    popoverPosition,
    menuItems,
    addToListAnchorEl,
    close,
    closeAddToListMenu
  } = useContextMenu();

  return (
    <Popover
      anchorReference="anchorPosition"
      anchorPosition={popoverPosition ?? undefined}
      open={Boolean(popoverPosition)}
      onClose={close}
      transformOrigin={{
        vertical: 'top',
        horizontal: 'left'
      }}>
      <MenuList dense>
        {menuItems.flatMap(
          ({ label, disabled, renderDivider, rightIcon, handleClick }) => {
            const items = [
              <MenuItem
                key={label}
                onClick={handleClick}
                sx={{
                  paddingRight: '4px',
                  width: '160px'
                }}
                disabled={disabled}>
                <ListItemText>{label}</ListItemText>
                {rightIcon && rightIcon}
              </MenuItem>
            ];

            if (renderDivider) {
              items.push(<Divider key={`${label}-divider`} />);
            }
            return items;
          }
        )}

        <AddToList
          anchorEl={addToListAnchorEl}
          open={Boolean(addToListAnchorEl)}
          onClose={closeAddToListMenu}
        />
      </MenuList>
    </Popover>
  );
};

export default ContextMenu;
