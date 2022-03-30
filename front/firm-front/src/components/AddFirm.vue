<script setup lang="ts">
import type { BaseInfo, Firm } from '@/models';
import { ref, type Ref } from 'vue'
const props = defineProps<{
    baseInfo: BaseInfo | null,
    show: boolean,
}>()

defineEmits<{
    (e: "save", firm: Firm): void,
    (e: "cancel"): void
}>()

const hardVersion = ref(0)
const versionName = ref("")
const versionFormat = ref("")
const versionType = ref(0)
const fingerLevel = ref(0)
const desc = ref("")
const file: Ref<File | null> = ref(null)
const relyVersionType = ref(-1)
const min = ref("")
const max = ref("")
const descEn = ref("")
const descKo = ref("")
const descSp = ref("")
function onAdd() {

}

function onSelectFile(e: Event) {
    const f = (e.target as HTMLInputElement).files
    if (f) {
        file.value = f[0]
    } else {
        file.value = null
    }

}
</script>
<template>
    <Transition name="modal">
        <div v-if="show && baseInfo" class="modal-mask">
            <div class="modal-wrapper">
                <div class="modal-container">
                    <h3>添加新固件</h3>
                    <table>
                        <tr>
                            <td>硬件类型</td>
                            <td>
                                <select v-model.number="hardVersion" id="hard-version">
                                    <option value="0">--</option>
                                    <option
                                        v-for="hard in baseInfo.hard"
                                        :value="hard.id"
                                    >{{ hard.name }}</option>
                                </select>
                            </td>
                        </tr>
                        <tr>
                            <td>版本名</td>
                            <td>
                                <input v-model="versionName" id="version-name" />
                            </td>
                        </tr>
                        <tr>
                            <td>格式化</td>
                            <td>
                                <input v-model="versionFormat" id="version-format" />
                            </td>
                        </tr>
                        <tr>
                            <td>软件类型</td>
                            <td>
                                <select v-model.number="versionType" id="soft-version">
                                    <option value="0">--</option>
                                    <option
                                        v-for="soft in baseInfo.soft"
                                        :value="soft.id"
                                    >{{ soft.name }}</option>
                                </select>
                            </td>
                        </tr>
                        <tr>
                            <td>指纹版本</td>
                            <td>
                                <select v-model.number="fingerLevel" id="fingerLevel">
                                    <option value="0">All</option>
                                    <option value="1">1</option>
                                    <option value="2">2</option>
                                </select>
                            </td>
                        </tr>
                        <tr>
                            <td>固件</td>
                            <td>
                                <input
                                    type="file"
                                    id="file"
                                    @change="onSelectFile"
                                    multiple="false"
                                />
                            </td>
                        </tr>
                        <tr>
                            <td>关联版本</td>
                            <td>
                                <select v-model.number="relyVersionType" id="relyVersionType">
                                    <option value="-1">--</option>
                                    <option
                                        v-for="soft in baseInfo.soft"
                                        :value="soft.id"
                                    >{{ soft.name }}</option>
                                </select>
                            </td>
                        </tr>
                        <tr>
                            <td>最低版本</td>
                            <td>
                                <input v-model="min" id="min" />
                            </td>
                        </tr>
                        <tr>
                            <td>最高版本</td>
                            <td>
                                <input v-model="max" id="max" />
                            </td>
                        </tr>
                        <tr>
                            <td>变更描述</td>
                            <td>
                                <textarea v-model="desc" rows="2"></textarea>
                            </td>
                        </tr>
                        <tr>
                            <td>英语描述</td>
                            <td>
                                <textarea v-model="descEn" rows="2"></textarea>
                            </td>
                        </tr>
                        <tr>
                            <td>韩语描述</td>
                            <td>
                                <textarea v-model="descKo" rows="2"></textarea>
                            </td>
                        </tr>
                        <tr>
                            <td>西班牙语描述</td>
                            <td>
                                <textarea v-model="descSp" rows="2"></textarea>
                            </td>
                        </tr>
                        <tr>
                            <td></td>
                            <td class="button">
                                <button>添加</button>
                                <button class="outlineButton" @click="$emit('cancel')">取消</button>
                            </td>
                        </tr>
                    </table>
                </div>
            </div>
        </div>
    </Transition>
</template>
<style scoped>
.modal-wrapper {
    display: table-cell;
    vertical-align: middle;
}

.modal-container {
    width: 600px;
    margin: 0px auto;
    padding: 20px 30px;
    background-color: #fff;
    border-radius: 2px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.33);
    transition: all 0.3s ease;
}
.modal-mask {
    position: fixed;
    z-index: 9998;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: table;
    transition: opacity 0.3s ease;
}
.modal-enter-from {
    opacity: 0;
}

.modal-leave-to {
    opacity: 0;
}

.modal-enter-from .modal-container,
.modal-leave-to .modal-container {
    -webkit-transform: scale(1.1);
    transform: scale(1.1);
}
input {
    width: 300px;
    height: 24px;
}
textarea {
    width: 300px;
}
h3 {
    color: forestgreen;
}
.button {
    text-align: right;
}
table {
    width: 100%;
}
.outlineButton {
    border: 0;
    background-color: transparent;
    text-decoration: underline;
    margin-left: 8px;
}
</style>