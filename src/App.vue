<script setup>
import headerVue from './layout/_partial/header.vue';
import demo from './components/demo/index.vue'

</script>

<template>
  <div style="height: 100vh;width: 100vw;">
    <headerVue></headerVue>
    <div class="content">
      <demo></demo>
    </div>
  </div>
</template>

<style scoped>
.content{
  height: calc(100vh - 30px);
  width: 100vw;
  overflow-y: scroll;
  overflow-x: hidden;
}
</style>
