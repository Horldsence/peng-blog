import { ReactNode, useRef } from 'react';
import gsap from 'gsap';
import { useGSAP } from '@gsap/react';

interface PageTransitionProps {
  children: ReactNode;
  className?: string;
}

export const PageTransition = ({ children, className }: PageTransitionProps) => {
  const comp = useRef<HTMLDivElement>(null);

  useGSAP(
    () => {
      // 简单的进场动画：从下方轻微划入并淡入
      gsap.fromTo(
        comp.current,
        { opacity: 0, y: 15 },
        {
          opacity: 1,
          y: 0,
          duration: 0.35,
          ease: 'power2.out',
          clearProps: 'all', // 动画结束后清除样式，避免干扰布局交互
        }
      );
    },
    { scope: comp }
  );

  return (
    <div ref={comp} className={className} style={{ width: '100%', height: '100%' }}>
      {children}
    </div>
  );
};