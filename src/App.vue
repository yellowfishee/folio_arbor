<script setup lang="ts">
import {NNotificationProvider} from 'naive-ui'

</script>

<template>
  <n-notification-provider :placement="'bottom'">
    <router-view />
  </n-notification-provider>
</template>

<style scoped>

</style>