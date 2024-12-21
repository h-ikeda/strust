<template>
  <VResponsive>
  <VApp>
    <VSystemBar>
      strust v{{ version }}
    </VSystemBar>
    <VNavigationDrawer v-model="drawerOpen">
      <VListItem>
        <RouterLink to="/">Home</RouterLink>
      </VListItem>
      <VListItem>
        <RouterLink to="/calculation-tools">Calculation Tools</RouterLink>
      </VListItem>
    </VNavigationDrawer>
    <VAppBar>
      <VAppBarNavIcon @click="drawerOpen = !drawerOpen" v-show="!drawerOpen"/>
      <VAppBarTitle>
        strust
      </VAppBarTitle>
    </VAppBar>
    <VMain>
      <RouterView/>
    </VMain>
  </VApp>
</VResponsive>
</template>

<script setup lang="ts">
import { VResponsive } from 'vuetify/components/VResponsive'
import { VSystemBar } from 'vuetify/components/VSystemBar'
import { VApp } from 'vuetify/components/VApp'
import { VNavigationDrawer } from 'vuetify/components/VNavigationDrawer'
import { VListItem } from 'vuetify/components/VList'
import { VAppBar, VAppBarTitle, VAppBarNavIcon } from 'vuetify/components/VAppBar'
import { VMain } from 'vuetify/components/VMain'
import { version } from '../package.json'
import { ref } from 'vue'

const drawerOpen = ref(null)
</script>

<style lang="scss">
@use 'vuetify'
</style>
