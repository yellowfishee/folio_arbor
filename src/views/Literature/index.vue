<template>
  <div class="literature-container">
    <!-- 固定顶部的文献笔记输入框 -->
    <div class="editor-container" :style="editorStyle">
      <h3>添加文献笔记</h3>
      <TiptapEditor v-model="content" :publish="handlePublish"/>
    </div>

    <!-- 文献笔记卡片流 -->
    <div class="notes-feed" v-for="note in notes" :key="note.id">
      <NCard class="notes-item" :bordered="false">
        <!--     用小字显示update_time     -->
        <div class="notes-head">
          <div class="text-sm text-gray-500 text-center">
            发布时间：{{ formatDate(note.create_time) }}
          </div>
          <div>
            <n-popover>
              <template #trigger>
                <n-icon>
                  <MoreHorizSharp/>
                </n-icon>
              </template>
              <div class="popover-content">
                <!--       弹出菜单栏         -->
                <div class="action-buttons">
                  <!--           操作       -->
                  <n-button size="small" text class="action-button">编辑</n-button>
                  <n-button size="small" text class="action-button" @click="handleDelete(note.id)">删除</n-button>
                </div>
                <n-hr/>
                <div class="text-sm text-gray-500 text-center">
                  <!--            基本信息      -->
                  <div>创建：{{ formatDate(note.create_time) }}</div>
                  <div>更新：{{ formatDate(note.update_time) }}</div>
                </div>
              </div>
            </n-popover>
          </div>
        </div>
        <div v-html="note.content"></div>
      </NCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import {ref, reactive, onMounted, onUnmounted, watch, h} from "vue";
import type {Component} from 'vue'
import TiptapEditor from "@/components/TiptapEditor.vue";
import {invoke} from "@tauri-apps/api/core";
import {MoreHorizSharp} from "@vicons/material";
import {useNotification} from "naive-ui";

const content = ref("");
const notes = ref([]);
// 在setup函数外部初始化notification
const notification = useNotification();


// 发布处理函数
const handlePublish = async (content) => {
  try {
    await invoke("create_literature_note", {content});
    await getAllLiterature();
  } catch (error) {
    console.error("发布失败:", error);
  }
};

const getAllLiterature = async () => {
  try {
    notes.value = await invoke("get_all_literature_notes");
    console.log("所有文献笔记:", notes.value);
  } catch (error) {
    console.error("获取所有文献笔记失败:", error);
  }
};

// 编辑器样式响应式控制
const editorStyle = reactive({
  minHeight: "300px",
  transition: "min-height 0.3s ease",
});

const handleDelete = async (id) => {
  try {
    await invoke("delete_literature_note", {id});
    // 删除成功后，刷新列表并且提示删除成功
    notification.success({
      content: '已删除',
      duration: 2000,
    });
    await getAllLiterature();
  } catch (error) {
    console.error("删除文献笔记失败:", error);
  }
}

// 滚动监听处理函数
const handleScroll = () => {
  const scrollTop = window.scrollY;
  // 当滚动超过50px时，缩小编辑器高度
  if (scrollTop > 50) {
    editorStyle.minHeight = "120px";
  } else {
    editorStyle.minHeight = "300px";
  }
};

function renderIcon(icon: Component) {
  return () => h(NIcon, null, {default: () => h(icon)})
}

const formatDate = (dateString) => {
  // 加上时分秒
  const date = new Date(dateString);
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  const hour = String(date.getHours()).padStart(2, '0');
  const minute = String(date.getMinutes()).padStart(2, '0');
  const second = String(date.getSeconds()).padStart(2, '0');
  return `${year}-${month}-${day} ${hour}:${minute}:${second}`;
}

// 添加滚动监听
onMounted(() => {
  window.addEventListener("scroll", handleScroll);
  getAllLiterature();
});

// 移除滚动监听
onUnmounted(() => {
  window.removeEventListener("scroll", handleScroll);
});

watch(content, (newVal) => {
});

</script>

<style scoped>
.notes-feed {
  max-width: 750px;
  margin: 0 auto;
}

.notes-item {
  margin-top: 20px;
  background-color: #FAFAFA;
  border-radius: 15px;
}

.text-sm {
  font-size: 12px;
}

.text-gray-500 {
  color: #888;
}

.notes-head {
  display: flex;
  justify-content: space-between;
  height: 20px;
}

.text-center {
  line-height: 25px;
}

.popover-content {

  padding: 8px;
}

.action-buttons {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.action-button {
  width: 100%;
  text-align: left;
  justify-content: flex-start;
}

</style>