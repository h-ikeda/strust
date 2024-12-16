<template>
  <h1>strust</h1>
  <p>{{ num }}</p>
</template>

<script setup lang="ts">
import { add } from '~/pkg'

const num = add(BigInt(3), BigInt(8))
</script>
