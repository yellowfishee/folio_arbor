import {computePosition, flip, shift} from '@floating-ui/dom'
import {posToDOMRect, VueRenderer} from '@tiptap/vue-3'
import {invoke} from "@tauri-apps/api/core";
import MentionList from './MentionList.vue'

const updatePosition = (editor, element) => {
    const virtualElement = {
        getBoundingClientRect: () => posToDOMRect(editor.view, editor.state.selection.from, editor.state.selection.to),
    }
    computePosition(virtualElement, element, {
        placement: 'bottom-start',
        strategy: 'absolute',
        middleware: [shift(), flip()],
    }).then(({x, y, strategy}) => {
        element.style.width = 'max-content'
        element.style.position = strategy
        element.style.left = `${x}px`
        element.style.top = `${y}px`
    })
}

export default [
    {
        char: '@',
        items: ({query}) => {
            return [
                'Lea Thompson',
                'Cyndi Lauper',
                'Tom Cruise',
                'Madonna',
                'Jerry Hall',
                'Joan Collins',
                'Winona Ryder',
                'Christina Applegate',
                'Alyssa Milano',
                'Molly Ringwald',
                'Ally Sheedy',
                'Debbie Harry',
                'Olivia Newton-John',
                'Elton John',
                'Michael J. Fox',
                'Axl Rose',
                'Emilio Estevez',
                'Ralph Macchio',
                'Rob Lowe',
                'Jennifer Grey',
                'Mickey Rourke',
                'John Cusack',
                'Matthew Broderick',
                'Justine Bateman',
                'Lisa Bonet',
            ]
                .filter(item => item.toLowerCase().startsWith(query.toLowerCase()))
                .slice(0, 5)
        },

        render: () => {
            let component

            return {
                onStart: props => {
                    component = new VueRenderer(MentionList, {
                        // using vue 2:
                        // parent: this,
                        // propsData: props,
                        // using vue 3:
                        props,
                        editor: props.editor,
                    })

                    if (!props.clientRect) {
                        return
                    }

                    component.element.style.position = 'absolute'

                    document.body.appendChild(component.element)

                    updatePosition(props.editor, component.element)
                },

                onUpdate(props) {
                    component.updateProps(props)

                    if (!props.clientRect) {
                        return
                    }

                    updatePosition(props.editor, component.element)
                },

                onKeyDown(props) {
                    if (props.event.key === 'Escape') {
                        component.destroy()

                        return true
                    }

                    return component.ref?.onKeyDown(props)
                },

                onExit() {
                    component.destroy()
                },
            }
        },
    },
    {
        char: '#',
        items: async ({query}) => {
            try {
                const tags = await invoke('get_tags', {query});
                // 添加搜索过滤逻辑
                const filteredTags = tags
                    .filter(tag =>
                        tag.full_name.toLowerCase().includes(query.toLowerCase())
                    )
                    .slice(0, 10); // 限制显示数量

                return filteredTags.length > 0
                    ? filteredTags.map(tag => ({
                        id: tag.id,
                        label: tag.full_name,
                        isNew: false
                    }))
                    : [{
                        id: `new`,
                        label: query,
                        isNew: true
                    }];
            } catch (error) {
                console.error('标签获取失败:', error);
                return [{
                    id: `new_${query}`,
                    label: query,
                    isNew: true
                }];
            }
        },

        render: () => {
            let component

            return {
                onStart: props => {
                    component = new VueRenderer(MentionList, {
                        props: {
                            // 修正 props 结构传递
                            items: props.items,  // 直接传递 items 数组
                            command: props.command,  // 保留 command 回调
                            editor: props.editor
                        },
                        editor: props.editor,
                    })

                    if (!props.clientRect) {
                        return
                    }

                    component.element.style.position = 'absolute'

                    document.body.appendChild(component.element)

                    updatePosition(props.editor, component.element)
                },

                onUpdate(props) {
                    component.updateProps(props)

                    if (!props.clientRect) {
                        return
                    }

                    updatePosition(props.editor, component.element)
                },

                onKeyDown(props) {
                    if (props.event.key === 'Escape') {
                        component.destroy()
                        return true
                    }

                    return component.ref?.onKeyDown(props)
                },

                onExit() {
                    component.destroy()
                },
            }
        },
    },
]