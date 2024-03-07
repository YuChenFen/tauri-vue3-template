<template>
    <div class="message-box">
        <TransitionGroup name="massage-list">
            <div v-for="message in messageArr.value" :key="message.props.id">
                <component :is="message"></component>
            </div>
        </TransitionGroup>
    </div>
</template>

<script setup>
import { ref } from 'vue';

const poprs = defineProps({
    messageArr: {
        type: Object,
        default: ref([])
    }
})
</script>

<style scoped>
.message-box {
    display: flex;
    flex-direction: column;
    gap: 10px;
    align-items: flex-end;
}

.massage-list-enter-active,
.massage-list-leave-active {
    transition: all 0.3s ease;
}

.massage-list-enter-from,
.massage-list-leave-to {
    opacity: 0;
    transform: translateX(30px);
}
</style>