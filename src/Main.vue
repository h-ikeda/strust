<template>
  <p>{{ num }}</p>
</template>

<script setup lang="ts">
import { add } from 'strust'

const num = add(BigInt(3), BigInt(8))
</script>
