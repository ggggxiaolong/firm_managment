<script setup lang="ts">
import { RouterView } from 'vue-router'
import { provide, ref } from 'vue';
import { ApiError } from './models';
import { PROJECT_INJECT, type ProjectInject } from './inject';
import Login from './components/Login.vue';

const showLogin = ref(false)
const reload = ref(0)

function handleError(e: any) {
  console.log(e)
  if (e instanceof ApiError && e.status === 401) {
    showLogin.value = true
  } else {
    alert(e)
  }
}

function onLoginSuccess() {
  showLogin.value = false
  reload.value += 1
}

provide<ProjectInject>(PROJECT_INJECT, { reload, handleError })
</script>

<template>
  <RouterView />
  <Teleport to="body">
    <Login :show="showLogin" @success="onLoginSuccess" />
  </Teleport>
</template>

<style>
@import "@/assets/base.css";

#app {
  margin: 0 auto;
  padding: 2rem;
  font-weight: normal;
}
a {
  color: green;
}
.modal-wrapper {
  display: table-cell;
  vertical-align: middle;
}
.modal-header h3 {
  margin-top: 0;
  color: #42b983;
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
</style>
