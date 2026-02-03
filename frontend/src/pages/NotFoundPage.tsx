import { Button, tokens, makeStyles } from '@fluentui/react-components';
import { HomeRegular } from '@fluentui/react-icons';
import { useNavigate } from 'react-router-dom';
import { motion } from 'framer-motion';

const useStyles = makeStyles({
  pageContainer: {
    position: 'relative',
    height: '100%',
    minHeight: '80vh',
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    justifyContent: 'center',
  },
  backgroundOverlay: {
    position: 'absolute',
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    background: `radial-gradient(circle at 50% 50%, ${tokens.colorNeutralBackground1} 0%, transparent 85%)`,
    zIndex: 0,
    pointerEvents: 'none',
  },
  contentContainer: {
    position: 'relative',
    zIndex: 2,
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    justifyContent: 'center',
    padding: '32px',
    textAlign: 'center',
  },
  errorCode: {
    fontSize: '120px',
    fontWeight: tokens.fontWeightBold,
    color: 'transparent',
    margin: 0,
    lineHeight: '1',
    letterSpacing: '-0.05em',
    backgroundImage: `linear-gradient(135deg, ${tokens.colorNeutralForeground1} 0%, ${tokens.colorNeutralForeground2} 50%, ${tokens.colorBrandBackground} 100%)`,
    WebkitBackgroundClip: 'text',
    backgroundClip: 'text',
    filter: 'drop-shadow(0 4px 20px rgba(0,0,0,0.15))',
    userSelect: 'none',
  },
  errorMessage: {
    fontSize: tokens.fontSizeBase600,
    fontWeight: tokens.fontWeightSemibold,
    color: tokens.colorNeutralForeground1,
    margin: '16px 0 8px 0',
  },
  errorDescription: {
    fontSize: tokens.fontSizeBase300,
    color: tokens.colorNeutralForeground3,
    maxWidth: '400px',
    marginBottom: '32px',
    lineHeight: '1.6',
  },
  actionButton: {
    minWidth: '160px',
    height: '48px',
    fontSize: tokens.fontSizeBase300,
    borderRadius: tokens.borderRadiusCircular,
    boxShadow: '0 4px 20px rgba(0,0,0,0.1)',
    transition: 'transform 0.2s ease, box-shadow 0.2s ease',
    ':hover': {
      transform: 'translateY(-2px)',
      boxShadow: '0 8px 25px rgba(0,0,0,0.15)',
    },
  },
  decorativeCircle1: {
    position: 'absolute',
    width: '300px',
    height: '300px',
    borderRadius: '50%',
    background: `linear-gradient(45deg, ${tokens.colorBrandBackground}20, transparent)`,
    filter: 'blur(60px)',
    top: '20%',
    left: '20%',
    zIndex: 1,
  },
  decorativeCircle2: {
    position: 'absolute',
    width: '200px',
    height: '200px',
    borderRadius: '50%',
    background: `linear-gradient(45deg, ${tokens.colorPaletteRedBackground3}20, transparent)`,
    filter: 'blur(50px)',
    bottom: '20%',
    right: '20%',
    zIndex: 1,
  },
});

export function NotFoundPage() {
  const styles = useStyles();
  const navigate = useNavigate();

  return (
    <div className={styles.pageContainer}>
      <div className={styles.backgroundOverlay} />

      {/* Decorative background elements */}
      <motion.div
        className={styles.decorativeCircle1}
        animate={{
          y: [0, -20, 0],
          x: [0, 20, 0],
          scale: [1, 1.1, 1],
        }}
        transition={{ duration: 8, repeat: Infinity, ease: 'easeInOut' }}
      />
      <motion.div
        className={styles.decorativeCircle2}
        animate={{
          y: [0, 30, 0],
          x: [0, -30, 0],
          scale: [1, 1.2, 1],
        }}
        transition={{ duration: 10, repeat: Infinity, ease: 'easeInOut' }}
      />

      <motion.div
        initial={{ opacity: 0, scale: 0.9 }}
        animate={{ opacity: 1, scale: 1 }}
        transition={{ duration: 0.5, ease: 'easeOut' }}
        className={styles.contentContainer}
      >
        <motion.h1
          className={styles.errorCode}
          initial={{ y: 20 }}
          animate={{ y: 0 }}
          transition={{ duration: 0.6, delay: 0.1 }}
        >
          404
        </motion.h1>

        <motion.h2
          className={styles.errorMessage}
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6, delay: 0.2 }}
        >
          页面未找到
        </motion.h2>

        <motion.p
          className={styles.errorDescription}
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ duration: 0.6, delay: 0.3 }}
        >
          抱歉，您访问的页面不存在。它可能已被移动、删除或您输入的链接有误。
        </motion.p>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6, delay: 0.4 }}
        >
          <Button
            appearance="primary"
            size="large"
            icon={<HomeRegular />}
            onClick={() => navigate('/')}
            className={styles.actionButton}
          >
            返回首页
          </Button>
        </motion.div>
      </motion.div>
    </div>
  );
}
