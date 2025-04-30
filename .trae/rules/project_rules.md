# Switch Hosts R 开发规范

## 技术栈

本项目基于以下技术栈进行开发：

- **前端框架**：Vue 3
- **UI 组件库**：Naive UI
- **构建工具**：Vite 6.x
- **桌面应用框架**：Tauri 2.x
- **系统语言**：Rust
- **样式处理**：SCSS、UnoCSS
- **测试工具**：Vitest
- **类型系统**：TypeScript

## 代码风格和结构

- 编写干净、可维护且技术上准确的 TypeScript 代码
- 优先使用函数式和声明式编程模式；避免使用类
- 注重迭代和模块化，遵循 DRY 原则，尽量减少代码重复
- 优先使用命名导出以保持一致性和可读性
- 文件结构：导出的组件、Composables、辅助函数、静态内容和类型

## 命名约定

- 组件名称使用 PascalCase 格式（如 `AddHosts.vue`）
- 文件名使用 kebab-case 格式（如 `add-hosts.vue`）
- 变量和函数使用 camelCase 格式（如 `getUserInfo`）
- 常量使用 UPPER_SNAKE_CASE 格式（如 `APP_NAME`）
- 类型和接口使用 PascalCase 格式（如 `interface UserInfo`）
- 布尔类型变量使用 is、has、should 等前缀（如 `isLoading`）

## Vue 组件开发规范

### Vue3 组件规范

- 使用`<script setup lang="ts">`语法
- 组件 props 和 emits 必须使用类型声明

```vue
<script setup lang="ts">
import { ref } from 'vue'

interface Props {
  message: string
  count?: number
}

defineProps<Props>()
defineEmits<{
  (e: 'update', value: string): void
  (e: 'submit'): void
}>()

const count = ref(0)
</script>
```

## UI 和样式

- 使用 SCSS 作为主要样式语言
- 遵循 BEM 命名规范
- 样式文件结构：

```scss
// 变量定义

// 混入
@mixin component-style {
  // 样式定义
}

// 组件样式
.#{$component-prefix} {
  &__element {
    // 元素样式
  }

  &--modifier {
    // 修饰符样式
  }
}
```

### 颜色系统规范

1. 核心原则

- 主题色的层级关系（如 hover、pressed 等状态）应保持一致

2. 色彩语义化

- 使用语义化的命名而不是具体的色值
- 颜色应当传达其用途而不是具体的外观
- 避免在组件中硬编码颜色值

3. 灵活配置

- 主题色系可以根据项目需求选择（不限定具体色系）
- 中性色系可以根据设计风格调整
- 可以通过配置文件统一修改整个应用的配色方案

4. 组件开发指南

- 优先使用 Element UI 的主题变量
- 确保所有状态（normal

5. 可访问性

- 确保文本和背景色的对比度符合 WCAG 标准
- 考虑色盲用户的可访问性需求

## TypeScript 使用

- 全面使用 TypeScript
- 优先使用 interface 而非 type
- 避免使用 enum，使用 const enum 或对象映射
- 确保类型安全，禁止使用 any
- 必要时才使用 unknown 类型
- 泛型命名规范：T（通用类型）、K（键类型）、V（值类型）

## 语法与格式

- 函数方法尽量使用箭头函数
- 在条件语句中避免不必要的大括号；对于简单语句使用简洁语法
- 部分代码规范参考 ./eslint.config.js

## 目录约定

- src/ - 前端 Vue 相关文件
  - components/ - 可复用组件
  - pages/ - 页面组件
  - store/ - Pinia 状态管理
  - utils/ - 工具函数
  - types/ - 类型定义
  - assets/ - 静态资源
  - layout/ - 布局组件
  - apis/ - API 接口定义
  - langs/ - 国际化文件
- src-tauri/ - Tauri 和 Rust 相关文件
  - src/ - Rust 源代码
  - Cargo.toml - Rust 依赖配置
- vite-plugin/ - Vite 插件配置
- tests/ - 单元测试
- docs/ - 项目文档

## 提交规范

- 使用 commitizen 和 cz-customizable 进行规范化提交
- 提交前使用 lint-staged 进行代码检查
- 遵循 Conventional Commits 规范

```text

## 优化说明

1. **结构优化**：
   - 整合了项目技术栈，更清晰地展示了核心技术
   - 优化了目录结构说明，与实际项目结构保持一致
   - 添加了提交规范部分

2. **内容完善**：
   - 完善了 Vue 组件示例，添加了 `defineOptions` 的使用
   - 修正了颜色系统规范中关于 UI 库的引用（从 Element UI 改为 Naive UI）
   - 添加了更详细的目录结构说明

3. **格式优化**：
   - 使用更清晰的标题层级和分组
   - 添加了代码示例的注释说明
   - 统一了文档风格和术语

这份优
化后的规范文件更加符合项目的实际情况，并提供了更清晰的指导，有助于团队成员理解和遵循项目的开发规范。
```
