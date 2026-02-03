import { useEffect, useState } from 'react';
import mermaid from 'mermaid';
import { makeStyles, tokens } from '@fluentui/react-components';

const useStyles = makeStyles({
  container: {
    display: 'flex',
    justifyContent: 'center',
    padding: '16px',
    backgroundColor: tokens.colorNeutralBackground1,
    borderRadius: '8px',
    marginBottom: '16px',
    overflowX: 'auto',
  },
  error: {
    color: tokens.colorPaletteRedForeground1,
    padding: '8px',
    border: `1px solid ${tokens.colorPaletteRedBorderActive}`,
    borderRadius: '4px',
  },
});

interface MermaidProps {
  chart: string;
}

mermaid.initialize({
  startOnLoad: false,
  theme: 'dark',
  securityLevel: 'loose',
});

let diagramId = 0;

export const Mermaid = ({ chart }: MermaidProps) => {
  const styles = useStyles();
  const [svg, setSvg] = useState<string>('');
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const renderChart = async () => {
      if (!chart) return;

      try {
        const id = `mermaid-diagram-${diagramId++}`;
        const { svg } = await mermaid.render(id, chart);
        setSvg(svg);
        setError(null);
      } catch (err) {
        console.error('Mermaid render error:', err);
        setError('无法渲染图表，请检查语法');
      }
    };

    renderChart();
  }, [chart]);

  if (error) {
    return (
      <div className={styles.container}>
        <div className={styles.error}>{error}</div>
        <pre>{chart}</pre>
      </div>
    );
  }

  return <div className={styles.container} dangerouslySetInnerHTML={{ __html: svg }} />;
};
