import { useState, useEffect } from 'react';
import { Button, makeStyles, tokens } from '@fluentui/react-components';
import { ArrowUpRegular } from '@fluentui/react-icons';

const useStyles = makeStyles({
  container: {
    position: 'fixed',
    bottom: '32px',
    right: '32px',
    zIndex: 1000,
    opacity: 0,
    transform: 'translateY(20px)',
    transition: 'all 0.3s cubic-bezier(0.33, 1, 0.68, 1)',
    pointerEvents: 'none',
  },
  visible: {
    opacity: 1,
    transform: 'translateY(0)',
    pointerEvents: 'auto',
  },
  button: {
    backgroundColor: tokens.colorBrandBackground,
    color: tokens.colorNeutralForegroundOnBrand,
    boxShadow: tokens.shadow16,
    ':hover': {
      backgroundColor: tokens.colorBrandBackgroundHover,
    },
    ':active': {
      backgroundColor: tokens.colorBrandBackgroundPressed,
    },
  },
});

export const BackToTop = () => {
  const styles = useStyles();
  const [visible, setVisible] = useState(false);

  useEffect(() => {
    const handleScroll = () => {
      if (window.scrollY > 400) {
        setVisible(true);
      } else {
        setVisible(false);
      }
    };

    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  const scrollToTop = () => {
    window.scrollTo({
      top: 0,
      behavior: 'smooth',
    });
  };

  return (
    <div className={`${styles.container} ${visible ? styles.visible : ''}`}>
      <Button
        className={styles.button}
        icon={<ArrowUpRegular fontSize={24} />}
        shape="circular"
        size="large"
        onClick={scrollToTop}
        aria-label="Back to top"
      />
    </div>
  );
};
