<script setup lang="ts">
const props = defineProps<{
  title: string;
}>();
</script>
<template>
  <div class="tooltip">
    <slot></slot>
    <span class="tooltiptext tooltip-bottom">
      <pre>{{ title }}</pre>
    </span>
  </div>
</template>
<style>
.tooltip {
    position: relative;
    display: inline-block;
}
.tooltip .tooltiptext {
    visibility: hidden;
    position: absolute;
    background-color: #555;
    color: #fff;
    text-align: center;
    padding: 8px;
    border-radius: 6px;
    z-index: 1;
    opacity: 0;
    transition: opacity .6s;
}
.tooltip:hover .tooltiptext {
    visibility: visible;
    opacity: 1;
}
.tooltip-bottom {
  top: 135%;
  left: 50%;  
  margin-left: -60px;
}
.tooltip-bottom::after {
    content: "";
    position: absolute;
    bottom: 100%;
    left: 50%;
    margin-left: -5px;
    border-width: 5px;
    border-style: solid;
    border-color: transparent transparent #555 transparent;
}
</style>
