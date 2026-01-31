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
