<template>
  <n-drawer v-model:show="data.isShow" placement="right" :width="502" :on-after-leave="onClose">
    <n-drawer-content>
      <template #header>
        {{  title  }}
      </template>
      <!-- 表单 -->
      <n-form
        ref="formRef"
        :model="data.model"
        :rules="rules"
        label-placement="left"
        label-width="auto"
        require-mark-placement="right-hanging"
        size="medium"
      >
        <!-- hosts 类型 -->
        <n-form-item label="Hosts 类型" path="hostsType">
          <n-select
            v-model:value="data.model.hostsType"
            placeholder="请选择 Hosts 类型"
            :options="hostsTypeOptions"
          />
        </n-form-item>

        <!-- 名称 -->
        <n-form-item label="名称" path="title">
          <n-input v-model:value="data.model.title" placeholder="请输入名称" />
        </n-form-item>

        <!-- url -->
        <n-form-item
          v-show="isShowRemote"
          label="远程地址"
          path="url">
          <n-input v-model:value="data.model.url" placeholder="请输入远程地址" />
        </n-form-item>

        <!-- refresh 类型 -->
        <n-form-item
          path="refreshType"
          v-show="isShowRemote" label="刷新类型">
          <n-select
            v-model:value="data.model.refreshType"
            placeholder="请选择刷新类型"
            :options="refreshTypeOptions"
          />
        </n-form-item>
      </n-form>

      <template #footer>
        <n-button round type="dander" @click="onCancel">取消</n-button>
        <n-button round type="primary" @click="onSubmit">确定</n-button>
      </template>
    </n-drawer-content>
  </n-drawer>
</template>

<script lang="ts">
import { storeToRefs } from 'pinia'
import { useRouter } from 'vue-router'
import { useHostsStore } from '@/store/index'
import { addLocalHostsRoute } from './utils'
import { type FormInst, useMessage, type FormItemRule } from 'naive-ui'
import { ref, reactive, computed, watch, defineComponent, onMounted } from 'vue'

export default defineComponent({
  name: "ListDraw",
  setup(props, ctx) {
    const message = useMessage()
    const router = useRouter()
    const store = useHostsStore()
    const { isShowEditor, mode } = storeToRefs(store)
    const formRef = ref<FormInst | null>(null)

    const data = reactive<{
      isShow: boolean
      model: {
        title: string
        url: string
        hostsType: string | null
        refreshType: number | null
      }
    }>({
      isShow: false,
      model: {
        title: "",
        url: "",
        hostsType: null,
        refreshType: null
      },
    })

    const isShowRemote = computed(() => {
      return data.model.hostsType === 'remote'
    })

    const rules = computed(() => {
      let rule: { [key: string]: any } = {
        title: {
          required: true,
          trigger: ['blur', 'input'],
          message: '请输入名称'
        },
        hostsType: {
          required: true,
          trigger: ['blur', 'change'],
          message: '请选择 Hosts 类型'
        },
      }
      switch (data.model.hostsType) {
        case "remote":
          rule.url = {
            required: true,
            trigger: ['blur', 'input'],
            message: '请输入远程 Url 地址',
            validator(rule: FormItemRule, value: any) {
              const reg = /(http|https):\/\/([\w.]+\/?)\S*/
              if (!value) return new Error('请输入远程 Url 地址')
              return reg.test(value) ? '' : new Error('请输入正确的远程 Url 地址')
            }
          }
          rule.refreshType = {
            required: true,
            trigger: ['blur', 'change'],
            message: '请选择刷新类型',
            validator(rule: FormItemRule, value: any) {
              if(!value) return new Error('请选择刷新类型')
            }
          }
          break;

        default:
          break;
      }
      return rule
    })
    const title = computed(() => {
      return mode.value === 'create' ? '创建 Hosts' : '编辑 Hosts'
    })

    const onClose = () => {
      formRef.value?.restoreValidation()
      data.model = {
        title: "",
        url: "",
        hostsType: null,
        refreshType: null
      }
      store.hide()
    }

    const onSubmit = (event: MouseEvent) => {
      event.preventDefault()
      formRef.value?.validate((errors) => {
        if (!errors) {

          type modeType = "remote" | "local"
          addLocalHostsRoute({
            name: data.model.title,
            title: data.model.title,
            type: data.model.hostsType as modeType,
          }, (data) => {
            if (data) {
              message.success('创建成功')
              router.addRoute(data)
              setTimeout(() => {
                router.push({ name: data.name })
              }, 0);
              onClose()
            } else {
              message.success('创建失败')
            }
          })
        } else {
          message.error('创建失败')
        }
      })
    }

    const onCancel = () => {
      onClose()
    }

    onMounted(() => {
      watch(() => isShowEditor.value, () => {
        data.isShow = isShowEditor.value
      }, { immediate: true })
    })

    return {
      data,
      title,
      rules,
      formRef,
      onClose,
      onSubmit,
      onCancel,
      isShowRemote,
      hostsTypeOptions: [
        {
          label: "本地",
          value: "local"
        },
        {
          label: "远程",
          value: "remote"
        }
      ],
      refreshTypeOptions: [
        {
          label: "从不",
          value: 0
        },
        {
          label: "1 分钟",
          value: 1
        },
        {
          label: "5 分钟",
          value: 5
        },
        {
          label: "15 分钟",
          value: 15
        },
        {
          label: "1 小时",
          value: 60
        },
        {
          label: "24 小时",
          value: 60 * 24
        },
        {
          label: "7 天",
          value: 60 * 24 * 7
        },
      ]
    }
  },
})
</script>
