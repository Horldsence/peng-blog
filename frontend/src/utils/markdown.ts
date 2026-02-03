/**
 * Markdown 工具函数
 */

/**
 * 将 Markdown 内容转换为纯文本，去除所有 markdown 语法
 */
export function stripMarkdown(markdown: string): string {
  let text = markdown;

  // 移除标题标记 #, ##, ### 等
  text = text.replace(/^#{1,6}\s+/gm, '');

  // 移除加粗 **text** 或 __text__
  text = text.replace(/\*\*(.*?)\*\*/g, '$1');
  text = text.replace(/__(.*?)__/g, '$1');

  // 移除斜体 *text* 或 _text_
  text = text.replace(/\*(.*?)\*/g, '$1');
  text = text.replace(/_(.*?)_/g, '$1');

  // 移除删除线 ~~text~~
  text = text.replace(/~~(.*?)~~/g, '$1');

  // 移除行内代码 `code`
  text = text.replace(/`([^`]+)`/g, '$1');

  // 移除代码块 ```code```
  text = text.replace(/```[\s\S]*?```/g, '');

  // 移除链接 [text](url)
  text = text.replace(/\[([^\]]+)\]\([^)]+\)/g, '$1');

  // 移除图片 ![alt](url)
  text = text.replace(/!\[([^\]]*)\]\([^)]+\)/g, '$1');

  // 移除引用 > text
  text = text.replace(/^>\s+/gm, '');

  // 移除无序列表 - 或 * 或 +
  text = text.replace(/^[\s]*[-*+]\s+/gm, '');

  // 移除有序列表 1. 2. 等
  text = text.replace(/^\d+\.\s+/gm, '');

  // 移除水平线 ---, ***, ___
  text = text.replace(/^[-*_]{3,}\s*$/gm, '');

  // 移除多余的换行符
  text = text.replace(/\n{3,}/g, '\n\n');

  return text.trim();
}

/**
 * 获取文章简介（纯文本，截取到指定长度）
 */
export function getPostExcerpt(markdown: string, maxLength: number = 180): string {
  const plainText = stripMarkdown(markdown);

  if (plainText.length <= maxLength) {
    return plainText;
  }

  // 截取到最大长度，尝试在单词边界截断
  let truncated = plainText.substring(0, maxLength);
  const lastSpaceIndex = truncated.lastIndexOf(' ');

  if (lastSpaceIndex > maxLength * 0.8) {
    // 如果最后一个空格在 80% 之后，就在空格处截断
    truncated = truncated.substring(0, lastSpaceIndex);
  }

  return truncated + '...';
}

export interface TOCItem {
  id: string;
  text: string;
  level: number;
}

/**
 * 生成锚点 ID
 */
export function slugify(text: string): string {
  return text
    .toLowerCase()
    .trim()
    .replace(/\s+/g, '-')
    .replace(/[^\w\u4e00-\u9fa5-]/g, '')
    .replace(/^-+|-+$/g, '');
}

/**
 * 从 Markdown 提取目录
 */
export function extractTOC(markdown: string): TOCItem[] {
  const lines = markdown.split('\n');
  const toc: TOCItem[] = [];
  let isInCodeBlock = false;

  // 用于处理重复的 id
  const idCounts: Record<string, number> = {};

  for (const line of lines) {
    // 检查代码块标记
    if (line.trim().startsWith('```')) {
      isInCodeBlock = !isInCodeBlock;
      continue;
    }

    if (isInCodeBlock) continue;

    const match = line.match(/^(#{1,6})\s+(.+)$/);
    if (match) {
      const level = match[1].length;
      // 去除可能存在的 markdown 格式（如加粗）
      const text = match[2].replace(/\*\*(.*?)\*\*/g, '$1').replace(/__(.*?)__/g, '$1').trim();
      
      let id = slugify(text);

      // 如果 id 为空（全是特殊字符的情况），使用默认前缀
      if (!id) id = 'heading';

      if (idCounts[id]) {
        id = `${id}-${idCounts[id]}`;
        idCounts[id]++;
      } else {
        idCounts[id] = 1;
      }

      toc.push({ id, text, level });
    }
  }

  return toc;
}

/**
 * 计算预计阅读时间（分钟）
 */
export function calculateReadingTime(markdown: string): number {
  const text = stripMarkdown(markdown);
  
  // 统计中文字符
  const cnMatch = text.match(/[\u4e00-\u9fa5]/g);
  const cnCount = cnMatch ? cnMatch.length : 0;
  
  // 统计英文单词（去除非单词字符后分割）
  const enText = text.replace(/[\u4e00-\u9fa5]/g, ' ');
  const enMatch = enText.match(/[a-zA-Z0-9]+/g);
  const enCount = enMatch ? enMatch.length : 0;

  // 估算：中文 400字/分钟，英文 200词/分钟
  // 将英文单词换算成"中文单位"：1单词 ≈ 2中文字符的时间（粗略）
  const totalWeight = cnCount + (enCount * 2);
  
  // 以 400 为基准计算分钟数
  const readingTime = Math.ceil(totalWeight / 400);
  
  return readingTime < 1 ? 1 : readingTime;
}
