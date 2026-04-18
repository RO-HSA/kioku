import { HTML_TAG_PATTERN } from './constants';

const escapeHtml = (value: string) =>
  value
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#39;');

const escapeAttribute = (value: string) =>
  value.replace(/&/g, '&amp;').replace(/"/g, '&quot;');

const normalizeLineEndings = (value: string) => value.replace(/\r\n?/g, '\n');

const getSafeUrl = (value: string) => {
  const url = value.trim();

  if (/^(https?:|mailto:)/i.test(url)) {
    return url;
  }

  return null;
};

const hasHtmlTags = (value: string) => HTML_TAG_PATTERN.test(value);

export {
  escapeAttribute,
  escapeHtml,
  getSafeUrl,
  hasHtmlTags,
  normalizeLineEndings
};
