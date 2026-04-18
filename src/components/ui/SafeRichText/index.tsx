import { Typography } from '@mui/material';
import { useMemo } from 'react';

import { markdownToHtml } from './helpers/markdown';
import { sanitizeHtml } from './helpers/sanitize';
import { hasHtmlTags, normalizeLineEndings } from './helpers/utils';
import { safeRichTextSx } from './styles';
import { SafeRichTextProps } from './types';

const SafeRichText = ({
  content,
  component = 'div',
  sx,
  variant = 'body2',
  ...props
}: SafeRichTextProps) => {
  const renderedContent = useMemo(() => {
    const normalizedContent = normalizeLineEndings(content).trim();

    if (!normalizedContent) {
      return '';
    }

    const source = hasHtmlTags(normalizedContent)
      ? normalizedContent
      : markdownToHtml(normalizedContent);

    return sanitizeHtml(source);
  }, [content]);

  return (
    <Typography
      component={component}
      sx={Array.isArray(sx) ? [safeRichTextSx, ...sx] : [safeRichTextSx, sx]}
      variant={variant}
      {...props}>
      {renderedContent}
    </Typography>
  );
};

export default SafeRichText;
