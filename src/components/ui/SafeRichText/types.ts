import { TypographyProps } from '@mui/material';

export type SafeRichTextProps = Omit<TypographyProps, 'children'> & {
  content: string;
};

export type InlinePattern = {
  regex: RegExp;
  render: (match: RegExpExecArray) => string;
};
