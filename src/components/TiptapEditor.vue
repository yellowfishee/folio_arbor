<template>
  <div class="tiptap-editor-container">
    <editor-content :editor="editor" class="editor-content" />
  </div>
</template>
  
  <script setup>
import { useEditor, EditorContent } from "@tiptap/vue-3";
import StarterKit from "@tiptap/starter-kit";
import { Placeholder } from "@tiptap/extensions";
import { watch } from "vue";

const props = defineProps({
  modelValue: {
    type: String,
    default: "",
  },
});

const editor = useEditor({
  content: props.modelValue,

  extensions: [
    StarterKit,
    Placeholder.configure({
      placeholder: ({ node }) => {
        // 如果content为空，显示提示文本
        if (node.content.size === 0) {
          return "开始输入文献笔记... 📝";
        }
        console.log(node);
      },
    }),
  ],
  // 添加基础配置确保编辑器可用
  editorProps: {
    attributes: {
      class: "tiptap-editor",
    },
  },
  onUpdate: ({ editor }) => {
    // 当编辑器内容更新时，触发v-model的更新
    const html = editor.getHTML();
    // 通过$emit触发更新事件
    emit("update:modelValue", html);
  },
});

watch(
  () => props.modelValue,
  (newValue) => {
    if (editor && newValue !== editor.value.getHTML()) {
      editor.value.commands.setContent(newValue);
    }
  }
);

const emit = defineEmits(["update:modelValue"]);
</script>

<style scoped>
.tiptap-editor-container {
  border: 1px solid rgb(72, 72, 72);
  border-radius: 8px;
  max-width: 750px;
  margin: 0 auto;
}

.tiptap-editor-container:hover {
  border: 1px solid rgb(57, 115, 72);
  border-radius: 8px;
}

::v-deep .tiptap-editor {
  border: none;
  outline: none;
  min-height: 200px;
  padding: 0px 20px;
}

.tiptap-editor-container:focus {
  border: none;
}

::v-deep .tiptap-editor:focus-within {
  outline: none !important;
  border: none !important;
}
</style>