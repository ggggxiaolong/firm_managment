<script setup lang="ts">
import { projectInjetct } from '@/inject';
import type { DeviceSoft } from '@/models';
import { Api } from '@/models/api';
import { ref, watch, type Ref } from 'vue'
import { RouterLink } from 'vue-router'
const data: Ref<Array<DeviceSoft>> = ref([])
const reloadIndex = ref(0)
const {reload, handleError} = projectInjetct()
watch([reload, reloadIndex], async () => {
    Api.deviceSoft().then(d => data.value = d).catch(e => handleError(e))
})
reloadIndex.value += 1
</script>
<template>
    <div>
        <nav>
            <RouterLink to="/">/返回</RouterLink>
        </nav>
    </div>
    <table>
        <thead>
            <tr>
                <td>名称</td>
            </tr>
        </thead>
        <tbody>
            <tr v-for="device in data">
                <td>{{ device.name }}</td>
            </tr>
        </tbody>
    </table>
</template>
<style scoped>
table {
    width: 100%;
}
td {
    border-bottom: 1px solid slategray;
    border-left: 1px solid slategray;
}
thead td {
    border-top: 1px solid slategray;
}
tr td:last-child {
    border-right: 1px solid slategray;
}
</style>