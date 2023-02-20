<template>
  <div class="error-main box">
    <svg-icon name="404" size="300px" />
    <p class="not-found-text">你访问的页面不存在</p>
    <n-button
      round
      type="primary"
      @click="goHome">返回首页</n-button>
  </div>
</template>
<script lang="ts">
import { defineComponent } from "vue";
import { useRouter } from "vue-router";

export default defineComponent({
  setup () {
    const router = useRouter();

    return {
      goHome: () => {
        router.push("/");
      },
    };
  },
});
</script>

<style scope lang="stylus">
.error-main
  display flex
  flex-direction column
  justify-content center
  align-items center

  &.box
    height 100%

  & .not-found-text
    font-size 16px
    font-weight 600
</style>
