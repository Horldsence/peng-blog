import {
  Card,
  CardHeader,
  Caption1,
  Text,
  Button,
  tokens,
  makeStyles,
} from '@fluentui/react-components';
import { ArrowRightRegular, CalendarRegular, EyeRegular } from '@fluentui/react-icons';
import { useNavigate } from 'react-router-dom';
import { motion } from 'framer-motion';
import type { Post } from '../../types';
import { getPostExcerpt } from '../../utils/markdown';

const useStyles = makeStyles({
  postCard: {
    cursor: 'pointer',
    borderRadius: tokens.borderRadiusLarge,
    backgroundColor: 'rgba(255, 255, 255, 0.1)',
    backdropFilter: 'blur(10px)',
    border: '1px solid rgba(255, 255, 255, 0.2)',
    display: 'flex',
    flexDirection: 'column',
    ':hover': {
      backgroundColor: 'rgba(255, 255, 255, 0.2)',
    },
  },
  cardHeader: {
    padding: '24px',
    flexGrow: 1,
  },
  cardTitle: {
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    display: '-webkit-box',
    WebkitLineClamp: 2,
    WebkitBoxOrient: 'vertical',
    marginBottom: '12px',
  },
  cardDescription: {
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    display: '-webkit-box',
    WebkitLineClamp: 3,
    WebkitBoxOrient: 'vertical',
    color: tokens.colorNeutralForeground2,
    lineHeight: '1.6',
  },
  cardFooter: {
    display: 'flex',
    alignItems: 'center',
    gap: '20px',
    padding: '16px 24px',
    borderTop: `1px solid ${tokens.colorNeutralStroke1}`,
  },
  metaItem: {
    display: 'flex',
    alignItems: 'center',
    gap: '6px',
    color: tokens.colorNeutralForeground2,
  },
  viewButtonContainer: {
    marginLeft: 'auto',
  },
});

interface PostCardProps {
  post: Post;
}

export function PostCard({ post }: PostCardProps) {
  const styles = useStyles();
  const navigate = useNavigate();

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    });
  };

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      whileInView={{ opacity: 1, y: 0 }}
      viewport={{ once: true, margin: '-50px' }}
      transition={{ duration: 0.4, ease: 'easeOut' }}
      whileHover={{
        y: -4,
        boxShadow: '0 10px 30px rgba(0,0,0,0.1)',
        transition: { duration: 0.2, ease: 'easeOut' },
      }}
      onClick={() => navigate(`/post/${post.id}`)}
    >
      <Card className={styles.postCard}>
        <CardHeader
          className={styles.cardHeader}
          header={
            <Text weight="semibold" size={500} className={styles.cardTitle}>
              {post.title}
            </Text>
          }
          description={
            <Caption1 className={styles.cardDescription}>
              {getPostExcerpt(post.content, 200)}
            </Caption1>
          }
        />

        <div className={styles.cardFooter}>
          <div className={styles.metaItem}>
            <CalendarRegular fontSize={14} />
            <Caption1>{formatDate(post.created_at)}</Caption1>
          </div>
          <div className={styles.metaItem}>
            <EyeRegular fontSize={14} />
            <Caption1>{post.views}</Caption1>
          </div>
          <div className={styles.viewButtonContainer}>
            <Button
              appearance="transparent"
              icon={<ArrowRightRegular />}
              onClick={(e) => {
                e.stopPropagation();
                navigate(`/post/${post.id}`);
              }}
            >
              阅读
            </Button>
          </div>
        </div>
      </Card>
    </motion.div>
  );
}
