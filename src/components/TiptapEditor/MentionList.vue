<template>
  <div class="dropdown-menu">
    <template v-if="items.length">
      <button
          :class="{ 'is-selected': index === selectedIndex }"
          v-for="(item, index) in items"
          :key="index"
          @click="selectItem(index)"
          style="font-size: 15px;"
      >
        {{ item }}
      </button>
    </template>
    <div class="item" v-else>No result</div>
  </div>
</template>

<script>
export default {
  props: {
    items: {
      type: Array,
      required: true,
    },

    command: {
      type: Function,
      required: true,
    },
  },

  data() {
    return {
      selectedIndex: 0,
    }
  },

  watch: {
    items() {
      this.selectedIndex = 0
    },
  },

  methods: {
    onKeyDown({event}) {
      if (event.key === 'ArrowUp') {
        this.upHandler()
        return true
      }

      if (event.key === 'ArrowDown') {
        this.downHandler()
        return true
      }

      if (event.key === 'Enter') {
        this.enterHandler()
        return true
      }

      return false
    },

    upHandler() {
      this.selectedIndex = (this.selectedIndex + this.items.length - 1) % this.items.length
    },

    downHandler() {
      this.selectedIndex = (this.selectedIndex + 1) % this.items.length
    },

    enterHandler() {
      this.selectItem(this.selectedIndex)
    },

    selectItem(index) {
      const item = this.items[index]

      if (item) {
        this.command({id: item})
      }
    },
  },
}
</script>

<style lang="scss" scoped>
:root {
  --white: #ffffff;
  --gray-1: #e5e7eb;
  --gray-2: #f3f4f6;
  --gray-3: #f9fafb;
  --shadow: 0px 12px 33px 0px rgba(0, 0, 0, 0.06), 0px 3.618px 9.949px 0px rgba(0, 0, 0, 0.04);
}

/* Dropdown menu */
.dropdown-menu {
  background: var(--white);
  border: 1px solid var(--gray-1);
  border-radius: 0.7rem;
  box-shadow: var(--shadow);
  display: flex;
  flex-direction: column;
  overflow: auto;
  padding: 0.4rem;
  position: relative;

  button {
    align-items: center;
    background-color: transparent;
    display: flex;
    text-align: left;
    width: 100%;
    border: none;
    font-size: 14px;
    height: 35px;

    &:hover,
    &:hover.is-selected {
      background-color: var(--gray-3);
    }

    &.is-selected {
      background-color: var(--gray-2);
    }
  }
}
</style>