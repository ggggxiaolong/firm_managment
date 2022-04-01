<script setup lang="ts">
import { ref } from "vue";
const mail = ref("");
const password = ref("");

function login() {}
</script>

<template>
  <div class="root">
    <label for="mail">Email</label>
    <input id="mail" v-model="mail" />
    <label for="password">Password</label>
    <input id="password" v-model="password" />
    <button @click="login">Login</button>
  </div>
</template>
<style scoped>
root{
  width: 100%;
}
label {
  display: block;
}
input {
  width: 100%;
}
button {
  width: 100%;
  height: 48px;
  font-size: 18px;
}
</style>
