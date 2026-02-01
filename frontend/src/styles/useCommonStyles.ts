import { makeStyles, tokens } from '@fluentui/react-components';

export const useCommonStyles = makeStyles({
  pageContainer: {
    padding: '24px',
    backgroundColor: 'rgba(255, 255, 255, 0.6)', // Acrylic background for content pages
    backdropFilter: 'blur(20px)',
    minHeight: '100%',
    boxSizing: 'border-box',
    borderRadius: tokens.borderRadiusLarge, // Optional: rounded corners for the "sheet" look
    margin: '16px', // Add some margin so it looks like a floating sheet
  },
  card: {
    borderRadius: tokens.borderRadiusLarge,
    boxShadow: tokens.shadow4,
    backgroundColor: 'rgba(255, 255, 255, 0.7)', // Semi-transparent
    backdropFilter: 'blur(10px)', // Acrylic effect on cards
  },
  flexColumn: {
    display: 'flex',
    flexDirection: 'column',
  },
  flexRow: {
    display: 'flex',
    flexDirection: 'row',
    alignItems: 'center',
  },
  centerContainer: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    minHeight: '100vh',
    backgroundColor: 'transparent', // Changed to transparent
  },
  fullWidth: {
    width: '100%',
  },
  gap8: {
    gap: '8px',
  },
  gap16: {
    gap: '16px',
  },
  gap24: {
    gap: '24px',
  },
  marginTop16: {
    marginTop: '16px',
  },
  marginTop24: {
    marginTop: '24px',
  },
  textCenter: {
    textAlign: 'center',
  },
  link: {
    color: tokens.colorBrandForeground1,
    textDecoration: 'none',
    fontWeight: tokens.fontWeightSemibold,
    ':hover': {
      textDecoration: 'underline',
    },
  },
});
