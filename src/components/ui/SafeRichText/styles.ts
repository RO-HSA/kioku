import { SxProps, Theme } from '@mui/material';

const safeRichTextSx: SxProps<Theme> = {
  py: 1,
  whiteSpace: 'normal',
  wordBreak: 'break-word',
  '& *': {
    fontSize: 'inherit',
    fontFamily: 'inherit'
  },
  '& p, & ul, & ol, & blockquote, & pre': {
    m: 0
  },
  '& p + p, & p + ul, & p + ol, & p + blockquote, & p + pre': {
    mt: 1
  },
  '& ul, & ol': {
    pl: 1
  },
  '& ul + p, & ol + p, & blockquote + p, & pre + p': {
    mt: 1.5
  },
  '& li': {
    listStyle: 'inside',
    mt: 0.5
  },
  '& h1, & h2, & h3, & h4, & h5, & h6': {
    fontWeight: 600,
    lineHeight: 1.3,
    mt: 2,
    mb: 1
  },
  '& a': {
    color: 'primary.main',
    textDecoration: 'underline'
  },
  '& blockquote': {
    borderLeft: (theme) => `3px solid ${theme.palette.divider}`,
    color: 'text.secondary',
    pl: 1.5
  },
  '& code': {
    backgroundColor: 'action.hover',
    borderRadius: 1,
    fontFamily:
      'ui-monospace, SFMono-Regular, SFMono-Regular, Consolas, Liberation Mono, Menlo, monospace',
    px: 0.5,
    py: 0.25
  },
  '& pre': {
    backgroundColor: 'action.hover',
    borderRadius: 1,
    overflowX: 'auto',
    p: 1.5
  },
  '& pre code': {
    backgroundColor: 'transparent',
    p: 0
  }
};

export { safeRichTextSx };
