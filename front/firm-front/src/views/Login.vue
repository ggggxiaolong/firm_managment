<script setup lang="ts">
import { ref } from "vue";
const mail = ref("");
const password = ref("");

function login() {}
</script>

<template>
  <div>
    <label for="mail">Email</label>
    <input id="mail" v-model="mail" />
    <label for="password">Password</label>
    <input id="password" v-model="password" />
    <button @click="login">Login</button>
  </div>
</template>
