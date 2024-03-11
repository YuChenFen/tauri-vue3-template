<template>
  <div class="input-content" :style="{ width : width }">
    <input :type="inputType" :value="value" @change="change" @input="input" :placeholder="placeholder"
      :style="{ 'padding-right': showClear ? '' : '15px' }">
    <div class="input-container">
      <div v-if="showClear" :style="{ visibility: value ? '' : 'hidden' }" class="clear" @click.stop="clear">
        <svg t="1709375294946" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="1614">
          <path
            d="M217.82708 851.418089m-22.627417-22.627417l0 0q-22.627417-22.627417 0-45.254834l588.312842-588.312842q22.627417-22.627417 45.254834 0l0 0q22.627417 22.627417 0 45.254834l-588.312842 588.312842q-22.627417 22.627417-45.254834 0Z"
            p-id="1615"></path>
          <path
            d="M172.572246 217.850413m22.627417-22.627417l0 0q22.627417-22.627417 45.254834 0l588.312842 588.312842q22.627417 22.627417 0 45.254834l0 0q-22.627417 22.627417-45.254834 0l-588.312842-588.312842q-22.627417-22.627417 0-45.254834Z"
            p-id="1616"></path>
        </svg>
      </div>
    </div>
    <div class="underline"></div>
    <slot name="suffix">
    </slot>
  </div>

</template>

<script setup>
import { type } from '@tauri-apps/api/os';

const props = defineProps({
  value: {
    type: String,
    default: ''
  },
  placeholder: {
    type: String,
    default: ''
  },
  inputType: {
    type: String,
    default: 'text'
  },
  showClear: {
    type: Boolean,
    default: false
  },
  width:{
    type: String,
    default: ''
  }
})

const emits = defineEmits(['update:value', 'change', 'input']);

const change = (e) => {
  emits('update:value', e.target.value);
  emits('change', e.target.value);
}
const input = (e) => {
  emits('update:value', e.target.value);
  emits('input', e.target.value);
}
const clear = () => {
  emits('update:value', '');
  emits('change', '');
}
</script>

<style scoped>
.input-content,
.input-container {
  position: relative;
  display: flex;
  align-items: center;
}

.input-content {
  border: 1px solid rgba(var(--app-color), .1);
  box-shadow: var(--app-shadow);
  border-radius: 2px;
  background-color: var(--app-background-color);
}

.input-content input {
  width: 100%;
  height: 32px;
  display: inline-block;
  padding: 8px 28px 8px 15px;
  background-color: #00000000;
  color: rgb(var(--app-color));
  letter-spacing: 1px;
  border: none;
}

.input-content .underline {
  position: absolute;
  bottom: 0;
  left: 0;
  height: 2px;
  width: 100%;
  border-radius: 2px;
  background-color: var(--app-highlight-background-color);
  transform: scaleX(0);
  transition: all 0.3s ease;
}

.input-content:has(> input:focus) {
  box-shadow: none;
}

.input-content input:focus~.underline {
  transform: scaleX(1);
}

.clear {
  position: absolute;
  display: flex;
  align-items: center;
  border-radius: 2px;
  overflow: hidden;
  right: 5px;
  visibility: hidden;
  transition: visibility 0.2s cubic-bezier(0, 0, 0, 1);
}

.clear svg {
  fill: #5b5b5b;
  width: 15px;
  height: 15px;
  margin: 2px;
}

.input-content input:focus~.input-container .clear {
  visibility: visible;
}

.clear:hover {
  background-color: rgba(var(--app-color), .05);
}

input[type="password"]::-ms-reveal {
  display: none
}
input[type="password"]{
  font-family:Arial;
  font-weight: 900;
  letter-spacing: 2px;
}
input[type="password"]::placeholder{
  font-size: 14px;
  letter-spacing: 0;
  font-weight: 400;
}
</style>