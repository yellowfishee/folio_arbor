<template>
  <n-layout class="layout" has-sider>
    <!-- 侧边栏 - 公共导航区域 -->
    <n-layout-sider
      bordered
      class="sidebar"
      :collapsed="collapsed"
      collapse-mode="width"
      :collapsed-width="70"
      @collapse="collapsed = true"
      @expand="collapsed = false"
      show-trigger
    >
    <!-- Todo 触发侧边栏时，应该直显示图标，但是还没制作图标，所以搁置 -->
      <div class="logo">
        <div v-if="!collapsed">Folio Arbor</div>
      </div>
      <n-menu
        :items="menuItems"
        :value="currentRoute"
        @update:value="handleMenuChange"
        :collapsed-width="70"
        :collapsed-icon-size="22"
      />
    </n-layout-sider>

    <!-- 主内容区 - 子路由出口 -->
    <n-layout>
      <!-- <n-layout-header bordered class="header">
        <n-breadcrumb :items="breadcrumbItems" />
      </n-layout-header> -->
      <n-layout-content class="main-content">
        <router-view />
      </n-layout-content>
    </n-layout>
  </n-layout>
</template>

<script setup lang="ts">
import { useRouter, useRoute } from "vue-router";
import { ref, watch, Component, h } from "vue";
import {
  BookOutline,
  GitNetworkOutline,
  SettingsOutline,
} from "@vicons/ionicons5";
import {
  NLayout,
  NLayoutSider,
  NMenu,
  NLayoutContent,
  MenuOption,
  NIcon,
} from "naive-ui";

const renderIcon = (icon: Component) => {
  return () => h(NIcon, null, { default: () => h(icon) });
};
const router = useRouter();
const route = useRoute();
const currentRoute = ref(route.name as string);
const collapsed = ref(false);

const menuItems: MenuOption[] = [
  {
    key: "NotesList",
    label: "文献笔记",
    type: "item",
    icon: renderIcon(BookOutline),
  },
  {
    key: "KnowledgeGraph",
    label: "知识图谱",
    type: "item",
    icon: renderIcon(GitNetworkOutline),
  },
  {
    key: "Settings",
    label: "设置",
    type: "item",
    icon: renderIcon(SettingsOutline),
  },
];

// 监听路由变化更新菜单选中状态
watch(route, (newRoute: any) => {
  currentRoute.value = newRoute.name as string;
});

const handleMenuChange = (key: string) => {
  router.push({ name: key });
};

// 动态生成面包屑
const breadcrumbItems = ref([{ label: "首页", key: "home" }]);
watch(
  route,
  (newRoute: any) => {
    breadcrumbItems.value = [
      { label: "首页", key: "home" },
      {
        label: newRoute.meta.title || newRoute.name,
        key: newRoute.name as string,
      },
    ];
  },
  { immediate: true }
);
</script>

<style scoped>
.layout {
  height: calc(100vh);
}
.sidebar {
  width: 240px;
}
.header {
  height: 60px;
  padding: 0 20px;
  display: flex;
  align-items: center;
}
.main-content {
  padding: 20px;
}
.logo {
  text-align: center;
  padding: 20px;
  font-weight: bold;
  font-size: large;
}
</style>
