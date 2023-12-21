<script lang="ts">
import { defineComponent, ref } from 'vue'

export default defineComponent({
  name: 'EditPage',
  setup() {
    const content = ref('')
    return {
      useFormat: true,
      content,
    }
  },
})
</script>

<template>
  <editor v-model="content" :format="useFormat" class="editor" />
</template>

<style lang="stylus" scoped>
.editor
  width 100%
  height 100%
</style>
