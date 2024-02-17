<script lang="ts">
import { defineComponent } from 'vue'
import { getNowtime } from '@/utils/time'

export default defineComponent({
  name: 'Home',
  setup() {
    const clock = getNowtime()

    return {
      clock,
    }
  },
})
</script>

<template>
  <div class="clock">
    <SvgIcon name="logo" size="240px" />
    <p class="date">
      {{ clock.date }}
    </p>
    <p class="time">
      {{ clock.time }}
    </p>
  </div>
</template>

<style lang="less" scoped>
.clock {
  display: flex;
  height: 100%;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  font-family: "Share Tech Mono", monospace;
  text-shadow: 2px 2px 10px rgba(7, 7, 7, 0.7);

  & .time {
    letter-spacing: 0.05em;
    font-size: 80px;
    padding: 5px 0;
    margin: 0;
  }
  & .date {
    letter-spacing: 0.1em;
    font-size: 24px;
    margin-top: 10px;
    font-weight: 600;
  }
  & .text {
    letter-spacing: 0.1em;
    font-size: 16px;
    padding: 20px 0 0;

    & .link {
      text-decoration: none;
    }
  }
}
</style>
