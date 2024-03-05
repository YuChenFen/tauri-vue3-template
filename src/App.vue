<script setup>
import headerVue from './layout/_partial/header.vue';
import testVue from './components/test.vue'

</script>

<template>
  <div style="height: 100vh;width: 100vw;">
    <headerVue></headerVue>
    <div class="content">
      <testVue></testVue>
    </div>
  </div>
</template>

<style scoped>
.content{
  height: calc(100vh - 30px);
  width: 100vw;
  overflow-y: scroll;
}
</style>
