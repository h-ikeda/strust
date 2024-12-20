<template>
  <label>x: <input v-model="x"></label>
  <label>y: <input v-model="y"></label>
  <p>x + y = {{ sum }}</p>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { add } from '#wasm'

const x = ref(BigInt(0))
const y = ref(BigInt(0))
const sum = computed(() => add(x.value, y.value))
</script>
