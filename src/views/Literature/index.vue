<template>
  <div class="literature-container">
    <!-- 固定顶部的文献笔记输入框 -->
    <div class="editor-container" :style="editorStyle">
      <h3>添加文献笔记</h3>
      <TiptapEditor v-model="noteContent" :publish="handlePublish"/>
    </div>

    <!-- 文献笔记卡片流 -->
    <div class="notes-feed">
      <!-- 卡片示例 - 实际应从数据渲染 -->
      <div class="note-card">笔记卡片 1</div>
      <div class="note-card">笔记卡片 2</div>
      <div class="note-card">笔记卡片 3</div>
      <div class="note-card">笔记卡片 4</div>
      <div class="note-card">笔记卡片 5</div>
      <div class="note-card">笔记卡片 6</div>
      <div class="note-card">笔记卡片 7</div>
      <div class="note-card">笔记卡片 8</div>
    </div>
  </div>
</template>

<script setup>
import {ref, reactive, onMounted, onUnmounted, watch} from "vue";
import TiptapEditor from "@/components/TiptapEditor.vue";
import {invoke} from "@tauri-apps/api/core";

// 笔记内容双向绑定
const noteContent = ref("");

// 发布处理函数
const handlePublish = async (content) => {
  try {
    await invoke("create_literature_note", {content});
    // 发布成功后，清空内容
    noteContent.value = "";
  } catch (error) {
    console.error("发布失败:", error);
  }
};

// 编辑器样式响应式控制
const editorStyle = reactive({
  minHeight: "300px",
  transition: "min-height 0.3s ease",
});

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

// 添加滚动监听
onMounted(() => {
  window.addEventListener("scroll", handleScroll);
});

// 移除滚动监听
onUnmounted(() => {
  window.removeEventListener("scroll", handleScroll);
});

watch(noteContent, (newVal) => {
});
</script>

<style scoped>
.notes-feed {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 20px;
  margin: 0 auto;
  max-width: 750px;
}
</style>