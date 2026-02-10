import useWindowSize from '@/hooks/useWindowSize';
import { DesktopDatePicker, MobileDatePicker } from '@mui/x-date-pickers';
import { FC } from 'react';

interface DatePickerProps {
  value: Date | null;
  label: string;
  onChange: (newValue: Date | null) => void;
}

const DatePicker: FC<DatePickerProps> = ({ label, value, onChange }) => {
  const { isMobile } = useWindowSize();

  if (isMobile) {
    return (
      <MobileDatePicker
        className="max-w-56.75 w-full min-w-0!"
        format="yyyy-MM-dd"
        label={label}
        slotProps={{ textField: { size: 'small' } }}
        value={value}
        onChange={onChange}
      />
    );
  }

  return (
    <DesktopDatePicker
      className="max-w-56.75 w-full min-w-0!"
      format="yyyy-MM-dd"
      label={label}
      slotProps={{ textField: { size: 'small' } }}
      value={value}
      onChange={onChange}
    />
  );
};

export default DatePicker;
