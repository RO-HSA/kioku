import { createElement, Fragment, ReactNode } from 'react';

import { ALLOWED_TAGS, BLOCKED_TAGS } from './constants';
import { getSafeUrl } from './utils';

const sanitizeNodes = (nodes: ChildNode[], path: string): ReactNode[] =>
  nodes
    .map((node, index) => sanitizeNode(node, `${path}-${index}`))
    .filter((node): node is ReactNode => node !== null);

const sanitizeNode = (node: ChildNode, key: string): ReactNode | null => {
  if (node.nodeType === Node.TEXT_NODE) {
    return node.textContent;
  }

  if (node.nodeType !== Node.ELEMENT_NODE) {
    return null;
  }

  const element = node as HTMLElement;
  const tagName = element.tagName.toLowerCase();

  if (BLOCKED_TAGS.has(tagName)) {
    return null;
  }

  const children = sanitizeNodes(Array.from(element.childNodes), key);

  if (!ALLOWED_TAGS.has(tagName)) {
    return <Fragment key={key}>{children}</Fragment>;
  }

  if (tagName === 'br') {
    return <br key={key} />;
  }

  const props: Record<string, string> = { key };

  if (tagName === 'a') {
    const safeUrl = getSafeUrl(element.getAttribute('href') || '');

    if (!safeUrl) {
      return <Fragment key={key}>{children}</Fragment>;
    }

    props.href = safeUrl;
    props.target = '_blank';
    props.rel = 'noreferrer noopener';
  }

  return createElement(tagName, props, ...children);
};

const sanitizeHtml = (source: string) => {
  const document = new DOMParser().parseFromString(source, 'text/html');

  return sanitizeNodes(Array.from(document.body.childNodes), 'root');
};

export { sanitizeHtml };
