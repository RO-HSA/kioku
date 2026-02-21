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

export const getTodayAsYmd = (): string => {
  const now = new Date();
  const year = now.getFullYear();
  const month = String(now.getMonth() + 1).padStart(2, '0');
  const day = String(now.getDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
};

export const formatDate = (isoDate: string | null): string | null => {
  if (!isoDate) {
    return null;
  }

  const parsedDate = new Date(isoDate);

  if (Number.isNaN(parsedDate.getTime())) {
    return null;
  }

  return parsedDate.toLocaleString();
};
