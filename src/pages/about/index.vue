<template>
  <div>
    <span @click="open">{{  'By Mark'  }}</span>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { useHostsStore } from '@/store';
export default defineComponent({
  name: "About",
  setup(props) {
    const { show } = useHostsStore()
    return {
      open() {
        show({ mode: "create" })
      }
    }
  }
})
</script>
