<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 1024 1024"
    width="12"
    height="12"
    fill="currentColor"
    class="icon-loading"
  >
    <path
      d="M608 96c0 53.02-42.98 96-96 96s-96-42.98-96-96 42.98-96 96-96 96 42.98 96 96z m-96 736c-53.02 0-96 42.98-96 96s42.98 96 96 96 96-42.98 96-96-42.98-96-96-96z m416-416c-53.02 0-96 42.98-96 96s42.98 96 96 96 96-42.98 96-96-42.98-96-96-96zM192 512c0-53.02-42.98-96-96-96S0 458.98 0 512s42.98 96 96 96 96-42.98 96-96z m25.844 198.156c-53.02 0-96 42.98-96 96s42.98 96 96 96 96-42.98 96-96c0-53.018-42.982-96-96-96z m588.312 0c-53.02 0-96 42.98-96 96s42.98 96 96 96 96-42.98 96-96c0-53.018-42.98-96-96-96zM217.844 121.844c-53.02 0-96 42.98-96 96s42.98 96 96 96 96-42.98 96-96-42.982-96-96-96z"
    />
  </svg>
</template>
<style>
.icon-loading {
  -webkit-animation: fa-spin 2s infinite linear;
  animation: fa-spin 2s infinite linear;
}

@keyframes icon-loading {
  0% {
    -webkit-transform: rotate(0deg);
    transform: rotate(0deg);
  }
  100% {
    -webkit-transform: rotate(359deg);
    transform: rotate(359deg);
  }
}
</style>
