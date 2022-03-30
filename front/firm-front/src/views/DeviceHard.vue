<script setup lang="ts">
import type { DeviceHard } from '@/models';
import { Api } from '@/models/api';
import { ref, watch, type Ref } from 'vue'
import { RouterLink } from 'vue-router'
const data: Ref<Array<DeviceHard>> = ref([])
const reload = ref(0)
watch(reload, async () => {
    Api.deviceHard().then(d => data.value = d).catch(e => alert(e))
})
reload.value += 1
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
                <td>代码</td>
                <td>类别</td>
                <td>蓝牙模块</td>
                <td>指纹模块</td>
                <td>stm模块</td>
                <td>描述</td>
            </tr>
        </thead>
        <tbody>
            <tr v-for="device in data">
                <td>{{ device.hard_version }}</td>
                <td>{{ device.name }}</td>
                <td>{{ device.category }}</td>
                <td
                    :class="{ green: device.has_ble, red: !device.has_ble }"
                >{{ device.has_ble ? "是" : "否" }}</td>
                <td
                    :class="{ green: device.has_finger, red: !device.has_finger }"
                >{{ device.has_finger ? "是" : "否" }}</td>
                <td
                    :class="{ green: device.has_stm32, red: !device.has_stm32 }"
                >{{ device.has_stm32 ? "是" : "否" }}</td>
                <td>{{ device.desc }}</td>
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
.green {
    color: green;
}
.red {
    color: darkred;
}
</style>