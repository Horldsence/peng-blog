import React, { useState } from 'react';
import { Button, Tooltip, makeStyles, tokens } from '@fluentui/react-components';
import { CopyRegular, CheckmarkRegular } from '@fluentui/react-icons';
import { Mermaid } from './Mermaid';

const useStyles = makeStyles({
  root: {
    position: 'relative',
    marginBottom: '16px',
    backgroundColor: '#0d1117',
    borderRadius: '8px',
    overflow: 'hidden',
    border: `1px solid ${tokens.colorNeutralStroke1}`,
  },
  header: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    padding: '8px 16px',
    backgroundColor: '#161b22',
    borderBottom: '1px solid #30363d',
    color: '#c9d1d9',
    fontSize: '12px',
    fontFamily: 'SFMono-Regular, Consolas, "Liberation Mono", Menlo, monospace',
    userSelect: 'none',
  },
  contentWrapper: {
    position: 'relative',
    overflow: 'auto',
  },
  content: {
    padding: '16px',
    margin: 0,
    fontSize: '14px',
    lineHeight: '1.5',
    fontFamily: 'SFMono-Regular, Consolas, "Liberation Mono", Menlo, monospace',
    color: '#c9d1d9',
    backgroundColor: 'transparent',
    border: 'none',
  },
  inline: {
    backgroundColor: tokens.colorNeutralBackground1Hover,
    padding: '2px 6px',
    borderRadius: '4px',
    fontSize: '14px',
    fontFamily: 'SFMono-Regular, Consolas, "Liberation Mono", Menlo, monospace',
    color: tokens.colorNeutralForeground1,
  },
  copyButton: {
    color: '#8b949e',
    '&:hover': {
      color: '#c9d1d9',
      backgroundColor: 'rgba(177, 186, 196, 0.12)',
    },
  },
  langLabel: {
    textTransform: 'uppercase',
    fontWeight: '600',
    color: '#8b949e',
  },
});

// Helper to extract text from React children
const extractText = (children: any): string => {
  if (typeof children === 'string') return children;
  if (Array.isArray(children)) return children.map(extractText).join('');
  if (children?.props?.children) return extractText(children.props.children);
  return '';
};

export const CodeBlock = ({ inline, className, children, ...props }: any) => {
  const styles = useStyles();
  const [copied, setCopied] = useState(false);

  const match = /language-(\w+)/.exec(className || '');
  const language = match ? match[1] : 'text';

  // Extract raw text for copy/mermaid
  const rawContent = extractText(children).replace(/\n$/, '');

  if (inline) {
    return (
      <code className={styles.inline} {...props}>
        {children}
      </code>
    );
  }

  if (language === 'mermaid') {
    return <Mermaid chart={rawContent} />;
  }

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(rawContent);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  };

  return (
    <div className={styles.root}>
      <div className={styles.header}>
        <span className={styles.langLabel}>{language}</span>
        <Tooltip content={copied ? '已复制' : '复制内容'} relationship="label">
          <Button
            appearance="subtle"
            icon={copied ? <CheckmarkRegular /> : <CopyRegular />}
            size="small"
            onClick={handleCopy}
            className={styles.copyButton}
            aria-label="Copy code"
          />
        </Tooltip>
      </div>
      <div className={styles.contentWrapper}>
        <pre className={styles.content}>
          <code className={className} {...props}>
            {children}
          </code>
        </pre>
      </div>
    </div>
  );
};
