<template>
  <VContainer>
    <VCard>
      <VCardTitle>
        <RouterLink to="/calculation-tools/beam">
          Beam
        </RouterLink>
      </VCardTitle>
    </VCard>
  </VContainer>
</template>

<script setup lang="ts">
import { VContainer } from 'vuetify/components/VGrid';
import { VCard, VCardTitle } from 'vuetify/components/VCard';
</script>
