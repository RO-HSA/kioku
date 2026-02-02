import { useColorScheme, useTheme } from '@mui/material';
import { useMemo } from 'react';

const useMaterialTableTheme = () => {
  const theme = useTheme();

  const { mode, systemMode } = useColorScheme();

  const resolvedMode = mode === 'system' ? systemMode : mode;

  const palette = useMemo(() => {
    const typedTheme = theme as typeof theme & {
      colorSchemes?: Record<string, { palette: typeof theme.palette }>;
      defaultColorScheme?: string;
    };
    const defaultScheme = typedTheme.defaultColorScheme ?? 'light';
    const schemeKey = resolvedMode ?? defaultScheme;
    const schemePalette = typedTheme.colorSchemes?.[schemeKey]?.palette;

    return schemePalette ?? theme.palette;
  }, [theme, resolvedMode]);

  const mrtTheme = useMemo(
    () => ({
      baseBackgroundColor: palette.background.paper,
      cellNavigationOutlineColor: palette.primary.main,
      draggingBorderColor: palette.primary.main,
      matchHighlightColor: palette.warning.main,
      menuBackgroundColor: palette.background.paper,
      pinnedRowBackgroundColor: palette.action.selected,
      selectedRowBackgroundColor: palette.action.selected
    }),
    [palette]
  );

  const muiTablePaperProps = useMemo(
    () => ({
      elevation: 0,
      sx: {
        backgroundColor: palette.background.paper,
        color: palette.text.primary
      }
    }),
    [palette]
  );

  const muiTableContainerProps = useMemo(
    () => ({
      sx: {
        backgroundColor: palette.background.paper
      }
    }),
    [palette]
  );

  const muiTableHeadCellProps = useMemo(
    () => ({
      sx: {
        backgroundColor: palette.background.default,
        color: palette.text.primary,
        borderColor: palette.divider,
        '&& .MuiTableSortLabel-root': {
          color: palette.text.primary,
          '&:hover': {
            color: palette.text.primary
          }
        },
        '&& .MuiTableSortLabel-root.Mui-active': {
          color: palette.text.primary
        },
        '&& .MuiTableSortLabel-icon': {
          color: `${palette.text.secondary} !important`,
          opacity: 1
        },
        '&& .MuiTableSortLabel-root.Mui-active .MuiTableSortLabel-icon': {
          color: `${palette.text.primary} !important`
        }
      }
    }),
    [palette]
  );

  const muiTableBodyCellProps = useMemo(
    () => ({
      sx: {
        color: palette.text.primary,
        borderColor: palette.divider
      }
    }),
    [palette]
  );

  const muiTableBodyRowProps = useMemo(
    () => ({
      hover: true,
      sx: {
        '&:hover td': {
          backgroundColor: palette.action.hover
        }
      }
    }),
    [palette]
  );

  const muiTopToolbarProps = useMemo(
    () => ({
      sx: {
        backgroundColor: palette.background.default,
        borderBottom: `1px solid ${palette.divider}`
      }
    }),
    [palette]
  );

  return {
    mrtTheme,
    muiTablePaperProps,
    muiTableContainerProps,
    muiTableHeadCellProps,
    muiTableBodyCellProps,
    muiTableBodyRowProps,
    muiTopToolbarProps
  };
};

export default useMaterialTableTheme;
