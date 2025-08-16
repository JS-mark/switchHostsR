import type { UserConfig } from '@rspress/core';
import { createTransformerDiff, createTransformerErrorLevel, createTransformerFocus, createTransformerHighlight, createTransformerLineNumber, pluginShiki } from '@rspress/plugin-shiki'
const config: UserConfig = {
  head: [
    ['link', { rel: 'stylesheet', href: '/custom.css' }],
    [
      'link',
      { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' },
    ],
    ['meta', { name: 'keywords', content: 'hosts,hosts文件管理,域名解析,开发工具,Switch Hosts' }],
    ['meta', { name: 'author', content: 'Switch Hosts R Team' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:title', content: 'Switch Hosts R - 专业的 Hosts 文件管理工具' }],
    ['meta', { property: 'og:description', content: '一个简单易用的 hosts 文件管理工具，支持多环境切换、备份恢复、团队协作等功能' }],
  ],
  root: 'docs',
  title: 'Switch Hosts R',
  description: '一个简单易用的 hosts 文件管理工具',
  icon: '/logo.svg',
  logo: {
    light: '/logo.svg',
    dark: '/logo.svg'
  },
  logoText: 'Switch Hosts R',
  search: {
    mode: 'local',
    versioned: true,
  },
  mediumZoom: {
    selector: '.rspress-doc img',
  },
  plugins: [
    pluginShiki({
      langs: ['vue', 'ts', 'js', 'json', 'html', 'css', 'scss', 'less', 'markdown', 'tsx', 'bash', 'yaml'],
      transformers: [
        // 按需加入即可
        createTransformerDiff(),
        createTransformerLineNumber(),
        createTransformerErrorLevel(),
        createTransformerHighlight(),
        createTransformerFocus(),
      ],
    }),
  ],
  themeConfig: {
    outlineTitle: '页面导航',
    editLink: {
      docRepoBaseUrl: 'https://github.com/JS-mark/switchHostsR/edit/main/docs',
      text: '📝 在 GitHub 上编辑此页',
    },
    // 分享链接
    socialLinks: [
      {
        icon: 'github',
        mode: 'link',
        content: 'https://github.com/JS-mark/switchHostsR',
      },
    ],
    nav: [
      {
        text: '首页',
        link: '/',
      },
      {
        text: '使用指南',
        link: '/guide/',
      },
      {
        text: 'API 文档',
        link: '/api/',
      },
      {
        text: '下载',
        link: '/download',
      },
      {
        text: 'FAQ',
        link: '/faq',
      },
      {
        text: '更多',
        items: [
          {
            text: '更新日志',
            link: '/changelog',
          },
          {
            text: 'GitHub',
            link: 'https://github.com/JS-mark/switchHostsR',
          },
          {
            text: '问题反馈',
            link: 'https://github.com/JS-mark/switchHostsR/issues',
          },
          {
            text: '发布版本',
            link: 'https://github.com/JS-mark/switchHostsR/releases',
          },
        ],
      },
    ],
    sidebar: {
      '/guide/': [
        {
          text: '快速开始',
          items: [
            {
              text: 'Why?',
              link: '/guide/why',
            },
            {
              text: '安装',
              link: '/guide/installation',
            },
            {
              text: '基本使用',
              link: '/guide/basic-usage',
            },
          ],
        },
        {
          text: '功能介绍',
          items: [
            {
              text: 'Hosts 管理',
              link: '/guide/hosts-management',
            },
            {
              text: '用户管理',
              link: '/guide/user-management',
            },
            {
              text: '日志查看',
              link: '/guide/logs',
            },
          ],
        },
      ],
      '/api/': [
        {
          text: 'API 概述',
          items: [
            {
              text: 'API 介绍',
              link: '/api/',
            },
            {
              text: '认证授权',
              link: '/api/auth',
            },
          ],
        },
        {
          text: 'API 参考',
          items: [
            {
              text: 'Hosts API',
              link: '/api/hosts',
            },
            {
              text: 'Users API',
              link: '/api/users',
            },
            {
              text: 'Logs API',
              link: '/api/logs',
            },
          ],
        },
      ],
    },
    // 底部信息
    footer: {
      message: `Copyright © 2025-present The SwitchHostsR Contributors`,
    },

    // 最后更新时间文本
    lastUpdated: true,
    lastUpdatedText: '上次更新时间',
    prevPageText: '上一篇',
    nextPageText: '下一篇',
    enableScrollToTop: true,
    searchPlaceholderText: '搜索文档',
    searchNoResultsText: '无法找到相关结果',
    searchSuggestedQueryText: '请使用不同的关键字重试',
    enableContentAnimation: true,
    enableAppearanceAnimation: true, // 是否启用内容动画
  },
};

export default config;
