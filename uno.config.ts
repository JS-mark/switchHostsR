import {
  defineConfig,
  presetAttributify,
  presetIcons,
  presetTypography,
  presetWebFonts,
  presetWind3,
  transformerDirectives,
  transformerVariantGroup,
} from 'unocss'

export default defineConfig({
  presets: [
    presetWind3(),
    presetAttributify(),
    presetIcons(),
    presetTypography(),
    presetWebFonts(),
  ],
  transformers: [
    transformerDirectives(),
    transformerVariantGroup(),
  ], // 添加缺失的子元素样式
  rules: [
    [/^aspect-ratio-(\d+)\/(\d+)$/, ([, n, d]) => ({
      'position': 'relative',
      'padding-bottom': `${(Number(d) / Number(n)) * 100}%`,
    })],
  ],
  preflights: [
    {
      getCSS: () => `
        [class*="aspect-ratio-"] > * {
          position: absolute;
          height: 100%;
          width: 100%;
          top: 0;
          right: 0;
          bottom: 0;
          left: 0;
        }
      `,
    },
  ],
  // 主题扩展
  theme: {
    colors: {
      primary: {
        DEFAULT: '#096',
        dark: '#064',
      },
    },
    breakpoints: {
      'sm': '640px',
      'md': '768px',
      'lg': '1024px',
      'xl': '1280px',
      '2xl': '1536px',
    },
  },
})
