import { Box, Tooltip } from '@mui/material';
import {
  MRT_ExpandButton,
  MRT_Row,
  MRT_RowData,
  MRT_TableInstance
} from 'material-react-table';

interface GroupedExpandCellProps<TData extends MRT_RowData> {
  row: MRT_Row<TData>;
  staticRowIndex?: number;
  table: MRT_TableInstance<TData>;
  fontSize?: number | string;
}

const GroupedExpandCell = <TData extends MRT_RowData>({
  row,
  staticRowIndex,
  table,
  fontSize = '0.75rem'
}: GroupedExpandCellProps<TData>) => {
  if (row.groupingColumnId) {
    const subRowsLength = row.subRows?.length;

    return (
      <Box
        component="span"
        sx={{ display: 'inline-flex', alignItems: 'center', gap: 0.5 }}>
        <MRT_ExpandButton
          row={row}
          staticRowIndex={staticRowIndex}
          table={table}
        />
        <Tooltip title={table.getColumn(row.groupingColumnId).columnDef.header}>
          <Box component="span" sx={{ fontSize, lineHeight: 1.2 }}>
            {String(row.groupingValue)}
          </Box>
        </Tooltip>
        {!!subRowsLength && (
          <Box component="span" sx={{ fontSize, lineHeight: 1.2 }}>
            ({subRowsLength})
          </Box>
        )}
      </Box>
    );
  }

  return (
    <MRT_ExpandButton row={row} staticRowIndex={staticRowIndex} table={table} />
  );
};

export default GroupedExpandCell;
