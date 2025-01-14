<template>
  <VContainer>
    <VCol cols="auto">
      <VCard>
        <VCardTitle>
          High performance
        </VCardTitle>
        <VCardText>
          powered by Webassembly.
        </VCardText>
      </VCard>
    </VCol><VCol cols="auto">
      <VCard>
        <VCardTitle>
          Flexible
        </VCardTitle>
        <VCardText>
          open source
        </VCardText>
      </VCard>
    </VCol><VCol cols="auto">
      <VCard>
        <VCardTitle>
          Cross platform
        </VCardTitle>
        <VCardText>
          web based system
        </VCardText>
      </VCard>
    </VCol>
  </VContainer>
</template>

<script setup lang="ts">
import { VContainer, VCol } from 'vuetify/components/VGrid'
import { VCard, VCardTitle, VCardText } from 'vuetify/components/VCard';
</script>
