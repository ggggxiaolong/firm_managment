<script setup lang="ts">
const porps = defineProps<{
    show: boolean;
}>();
defineEmits<{
    (e: "onClose"): void;
    (e: "onOk"): void;
}>();
</script>

<template>
    <Transition name="modal">
        <div v-if="show" class="modal-mask" @click="$emit('onClose')">
            <div class="modal-wrapper">
                <div class="modal-container">
                    <div class="modal-header">
                        <slot name="header"></slot>
                    </div>
                    <div class="modal-body">
                        <slot name="body"></slot>
                    </div>
                    <div class="modal-footer">
                        <slot name="footer"></slot>
                        <div class="modal-default-button">
                            <button @click="$emit('onOk')">是</button>
                            <a href="#" @click.prevent="$emit('onClose')">否</a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </Transition>
</template>
<style scoped>
.modal-body {
    margin: 20px 0;
}
.modal-default-button {
    width: 100%;
    display: flex;
    justify-content: end;
}
.modal-default-button button {
    margin-right: 16px;
}
</style>