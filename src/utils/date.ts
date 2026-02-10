import { format, isValid, parseISO } from 'date-fns';

export const parseDateValue = (value?: string) => {
  if (!value) return null;

  const parsed = parseISO(value);

  return isValid(parsed) ? parsed : null;
};

export const formatDateValue = (value: Date | null) => {
  if (!value || !isValid(value)) return undefined;

  return format(value, 'yyyy-MM-dd');
};
