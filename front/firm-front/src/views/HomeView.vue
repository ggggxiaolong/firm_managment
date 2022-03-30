<script setup lang="ts">
import { ref, watch, watchEffect, type Ref } from 'vue'
import type { BaseInfo, Firm } from "@/models";
import { RouterLink } from 'vue-router'
import { Api } from "@/models/api";
const selectType: Ref<number> = ref(-1)
const data: Ref<Array<Firm>> = ref([]);
const baseInfo: Ref<null | BaseInfo> = ref(null);

watchEffect(async () => {
    Api.baseInfo().then(d => {
        baseInfo.value = d
        Api.firms().then(d => { data.value = d }).catch(e => alert(e))
    }).catch(e => alert(e))
})

watch(selectType, async () => {
    if (selectType.value === -1) {
        Api.firms().then(d => { data.value = d }).catch(e => alert(e))
    } else {
        Api.deviceFirms(selectType.value).then(d => { data.value = d }).catch(e => alert(e))
    }
})

function formatHard(id: number): string {
    const base = baseInfo.value
    if (base) {
        const hard = base.hard.find(v => v.id === id)
        if (hard) {
            return `${hard.name}`
        }
        return `${id}`
    }
    return `${id}`
}

function formatSoft(id: number): string {
    const base = baseInfo.value
    if (base) {
        const hard = base.soft.find(v => v.id === id)
        if (hard) {
            return hard.name
        }
        return `${id}`
    }
    return `${id}`
}

function formatFinger(type: number): string {
    if (!type) {
        return "All"
    } else {
        return `${type}`
    }
}

function formatTime(time: number): string {
    const date = new Date(time * 1000)
    return date.toLocaleString("zh-CN")
}
</script>

<template>
    <div>
        <nav>
            <RouterLink to="/hard">/硬件列表</RouterLink>
            <RouterLink to="/soft">/软件列表</RouterLink>
            <select v-model.number="selectType" v-if="baseInfo?.hard.length">
            <option value="-1">硬件类型</option>
            <option v-for="hard in baseInfo!!.hard" :value="hard.id">{{hard.name}}</option>
        </select>
        </nav>
    </div>
    <table>
        <thead>
            <tr>
                <td>硬件类型</td>
                <td>版本名</td>
                <td>格式化</td>
                <td>软件类型</td>
                <td>指纹版本</td>
                <td>URL</td>
                <td>描述</td>
                <td>更新时间</td>
                <td>关联版本</td>
                <td>最低版本</td>
                <td>最高版本</td>
                <td>英语</td>
                <td>韩语</td>
                <td>西班牙语</td>
            </tr>
        </thead>
        <tbody>
            <tr v-for="firm in data">
                <td>{{ formatHard(firm.hard_version) }}</td>
                <td>{{ firm.version_name }}</td>
                <td>{{ firm.version_format }}</td>
                <td>{{ formatSoft(firm.version_type) }}</td>
                <td>{{ formatFinger(firm.finger_level) }}</td>
                <td>
                    <a :href="firm.url" target="_blank">url</a>
                </td>
                <td>{{ firm.desc }}</td>
                <td>{{ formatTime(firm.update_time) }}</td>
                <td>{{ firm.rely_version_type ? formatSoft(firm.rely_version_type) : "" }}</td>
                <td>{{ firm.min || "" }}</td>
                <td>{{ firm.max || "" }}</td>
                <td>{{ firm.des_en || "" }}</td>
                <td>{{ firm.des_ko || "" }}</td>
                <td>{{ firm.des_sp || "" }}</td>
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
nav a {
    padding-right:1rem;
}
</style>
