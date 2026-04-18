import { InlinePattern } from '../types';
import {
  escapeAttribute,
  escapeHtml,
  getSafeUrl,
  normalizeLineEndings
} from './utils';

const renderInlineMarkdown = (value: string): string => {
  const patterns: InlinePattern[] = [
    {
      regex: /\[([^\]]+)\]\(([^)\s]+)\)/,
      render: (match) => {
        const [, label, rawUrl] = match;
        const safeUrl = getSafeUrl(rawUrl);

        if (!safeUrl) {
          return escapeHtml(match[0]);
        }

        return `<a href="${escapeAttribute(safeUrl)}">${renderInlineMarkdown(label)}</a>`;
      }
    },
    {
      regex: /`([^`]+)`/,
      render: (match) => `<code>${escapeHtml(match[1])}</code>`
    },
    {
      regex: /\*\*([\s\S]+?)\*\*/,
      render: (match) => `<strong>${renderInlineMarkdown(match[1])}</strong>`
    },
    {
      regex: /\*([\s\S]+?)\*/,
      render: (match) => `<em>${renderInlineMarkdown(match[1])}</em>`
    },
    {
      regex: /~~([\s\S]+?)~~/,
      render: (match) => `<del>${renderInlineMarkdown(match[1])}</del>`
    }
  ];

  let remaining = value;
  let html = '';

  while (remaining.length > 0) {
    let bestMatch:
      | {
          match: RegExpExecArray;
          pattern: InlinePattern;
          priority: number;
        }
      | undefined;

    patterns.forEach((pattern, priority) => {
      const match = pattern.regex.exec(remaining);

      if (!match) {
        return;
      }

      if (
        !bestMatch ||
        match.index < bestMatch.match.index ||
        (match.index === bestMatch.match.index && priority < bestMatch.priority)
      ) {
        bestMatch = { match, pattern, priority };
      }
    });

    if (!bestMatch) {
      html += escapeHtml(remaining);
      break;
    }

    const {
      match,
      pattern,
      match: { index }
    } = bestMatch;

    html += escapeHtml(remaining.slice(0, index));
    html += pattern.render(match);
    remaining = remaining.slice(index + match[0].length);
  }

  return html;
};

const markdownToHtml = (value: string) => {
  const lines = normalizeLineEndings(value).split('\n');
  const blocks: string[] = [];
  const paragraphLines: string[] = [];
  const blockquoteLines: string[] = [];
  const unorderedListItems: string[] = [];
  const orderedListItems: string[] = [];
  const codeFenceLines: string[] = [];
  let isInsideCodeFence = false;

  const flushParagraph = () => {
    if (paragraphLines.length === 0) {
      return;
    }

    blocks.push(
      `<p>${paragraphLines.map(renderInlineMarkdown).join('<br />')}</p>`
    );
    paragraphLines.length = 0;
  };

  const flushBlockquote = () => {
    if (blockquoteLines.length === 0) {
      return;
    }

    blocks.push(
      `<blockquote><p>${blockquoteLines
        .map(renderInlineMarkdown)
        .join('<br />')}</p></blockquote>`
    );
    blockquoteLines.length = 0;
  };

  const flushUnorderedList = () => {
    if (unorderedListItems.length === 0) {
      return;
    }

    blocks.push(`<ul>${unorderedListItems.join('')}</ul>`);
    unorderedListItems.length = 0;
  };

  const flushOrderedList = () => {
    if (orderedListItems.length === 0) {
      return;
    }

    blocks.push(`<ol>${orderedListItems.join('')}</ol>`);
    orderedListItems.length = 0;
  };

  const flushCodeFence = () => {
    if (codeFenceLines.length === 0) {
      return;
    }

    blocks.push(
      `<pre><code>${escapeHtml(codeFenceLines.join('\n'))}</code></pre>`
    );
    codeFenceLines.length = 0;
  };

  const flushAllBlocks = () => {
    flushParagraph();
    flushBlockquote();
    flushUnorderedList();
    flushOrderedList();
  };

  lines.forEach((line) => {
    const trimmedLine = line.trim();

    if (isInsideCodeFence) {
      if (trimmedLine.startsWith('```')) {
        flushCodeFence();
        isInsideCodeFence = false;
        return;
      }

      codeFenceLines.push(line);
      return;
    }

    if (trimmedLine.startsWith('```')) {
      flushAllBlocks();
      isInsideCodeFence = true;
      return;
    }

    if (trimmedLine.length === 0) {
      flushAllBlocks();
      return;
    }

    const headingMatch = /^(#{1,6})\s+(.+)$/.exec(trimmedLine);

    if (headingMatch) {
      flushAllBlocks();
      const level = headingMatch[1].length;
      blocks.push(
        `<h${level}>${renderInlineMarkdown(headingMatch[2])}</h${level}>`
      );
      return;
    }

    const blockquoteMatch = /^>\s?(.*)$/.exec(line);

    if (blockquoteMatch) {
      flushParagraph();
      flushUnorderedList();
      flushOrderedList();
      blockquoteLines.push(blockquoteMatch[1]);
      return;
    }

    flushBlockquote();

    const unorderedListMatch = /^[-*+]\s+(.+)$/.exec(trimmedLine);

    if (unorderedListMatch) {
      flushParagraph();
      flushOrderedList();
      unorderedListItems.push(
        `<li>${renderInlineMarkdown(unorderedListMatch[1])}</li>`
      );
      return;
    }

    flushUnorderedList();

    const orderedListMatch = /^\d+\.\s+(.+)$/.exec(trimmedLine);

    if (orderedListMatch) {
      flushParagraph();
      flushUnorderedList();
      orderedListItems.push(
        `<li>${renderInlineMarkdown(orderedListMatch[1])}</li>`
      );
      return;
    }

    flushOrderedList();
    paragraphLines.push(line.trimEnd());
  });

  if (isInsideCodeFence) {
    flushCodeFence();
  }

  flushAllBlocks();

  return blocks.join('');
};

export { markdownToHtml };
