/**
 * 颜色工具函数
 * 用于从 Bing 每日一图提取主色调，实现类似 Material You (Monet) 的动态取色效果
 */

/**
 * 将 RGB 转换为 Hex
 */
export const rgbToHex = (r: number, g: number, b: number): string => {
  return '#' + [r, g, b].map(x => {
    const hex = x.toString(16);
    return hex.length === 1 ? '0' + hex : hex;
  }).join('');
};

/**
 * 从图片 URL 提取主色调
 * 使用简单的平均色算法，配合 Canvas 读取像素数据
 * 注意：图片必须支持跨域访问 (CORS)，否则画布会被污染导致无法读取数据
 */
export const getDominantColor = async (imageUrl: string): Promise<string> => {
  return new Promise((resolve, reject) => {
    const img = new Image();
    // 启用跨域，Bing 图片通常支持
    img.crossOrigin = 'Anonymous';
    img.src = imageUrl;

    img.onload = () => {
      const canvas = document.createElement('canvas');
      const ctx = canvas.getContext('2d');
      if (!ctx) {
        resolve('#0078d4'); // 获取上下文失败，返回默认 Fluent 蓝
        return;
      }

      // 将图片缩小绘制以减少计算量，同时起到一定的模糊/平滑作用
      const sampleSize = 50;
      canvas.width = sampleSize;
      canvas.height = sampleSize;
      ctx.drawImage(img, 0, 0, sampleSize, sampleSize);

      try {
        const imageData = ctx.getImageData(0, 0, sampleSize, sampleSize).data;
        let r = 0, g = 0, b = 0;
        let count = 0;

        for (let i = 0; i < imageData.length; i += 4) {
          // 忽略透明度低的像素
          if (imageData[i + 3] < 128) continue;

          r += imageData[i];
          g += imageData[i + 1];
          b += imageData[i + 2];
          count++;
        }

        if (count === 0) {
          resolve('#0078d4');
          return;
        }

        r = Math.floor(r / count);
        g = Math.floor(g / count);
        b = Math.floor(b / count);

        // 稍微增加一点饱和度或亮度，避免颜色过于灰暗
        // 这里只是简单的返回平均色
        resolve(rgbToHex(r, g, b));
      } catch (e) {
        // 如果遇到跨域污染画布问题 (tainted canvas)
        console.warn('Failed to extract color from image (likely CORS issue):', e);
        resolve('#0078d4'); // Fallback
      }
    };

    img.onerror = () => {
      console.warn('Failed to load image for color extraction');
      resolve('#0078d4'); // Fallback
    };
  });
};

/**
 * 调整颜色亮度
 * @param hexColor #RRGGBB
 * @param percent -100 to 100 (negative to darken, positive to lighten)
 */
export const adjustColorBrightness = (hexColor: string, percent: number): string => {
  // 移除 # 号
  const hex = hexColor.replace('#', '');
  
  // 解析 RGB
  const num = parseInt(hex, 16);
  
  // 计算调整量
  const amt = Math.round(2.55 * percent);
  
  // 分别调整 R, G, B，并确保在 0-255 范围内
  let R = (num >> 16) + amt;
  let B = ((num >> 8) & 0x00FF) + amt;
  let G = (num & 0x0000FF) + amt;

  R = R < 255 ? (R < 1 ? 0 : R) : 255;
  G = G < 255 ? (G < 1 ? 0 : G) : 255;
  B = B < 255 ? (B < 1 ? 0 : B) : 255;

  // 重新组合为 Hex
  return '#' + (0x1000000 + R * 0x10000 + B * 0x100 + G).toString(16).slice(1);
};