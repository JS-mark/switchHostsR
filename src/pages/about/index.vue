<script lang="ts" setup>
import { onMounted, ref } from 'vue'
import { BrandGithub, Code, Heart, License } from '@vicons/tabler'

defineOptions({
  name: 'About',
})

const appVersion = ref('0.0.1')
const buildDate = ref(__BUILD_DATE__ || '未知')

/** 技术栈列表 */
const techStack = [
  { name: 'Tauri', version: '2.x', desc: '跨平台桌面框架' },
  { name: 'Vue', version: '3.5', desc: '前端框架' },
  { name: 'TypeScript', version: '5.x', desc: '类型安全' },
  { name: 'Naive UI', version: '2.x', desc: 'UI 组件库' },
  { name: 'Rust', version: 'stable', desc: '后端语言' },
  { name: 'Diesel', version: '2.1', desc: 'ORM (SQLite)' },
  { name: 'Vite', version: '7.x', desc: '构建工具' },
  { name: 'Pinia', version: '3.x', desc: '状态管理' },
  { name: 'Monaco Editor', version: '0.55', desc: '代码编辑器' },
  { name: 'UnoCSS', version: '0.66', desc: '原子化 CSS' },
]

/** 快捷链接 */
const links = [
  {
    label: 'GitHub 仓库',
    url: 'https://github.com/JS-mark/switchHostsR',
    icon: BrandGithub,
  },
  {
    label: '问题反馈',
    url: 'https://github.com/JS-mark/switchHostsR/issues',
    icon: Heart,
  },
]

function openLink(url: string) {
  window.open(url, '_blank')
}

onMounted(() => {
  // 未来可从 Tauri API 获取版本号
})
</script>

<template>
  <div class="about-page">
    <n-card :bordered="false">
      <!-- 应用标题 -->
      <div class="about-header">
        <h1 class="app-title">
          SwitchHostsR
        </h1>
        <p class="app-subtitle">
          简单易用的跨平台 Hosts 文件管理工具
        </p>
        <n-tag type="info" size="small">
          v{{ appVersion }}
        </n-tag>
      </div>

      <n-divider />

      <!-- 应用信息 -->
      <div class="about-section">
        <h3 class="section-title">
          应用信息
        </h3>
        <n-descriptions :column="2" label-placement="left" bordered>
          <n-descriptions-item label="版本">
            {{ appVersion }}
          </n-descriptions-item>
          <n-descriptions-item label="开源协议">
            MIT License
          </n-descriptions-item>
          <n-descriptions-item label="作者">
            Mark (sunduo3195@qq.com)
          </n-descriptions-item>
          <n-descriptions-item label="平台支持">
            macOS / Windows / Linux
          </n-descriptions-item>
        </n-descriptions>
      </div>

      <n-divider />

      <!-- 快捷链接 -->
      <div class="about-section">
        <h3 class="section-title">
          链接
        </h3>
        <n-space>
          <n-button
            v-for="link in links"
            :key="link.url"
            text
            type="primary"
            @click="openLink(link.url)"
          >
            <template #icon>
              <n-icon :component="link.icon" />
            </template>
            {{ link.label }}
          </n-button>
        </n-space>
      </div>

      <n-divider />

      <!-- 技术栈 -->
      <div class="about-section">
        <h3 class="section-title">
          <n-icon :component="Code" class="mr-1" />
          技术栈
        </h3>
        <n-grid :cols="2" :x-gap="12" :y-gap="8">
          <n-gi v-for="tech in techStack" :key="tech.name">
            <div class="tech-item">
              <span class="tech-name">{{ tech.name }}</span>
              <n-tag size="tiny" :bordered="false">
                {{ tech.version }}
              </n-tag>
              <span class="tech-desc">{{ tech.desc }}</span>
            </div>
          </n-gi>
        </n-grid>
      </div>

      <n-divider />

      <!-- 底部 -->
      <div class="about-footer">
        <n-icon :component="License" />
        <span>MIT License &copy; {{ new Date().getFullYear() }} Mark</span>
      </div>
    </n-card>
  </div>
</template>

<style lang="less" scoped>
.about-page {
  padding: 16px;
  max-width: 640px;
  margin: 0 auto;
}

.about-header {
  text-align: center;
  padding: 16px 0;

  .app-title {
    font-size: 22px;
    font-weight: 700;
    margin: 0 0 6px 0;
    background: linear-gradient(135deg, #18a058, #2080f0);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .app-subtitle {
    font-size: 13px;
    color: #999;
    margin: 0 0 8px 0;
  }
}

.about-section {
  .section-title {
    font-size: 14px;
    font-weight: 600;
    margin: 0 0 8px 0;
    display: flex;
    align-items: center;
    gap: 4px;
  }
}

.tech-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 0;
  font-size: 12px;

  .tech-name {
    font-weight: 600;
    min-width: 90px;
  }

  .tech-desc {
    color: #999;
    font-size: 11px;
  }
}

.about-footer {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  font-size: 12px;
  color: #999;
  padding: 12px 0;
}
</style>
