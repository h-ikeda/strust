<template>
  <VContainer>
    <VTextField v-model="length" label="length" />
    <VTextField v-model="density" label="density" />
    {{ mul }}
  </VContainer>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { VContainer } from 'vuetify/components/VGrid'
import { VTextField } from 'vuetify/components/VTextField'
import { multiply } from '#wasm'

const length = ref('0')
const density = ref('0')

const mul = computed(() => multiply(length.value, density.value))
</script>
