<template>
    <div>
        <div style="color: rgb(var(--text-color));">
            <h1 style="margin-top: 10px;font-weight: 500;" class="user-select">图标</h1>
            <span
                style="display: block; width: 220px;height: 1px;background-color: rgba(var(--text-color), 0.1);"></span>
            <span>icon</span>
        </div>
        <div class="line">
            <div class="icon-group">
                <span v-for="icon in iconGroup" :key="icon">
                    <svg-icon :name="icon"></svg-icon>
                    <p class="user-select">{{ icon }}</p>
                </span>
            </div>
        </div>
    </div>
</template>

<script setup>
const iconGroup = [
    "tag",
    "eye",
    "up",
    "add",
    "home",
    "flag",
    "mail",
    "send",
    "save",
    "cut",
    "edit",
    "code",
    "link",
    "heart",
    "search",
    "download",
    "play",
    "music",
    "photo",
    "movie",
    "pause",
    "share",
    "message",
    "setting",
    "menu",
    "paste",
    "copy",
    "delete",
    "application",
    "microphone",
    "headphone",
    "megaphone",
    "checkbox",
    "radio",
    "slider",
    "switch",
    "chevronRight"
];
</script>

<style scoped>
.icon-group {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
}

.icon-group span {
    position: relative;
    width: 80px;
    height: 80px;
    padding: 10px;
    border-radius: 5px;
    border: 1px solid rgba(var(--app-color), .1);
    background-color: var(--app-background-color);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
}

.icon-group span p {
    position: absolute;
    bottom: 5px;
    font-size: 12px;
    color: rgb(var(--app-color));
}


.icon-group span:hover {
    background-color: var(--app-hover-background-color);
}

.icon-group :deep(svg) {
    width: 20px;
    height: 20px;
    fill: rgb(var(--app-color));
}
</style>