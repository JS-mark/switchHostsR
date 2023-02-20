<template>
  <editor
    v-model="content"
    :format="useFormat"
    class="editor"
  />
</template>

<style lang="stylus" scoped>
.editor
  width 100%
  height 100%
</style>

<script lang="ts">
import { ref, defineComponent } from 'vue'
export default defineComponent({
  name: "EditPage",
  setup(props, ctx) {
    const content = ref("")
    return {
      useFormat: true,
      content
    }
  }
})
</script>
