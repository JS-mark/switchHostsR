<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

import type { Hosts } from '@/apis'

import { getAllHosts } from '@/apis'
import { getNowtime } from '@/utils/time'

defineOptions({
  name: 'Home',
})

const router = useRouter()
const clock = getNowtime()
const loading = ref(false)
const hostsStats = ref({
  total: 0,
  active: 0,
  system: 0,
})
const recentHosts = ref<Hosts[]>([])

// 获取 hosts 统计信息
async function getHostsStats() {
  loading.value = true
  try {
    const res = await getAllHosts()
    if (res.code === 200) {
      const hosts = res.data || []
      hostsStats.value = {
        total: hosts.length,
        active: hosts.filter(h => h.is_active === 1).length,
        system: hosts.filter(h => h.is_system === 1).length,
      }
      // 获取最近的3个hosts文件
      recentHosts.value = hosts.slice(0, 3)
    }
  }
  catch (error) {
    console.error('获取hosts统计失败:', error)
  }
  finally {
    loading.value = false
  }
}

// 快速操作
const quickActions = [
  {
    id: 'manage-hosts',
    title: '管理 Hosts',
    description: '查看和编辑所有 hosts 文件',
    icon: 'list',
    action: () => router.push('/hosts-list'),
  },
  {
    id: 'add-hosts',
    title: '添加新配置',
    description: '创建新的 hosts 配置文件',
    icon: 'add',
    action: () => router.push('/hosts-list'),
  },
  {
    id: 'view-logs',
    title: '查看日志',
    description: '查看操作历史和系统日志',
    icon: 'history',
    action: () => router.push('/admin'),
  },
]

function navigateToHosts(hostId: number) {
  router.push(`/edit/${hostId}`)
}

onMounted(() => {
  getHostsStats()
})
</script>

<template>
  <div class="home-container">
    <!-- 欢迎区域 -->
    <div class="welcome-section">
      <div class="welcome-content">
        <SvgIcon name="logo" size="64px" />
        <h1 class="welcome-title">
          Switch Hosts R
        </h1>
        <p class="welcome-subtitle">
          简单易用的 hosts 文件管理工具
        </p>
        <div class="time-display">
          <p class="date">
            {{ clock.date }}
          </p>
          <p class="time">
            {{ clock.time }}
          </p>
        </div>
      </div>
    </div>

    <!-- 状态概览 -->
    <div class="stats-section">
      <h2 class="section-title">
        状态概览
      </h2>
      <n-spin :show="loading">
        <div class="stats-grid">
          <n-card class="stat-card" hoverable>
            <div class="stat-content">
              <div class="stat-number">
                {{ hostsStats.total }}
              </div>
              <div class="stat-label">
                总配置数
              </div>
            </div>
            <div class="stat-icon">
              <n-icon size="24" color="#18a058">
                <svg viewBox="0 0 24 24">
                  <path fill="currentColor" d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-5 14H7v-2h7v2zm3-4H7v-2h10v2zm0-4H7V7h10v2z" />
                </svg>
              </n-icon>
            </div>
          </n-card>

          <n-card class="stat-card" hoverable>
            <div class="stat-content">
              <div class="stat-number">
                {{ hostsStats.active }}
              </div>
              <div class="stat-label">
                已启用
              </div>
            </div>
            <div class="stat-icon">
              <n-icon size="24" color="#2080f0">
                <svg viewBox="0 0 24 24">
                  <path fill="currentColor" d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z" />
                </svg>
              </n-icon>
            </div>
          </n-card>

          <n-card class="stat-card" hoverable>
            <div class="stat-content">
              <div class="stat-number">
                {{ hostsStats.system }}
              </div>
              <div class="stat-label">
                系统配置
              </div>
            </div>
            <div class="stat-icon">
              <n-icon size="24" color="#f0a020">
                <svg viewBox="0 0 24 24">
                  <path fill="currentColor" d="M18 8h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zm-6 9c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm3.1-9H8.9V6c0-1.71 1.39-3.1 3.1-3.1 1.71 0 3.1 1.39 3.1 3.1v2z" />
                </svg>
              </n-icon>
            </div>
          </n-card>
        </div>
      </n-spin>
    </div>

    <!-- 快速操作 -->
    <div class="actions-section">
      <h2 class="section-title">
        快速操作
      </h2>
      <div class="actions-grid">
        <n-card
          v-for="action in quickActions"
          :key="action.title"
          class="action-card"
          hoverable
          :data-testid="`home-action-${action.id}`"
          @click="action.action"
        >
          <div class="action-content">
            <div class="action-icon">
              <n-icon size="22" color="#18a058">
                <svg v-if="action.icon === 'list'" viewBox="0 0 24 24">
                  <path fill="currentColor" d="M3 13h2v-2H3v2zm0 4h2v-2H3v2zm0-8h2V7H3v2zm4 4h14v-2H7v2zm0 4h14v-2H7v2zM7 7v2h14V7H7z" />
                </svg>
                <svg v-else-if="action.icon === 'add'" viewBox="0 0 24 24">
                  <path fill="currentColor" d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z" />
                </svg>
                <svg v-else-if="action.icon === 'history'" viewBox="0 0 24 24">
                  <path fill="currentColor" d="M13 3c-4.97 0-9 4.03-9 9H1l3.89 3.89.07.14L9 12H6c0-3.87 3.13-7 7-7s7 3.13 7 7-3.13 7-7 7c-1.93 0-3.68-.79-4.94-2.06l-1.42 1.42C8.27 19.99 10.51 21 13 21c4.97 0 9-4.03 9-9s-4.03-9-9-9zm-1 5v5l4.28 2.54.72-1.21-3.5-2.08V8H12z" />
                </svg>
              </n-icon>
            </div>
            <div class="action-text">
              <h3 class="action-title">
                {{ action.title }}
              </h3>
              <p class="action-description">
                {{ action.description }}
              </p>
            </div>
          </div>
        </n-card>
      </div>
    </div>

    <!-- 最近使用的配置 -->
    <div v-if="recentHosts.length > 0" class="recent-section">
      <h2 class="section-title">
        最近使用
      </h2>
      <div class="recent-grid">
        <n-card
          v-for="host in recentHosts"
          :key="host.id"
          class="recent-card"
          hoverable
          @click="navigateToHosts(host.id)"
        >
          <div class="recent-content">
            <div class="recent-info">
              <h3 class="recent-name">
                {{ host.name }}
              </h3>
              <p v-if="host.description" class="recent-path">
                {{ host.description }}
              </p>
              <div class="recent-meta">
                <n-tag v-if="host.is_system === 1" type="warning" size="small">
                  系统
                </n-tag>
                <n-tag v-if="host.is_active === 1" type="success" size="small">
                  已启用
                </n-tag>
                <n-tag v-else type="default" size="small">
                  未启用
                </n-tag>
              </div>
            </div>
          </div>
        </n-card>
      </div>
    </div>
  </div>
