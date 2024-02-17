<script lang="ts">
import { defineComponent, reactive } from 'vue'

export default defineComponent({
  name: 'GlobalModal',
  setup() {
    const data = reactive({
      show: false,
    })

    return {
      data,
    }
  },
})
</script>

<template>
  <n-modal :show="data.show">
    <n-card
      style="width: 600px"
      title="关于"
      size="huge"
      :bordered="false"
      role="dialog"
      aria-modal="true"
    >
      <p>ddd</p>
      <p><a href="https://js-mark.com/switch-host-r" /></p>
    </n-card>
  </n-modal>
</template>
