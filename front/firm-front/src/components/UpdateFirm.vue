<script setup lang="ts">
import type { BaseInfo, Firm } from '@/models';
import { Api } from '@/models/api';
import { ref, watch, type Ref } from 'vue'
const props = defineProps<{
    baseInfo: BaseInfo | null,
    firm: Firm | null,
}>()

const emits = defineEmits<{
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
const date = ref("")
watch(() => props.firm, (f) => {
    if (f) {
        hardVersion.value = f.hard_version
        versionName.value = f.version_name
        versionFormat.value = f.version_format
        versionType.value = f.version_type
        fingerLevel.value = f.finger_level
        desc.value = f.desc
        relyVersionType.value = f.rely_version_type || -1
        min.value = f.min || ""
        max.value = f.max || ""
        descEn.value = f.des_en
        descKo.value = f.des_ko
        descSp.value = f.des_sp
        date.value = currentTimeString(f.update_time)
        file.value = null
    }
})

watch(versionName, (v) => {
    const arr: Array<number> = [];
    for (let index = v.length - 2; index > -2; index -= 2) {
        const start = index > -1 ? index : 0
        arr.push(Number.parseInt(v.slice(start, index + 2), 16))
    }
    versionFormat.value = arr.join('.')
})

watch(versionType, (v) => {
    if (v !== 3) {
        fingerLevel.value = 0
    }
})

function currentTimeString(time: number): string {
    const date = new Date(time * 1000);
    const year = date.getFullYear();
    const month = date.getMonth() + 1;
    const day = date.getDate();
    const hours = date.getHours();
    const minutes = date.getMinutes();
    return `${year}-${month < 10 ? 0 : ''}${month}-${day < 10 ? 0 : ''}${day}T${hours < 10 ? 0 : ''}${hours}:${minutes < 10 ? 0 : ''}${minutes}`
}

async function onAdd() {
    if (hardVersion.value === 0 || versionName.value.length === 0
        || versionFormat.value.length === 0 || versionType.value === 0
        || desc.value.length === 0) {
        alert("请检查必填项")
        return
    }
    if (relyVersionType.value !== -1 && min.value.length === 0 && max.value.length === 0) {
        alert("如果有升级依赖必须填写最低或最高限制")
        return
    }
    const isRely = relyVersionType.value !== -1
    if (file.value == null) {
        const data: Firm = {
            id: props.firm!!.id,
            hard_version: hardVersion.value,
            version_name: versionName.value,
            version_format: versionFormat.value,
            version_type: versionType.value,
            finger_level: fingerLevel.value,
            url: props.firm!!.url,
            desc: desc.value,
            update_time: new Date(date.value).getTime() / 1000,
            rely_version_type: isRely ? relyVersionType.value : undefined,
            min: isRely ? min.value : undefined,
            max: isRely ? max.value : undefined,
            des_en: descEn.value,
            des_ko: descKo.value,
            des_sp: descSp.value,
        }
        emits('save', data)
    } else {
        if (file.value.size > 1024 * 1024 * 10) {
            alert("上传文件不能大于10M")
            return
        }
        Api.uploadFirm(file.value).then(d => {
            console.log(d)
            // const url = "https://res.cloudinary.com/xiaolong/image/upload/v1648697177/upload_test/wjpohopty4tygc6motio.png"
            const url = d.secure_url
            const data: Firm = {
                id: props.firm!!.id,
                hard_version: hardVersion.value,
                version_name: versionName.value,
                version_format: versionFormat.value,
                version_type: versionType.value,
                finger_level: fingerLevel.value,
                url: url,
                desc: desc.value,
                update_time: new Date(date.value).getTime() / 1000,
                rely_version_type: isRely ? relyVersionType.value : undefined,
                min: isRely ? min.value : undefined,
                max: isRely ? max.value : undefined,
                des_en: descEn.value,
                des_ko: descKo.value,
                des_sp: descSp.value,
            }
            emits('save', data)
        }).catch(e => {
            console.log(e)
            alert("文件上传失败,请重试")
        })
    }
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
        <div v-if="baseInfo && firm" class="modal-mask">
            <div class="modal-wrapper" @click.self="$emit('cancel')">
                <div class="modal-container">
                    <h3>修改新固件</h3>
                    <table>
                        <tr>
                            <td>
                                <span class="requier">*</span>硬件类型
                            </td>
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
                            <td>
                                <span class="requier">*</span>版本名
                            </td>
                            <td>
                                <input v-model.trim="versionName" id="version-name" />
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <span class="requier">*</span>格式化
                            </td>
                            <td>
                                <input v-model.trim="versionFormat" id="version-format" />
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <span class="requier">*</span>软件类型
                            </td>
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
                                <select
                                    v-model.number="fingerLevel"
                                    id="fingerLevel"
                                    :disabled="versionType !== 3"
                                >
                                    <option value="0">All</option>
                                    <option value="1">1</option>
                                    <option value="2">2</option>
                                </select>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <span class="requier">*</span>固件
                            </td>
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
                            <td>URL</td>
                            <td>
                                <a :href="firm.url" target="_blank">
                                    <div class="breakword">{{ firm.url }}</div>
                                </a>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <span class="requier">*</span>创建时间
                            </td>
                            <td>
                                <input type="datetime-local" v-model="date" />
                            </td>
                        </tr>
                        <tr>
                            <td class="borderTop">升级依赖</td>
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
                            <td class="borderMiddle">最低版本</td>
                            <td>
                                <input v-model.trim="min" id="min" placeholder="010001" />
                            </td>
                        </tr>
                        <tr>
                            <td class="borderBottom">最高版本</td>
                            <td>
                                <input v-model.trim="max" id="max" />
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <span class="requier">*</span>变更描述
                            </td>
                            <td>
                                <textarea v-model.trim="desc" rows="2"></textarea>
                            </td>
                        </tr>
                        <tr>
                            <td>英语描述</td>
                            <td>
                                <textarea v-model.trim="descEn" rows="2"></textarea>
                            </td>
                        </tr>
                        <tr>
                            <td>韩语描述</td>
                            <td>
                                <textarea v-model.trim="descKo" rows="2"></textarea>
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
                                <button @click="onAdd">更新</button>
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
.borderTop {
    border-top: 1px solid chocolate;
    border-left: 1px solid chocolate;
    border-right: 1px solid chocolate;
}
.borderMiddle {
    border-left: 1px solid chocolate;
    border-right: 1px solid chocolate;
}
.borderBottom {
    border-bottom: 1px solid chocolate;
    border-left: 1px solid chocolate;
    border-right: 1px solid chocolate;
}
.requier {
    color: red;
}
.breakword {
    width: 300px;
    overflow: hidden;
    text-overflow: ellipsis;
    direction: rtl;
    @supports (-webkit-line-clamp: 2) {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: initial;
        display: -webkit-box;
        -webkit-line-clamp: 2;
        -webkit-box-orient: vertical;
    }
}
</style>