</template>

<style lang="less" scoped>
.home-container {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
  min-height: calc(100vh - 142px);
  overflow-y: auto;
}

.welcome-section {
  text-align: center;
  padding: 24px 0;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 12px;
  margin-bottom: 20px;
  color: white;

  .welcome-content {
    .welcome-title {
      font-size: 1.6rem;
      font-weight: 700;
      margin: 10px 0 4px;
      text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    }

    .welcome-subtitle {
      font-size: 0.95rem;
      opacity: 0.9;
      margin-bottom: 16px;
    }

    .time-display {
      font-family: "Share Tech Mono", monospace;

      .date {
        font-size: 0.9rem;
        margin: 4px 0 2px;
        opacity: 0.8;
      }

      .time {
        font-size: 1.4rem;
        font-weight: 600;
        letter-spacing: 0.05em;
        margin: 0;
      }
    }
  }
}

.section-title {
  font-size: 1.1rem;
  font-weight: 600;
  margin-bottom: 12px;
  color: #333;
}

.stats-section {
  margin-bottom: 20px;

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
    gap: 12px;

    .stat-card {
      cursor: default;

      :deep(.n-card__content) {
        padding: 14px;
        display: flex;
        justify-content: space-between;
        align-items: center;
      }

      .stat-content {
        .stat-number {
          font-size: 1.5rem;
          font-weight: 700;
          color: #18a058;
          line-height: 1;
        }

        .stat-label {
          font-size: 0.8rem;
          color: #666;
          margin-top: 2px;
        }
      }

      .stat-icon {
        opacity: 0.7;
      }
    }
  }
}

.actions-section {
  margin-bottom: 20px;

  .actions-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 12px;

    .action-card {
      cursor: pointer;
      transition: all 0.2s ease;

      &:hover {
        transform: translateY(-1px);
        box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
      }

      :deep(.n-card__content) {
        padding: 14px;
      }

      .action-content {
        display: flex;
        align-items: center;
        gap: 12px;

        .action-icon {
          flex-shrink: 0;
          width: 36px;
          height: 36px;
          display: flex;
          align-items: center;
          justify-content: center;
          background: rgba(24, 160, 88, 0.1);
          border-radius: 8px;
        }

        .action-text {
          flex: 1;

          .action-title {
            font-size: 0.95rem;
            font-weight: 600;
            margin: 0 0 2px;
            color: #333;
          }

          .action-description {
            font-size: 0.8rem;
            color: #666;
            margin: 0;
            line-height: 1.4;
          }
        }
      }
    }
  }
}

.recent-section {
  .recent-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: 12px;

    .recent-card {
      cursor: pointer;
      transition: all 0.2s ease;

      &:hover {
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
      }

      :deep(.n-card__content) {
        padding: 14px;
      }

      .recent-content {
        .recent-info {
          .recent-name {
            font-size: 0.95rem;
            font-weight: 600;
            margin: 0 0 6px;
            color: #333;
          }

          .recent-path {
            font-size: 0.8rem;
            color: #666;
            margin: 0 0 8px;
            font-family: monospace;
            background: #f5f5f5;
            padding: 3px 6px;
            border-radius: 4px;
            word-break: break-all;
          }

          .recent-meta {
            display: flex;
            gap: 6px;
            flex-wrap: wrap;
          }
        }
      }
    }
  }
}

@media (max-width: 768px) {
  .home-container {
    padding: 12px;
  }

  .welcome-section {
    padding: 16px 12px;

    .welcome-content {
      .welcome-title {
        font-size: 1.3rem;
      }

      .time-display .time {
        font-size: 1.1rem;
      }
    }
  }

  .stats-grid,
  .actions-grid,
  .recent-grid {
    grid-template-columns: 1fr;
  }
}
</style>
