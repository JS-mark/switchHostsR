<template>
  <div class="flex flex-col">
    <h4 class="my-10px">
      {{ title }}
    </h4>
    <slot></slot>
  </div>
</template>

<script lang="ts" setup>
defineOptions({
  name: 'AdvancedCell',
})

const props = defineProps<{
  title: string
}>()
</script>
