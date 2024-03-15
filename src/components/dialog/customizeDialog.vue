<template>
    <teleport to="#app">
        <dialog ref="dialog">
            <slot name="body"></slot>
            <footer>
                <slot name="footer">
                    <div class="footer-wrapper">
                        <button class="text-button sure" @click.stop="ok">
                            <p>确认</p>
                        </button>
                        <button class="text-button cancel" @click.stop="no">
                            <p>取消</p>
                        </button>
                    </div>
                </slot>
            </footer>
        </dialog>
    </teleport>
</template>

<script setup>
import { ref, watch } from 'vue';

const props = defineProps({
    open: {
        type: Boolean,
        default: false
    }
})
const emits = defineEmits(['update:open', 'ok', 'no']);
const dialog = ref(null);
watch(() => props.open, (value) => {
    if (dialog.value) {
        if (value) {
            dialog.value.showModal();
        } else {
            dialog.value.close();
        }
    }
}, { immediate: true, deep: true });

const close = () => {
    emits('update:open', false);
}
function ok() {
    emits('ok');
    close();
}
function no() {
    emits('no');
    close();
}
</script>

<style scoped>
dialog {
    background: var(--app-card-content-background-color);
    position: fixed;
    left: 0;
    top: 0;
    right: 0;
    bottom: 0;
    margin: auto;
    width: 80%;
    box-shadow: var(--app-shadow);
    border-radius: 5px;
    border: 1px solid rgba(var(--app-color), .1);
    color: rgb(var(--app-color));
}

dialog::backdrop {
    background-color: rgba(0, 0, 0, 0.2);
    /* backdrop-filter: blur(2px); */
}

dialog footer {
    background-color: var(--app-card-footer-background-color);
}

.footer-wrapper {
    width: 100%;
    display: flex;
    justify-content: space-around;
    gap: 15px;
    border-top: 1px solid rgba(var(--app-color), .1);
    padding: 20px 15px;
}

.text-button {
    display: inline-block;
    padding: 6px 10px;
    border: 1px solid rgba(var(--app-color), .1);
    box-shadow: var(--app-shadow);
    border-radius: 5px;
    cursor: pointer;
    flex: 1;
}

.text-button:active {
    color: var(--app-hover-color);
    box-shadow: none;
}

.text-button p {
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
    font-size: 14px;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.sure {
    background-color: var(--app-highlight-background-color);
    color: var(--app-highlight-text-color);
}

.sure:hover {
    background-color: var(--app-highlight-hover-background-color);
}

.cancel {
    background-color: var(--app-background-color);
    color: rgb(var(--app-color));
}

.cancel:hover {
    background-color: var(--app-hover-background-color);
}
</style>