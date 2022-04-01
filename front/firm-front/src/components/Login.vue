<script setup lang="ts">
import { ref } from 'vue'
import sha1 from 'sha1'
import { Api } from '@/models/api';
const porps = defineProps<{
    show: boolean;
}>();
const emit = defineEmits<{
    (e: "success"): void;
}>();
const mail = ref("");
const password = ref("");
function login() {
    if (mail.value.length > 0 && password.value.length > 0) {
        const pass = sha1(password.value)
        Api.login({ email: mail.value, password: pass }).then(d => { emit('success') }).catch(e => alert(e))
    } else {
        alert("帐号密码不能为空")
    }
}
</script>

<template>
    <Transition name="modal">
        <div v-if="show" class="modal-mask">
            <div class="modal-wrapper">
                <div class="modal-container">
                    <div class="root">
                        <h3>登陆</h3>
                        <label for="mail">Email</label>
                        <input id="mail" v-model.trim="mail" />
                        <label for="password">Password</label>
                        <input id="password" type="password" v-model="password" />
                        <button @click="login">Login</button>
                    </div>
                </div>
            </div>
        </div>
    </Transition>
</template>

<style scoped>
.root {
    padding-top: 16px;
    padding-bottom: 24px;
    padding-left: 24px;
    padding-right: 24px;
}
h3 {
    color: forestgreen;
    font-size: 20px;
}
label {
    display: block;
}
input {
    width: 100%;
    height: 40px;
    margin-bottom: 16px;
    font-size: 18px;
}
button {
    width: 100%;
    height: 48px;
    font-size: 18px;
}
</style>