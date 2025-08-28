<template>
  <div class="tiptap-editor-container">
    <!-- 添加相对定位容器 -->
    <div class="editor-wrapper">
      <editor-content :editor="editor" class="editor-content"/>
      <!-- 发布按钮 -->
      <button
          class="publish-button"
          @click="handlePublish"
          :disabled="isDisabled"
      >
        <PaperPlane class="icon"/>
      </button>
    </div>
  </div>
</template>

<script setup>
import {useEditor, EditorContent} from "@tiptap/vue-3";
import StarterKit from "@tiptap/starter-kit";
import {Placeholder} from "@tiptap/extensions";
import {watch} from "vue";
import {Button} from "../tiptap-ui-primitive/button/index.js";
import {PaperPlane} from "@vicons/ionicons5";
import {ref} from "vue";
import Mention from '@tiptap/extension-mention'
import suggestions from "@/components/TiptapEditor/suggestions.js";

const props = defineProps({
  modelValue: {
    type: String,
    default: "",
  },
  publish: {
    type: Function,
    default: () => {
    },
  },
});

const isDisabled = ref(true);
const isContentEmpty = (html) => {
  // 移除所有HTML标签和空白字符后检查是否为空
  const text = html.replace(/<[^>]*>|\s+/g, "");
  return text.length === 0;
};

const editor = useEditor({
  content: props.modelValue,
  extensions: [
    StarterKit,
    Placeholder.configure({
      placeholder: ({node}) => {
        // 如果content为空，显示提示文本
        if (node.content.size === 0) {
          return "开始输入文献笔记... 📝";
        }
        console.log(node);
      },
    }),
    Mention.configure({
      renderHTML({node}) {
        return ['a', {
          class: 'tag',
          'data-is-new': node.attrs.id,
          href: `#${node.attrs.label}`,
          target: '_blank',
          style: 'text-decoration: none;',
        },
        `#${node.attrs.label}`
        ]
      },
      HTMLAttributes: {
        class: 'tag',
      },
      suggestions,
    })
  ],
  // 添加基础配置确保编辑器可用
  editorProps: {
    attributes: {
      class: "tiptap-editor",
    },
  },
  onUpdate: ({editor}) => {
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
        // 检查内容是否为空
        isDisabled.value = isContentEmpty(editor.value.getHTML());
        return;
      }
      isDisabled.value = isContentEmpty(editor.value.getHTML());
    }
);

// 添加发布处理函数
const handlePublish = () => {
  if (!editor || !editor.value) return;
  // 获取编辑器内容
  const content = editor.value.getHTML();
  // 如果有传递的发布处理函数，则调用它
  if (props.publish && typeof props.publish === 'function') {
    props.publish(content);
  } else {
    // 如果没有传递处理函数，触发默认的发布事件
    emit("publish", content);
  }

  // 可选：发布后清空内容
  editor.value.commands.clearContent();
  console.log("发布内容:", content);
};

const emit = defineEmits(["update:modelValue", "publish"]);
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

:deep(.tiptap-editor) {
  border: none;
  outline: none;
  min-height: 200px;
  padding: 0px 20px;
}

.tiptap-editor-container:focus {
  border: none;
}

:deep(.tiptap-editor:focus-within) {
  outline: none !important;
  border: none !important;
}

.editor-wrapper {
  position: relative; /* 为按钮绝对定位提供参考 */
  min-height: 200px; /* 保持最小高度 */
}

.publish-button {
  position: absolute; /* 绝对定位在编辑器内 */
  right: 12px; /* 右间距 */
  bottom: 12px; /* 底部间距 */
  width: 60px; /* 按钮大小 */
  height: 35px; /* 按钮大小 */
  line-height: 30px; /* 居中 */
  border-radius: 10px; /* 圆形按钮 */
  background-color: rgb(57, 115, 72); /* 主题色 */
  color: white; /* 图标颜色 */
  border: none; /* 无边框 */
  cursor: pointer; /* 鼠标指针 */
  display: flex; /* 图标居中 */
  align-items: center; /* 图标居中 */
  justify-content: center; /* 图标居中 */
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15); /* 阴影效果 */
  transition: background-color 0.2s; /* 过渡效果 */
}

.publish-button:hover {
  background-color: rgb(45, 92, 57); /*  hover状态 */
}

.publish-button:disabled {
  background-color: #ccc; /* 禁用状态 */
  cursor: not-allowed; /* 禁用指针 */
}

.icon {
  width: 20px; /* 图标大小 */
  height: 20px; /* 图标大小 */
}

/* Basic editor styles */
:deep(.tiptap-editor-container) .tiptap {
  .tag {
    background-color: var(--purple-light);
    border-radius: 0.4rem;
    box-decoration-break: clone;
    color: var(--purple);
    padding: 0.1rem 0.3rem;
  }
}

/* Character count */
.character-count {
  align-items: center;
  color: var(--gray-5);
  display: flex;
  font-size: 0.75rem;
  gap: 0.5rem;
  margin: 1.5rem;

  svg {
    color: var(--purple);
  }

  &--warning,
  &--warning svg {
    color: var(--red);
  }
}
</style>