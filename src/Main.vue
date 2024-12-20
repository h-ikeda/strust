<template>
  <VResponsive>
  <VApp>
    <VSystemBar>
      strust v{{ version }}
    </VSystemBar>
    <VNavigationDrawer>
      <VListItem>
        <RouterLink to="/">Home</RouterLink>
      </VListItem>
      <VListItem>
        <RouterLink to="/calculation-tools">Calculation Tools</RouterLink>
      </VListItem>
    </VNavigationDrawer>
    <VMain>
      <VContainer>
        <RouterView/>
      </VContainer>
    </VMain>
  </VApp>
</VResponsive>
</template>

<script setup lang="ts">
import { VResponsive } from 'vuetify/components/VResponsive';
import { VContainer } from 'vuetify/components/VGrid'
import { VSystemBar } from 'vuetify/components/VSystemBar';
import { VApp } from 'vuetify/components/VApp'
import { VNavigationDrawer } from 'vuetify/components/VNavigationDrawer';
import { VListItem } from 'vuetify/components/VList';
import { VMain } from 'vuetify/components/VMain';
import { version } from '../package.json';
</script>

<style lang="scss">
@use 'vuetify'
</style>
