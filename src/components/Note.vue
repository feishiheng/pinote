<template>
  <div class="note" :style="noteStyle">
    <div class="note-header">
      <div class="note-title" @mousedown="startDrag">便签</div>
      <div class="note-controls">
        <button @click.stop="createNewNote" title="新建便签">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
            <path d="M19 13H13V19H11V13H5V11H11V5H13V11H19V13Z" fill="currentColor"/>
          </svg>
        </button>
        <button @click.stop="toggleAlwaysOnTop" :class="{ active: isAlwaysOnTop }" title="置顶">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
            <path d="M16 12V4H17V2H7V4H8V12L6 14V16H11.2V22H12.8V16H18V14L16 12Z" fill="currentColor"/>
          </svg>
        </button>
        <button @click.stop="toggleSettings" :class="{ active: showSettings }" title="设置">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
            <path d="M19.14 12.94C19.18 12.64 19.2 12.33 19.2 12C19.2 11.68 19.18 11.36 19.13 11.06L21.16 9.48C21.34 9.34 21.39 9.07 21.28 8.87L19.36 5.55C19.24 5.33 18.99 5.26 18.77 5.33L16.38 6.29C15.88 5.91 15.35 5.59 14.76 5.35L14.4 2.81C14.36 2.57 14.16 2.4 13.92 2.4H10.08C9.84 2.4 9.65 2.57 9.61 2.81L9.25 5.35C8.66 5.59 8.12 5.92 7.63 6.29L5.24 5.33C5.02 5.25 4.77 5.33 4.65 5.55L2.74 8.87C2.53 9.08 2.57 9.34 2.75 9.48L4.78 11.06C4.74 11.36 4.72 11.69 4.72 12C4.72 12.31 4.74 12.64 4.79 12.94L2.76 14.52C2.58 14.66 2.53 14.93 2.64 15.13L4.56 18.45C4.68 18.67 4.93 18.74 5.15 18.67L7.54 17.71C8.04 18.09 8.57 18.41 9.16 18.65L9.52 21.19C9.56 21.43 9.76 21.6 10 21.6H14C14.24 21.6 14.44 21.43 14.48 21.19L14.84 18.65C15.43 18.41 15.97 18.09 16.46 17.71L18.85 18.67C19.07 18.75 19.32 18.67 19.44 18.45L21.36 15.13C21.47 14.91 21.43 14.66 21.25 14.52L19.14 12.94ZM12 15.6C10.02 15.6 8.4 13.98 8.4 12C8.4 10.02 10.02 8.4 12 8.4C13.98 8.4 15.6 10.02 15.6 12C15.6 13.98 13.98 15.6 12 15.6Z" fill="currentColor"/>
          </svg>
        </button>
        <button @click.stop="closeNote" title="关闭">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
            <path d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12L19 6.41Z" fill="currentColor"/>
          </svg>
        </button>
      </div>
    </div>
    
    <!-- 设置面板 -->
    <div v-if="showSettings" class="settings-panel" @click.self="toggleSettings">
      <div class="settings-content">
        <div class="settings-list">
          <div class="setting-group">
            <label>颜色主题:</label>
            <div class="color-options">
              <button 
                v-for="color in colorThemes" 
                :key="color.name"
                @click="changeTheme(color)"
                :class="{ active: currentTheme.name === color.name }"
                :style="{ backgroundColor: color.background }"
                :title="color.name"
                class="color-btn"
              >
              </button>
            </div>
          </div>
          
          <div class="setting-group">
            <label>字体大小:</label>
            <div class="font-size-controls">
              <button @click="setFontSize(12)" :class="{ active: fontSize === 12 }">小</button>
              <button @click="setFontSize(14)" :class="{ active: fontSize === 14 }">中</button>
              <button @click="setFontSize(16)" :class="{ active: fontSize === 16 }">大</button>
              <button @click="setFontSize(18)" :class="{ active: fontSize === 18 }">特大</button>
            </div>
            <div class="font-size-slider">
              <button @click="changeFontSize(-1)" title="减小字体">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none">
                  <path d="M19 13H5V11H19V13Z" fill="currentColor"/>
                </svg>
              </button>
              <span>{{ fontSize }}px</span>
              <button @click="changeFontSize(1)" title="增大字体">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none">
                  <path d="M19 13H13V19H11V13H5V11H11V5H13V11H19V13Z" fill="currentColor"/>
                </svg>
              </button>
            </div>
          </div>
          
          <div class="setting-group">
            <label>字体样式:</label>
            <div class="font-family-controls">
              <button 
                v-for="font in fontFamilies" 
                :key="font.name"
                @click="changeFontFamily(font)"
                :class="{ active: fontFamily === font.value }"
                :title="font.name"
                class="font-family-btn"
              >
                {{ font.name }}
              </button>
            </div>
          </div>
          
          <div class="setting-group">
            <label>窗口大小:</label>
            <div class="size-controls">
              <button @click="shrinkWindow()" title="缩小窗口">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none">
                  <path d="M19 13H13V19H11V13H5V11H11V5H13V11H19V13Z" fill="currentColor"/>
                </svg>
              </button>
              <button @click="expandWindow()" title="放大窗口">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none">
                  <path d="M7 14H5V19H10V17H7V14ZM5 10H7V7H10V5H5V10ZM17 17H14V19H19V14H17V17ZM14 5V7H17V10H19V5H14Z" fill="currentColor"/>
                </svg>
              </button>
              <button @click="resetWindowSize()" title="重置大小">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none">
                  <path d="M12 5V1L7 6L12 11V7C15.31 7 18 9.69 18 13C18 16.31 15.31 19 12 19C8.69 19 6 16.31 6 13H4C4 17.42 7.58 21 12 21C16.42 21 20 17.42 20 13C20 8.58 16.42 5 12 5Z" fill="currentColor"/>
                </svg>
              </button>
            </div>
          </div>
          
          <div class="setting-group">
            <label>继承设置:</label>
            <div class="inherit-controls">
              <label class="switch">
                <input type="checkbox" v-model="inheritSettings">
                <span class="slider"></span>
              </label>
              <span class="inherit-label">子便签继承当前设置</span>
            </div>
          </div>
        </div>
      </div>
      <div class="settings-footer">
        <button class="collapse-btn" @click="toggleSettings" title="收起设置">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
            <path d="M7.41 15.41L12 10.83L16.59 15.41L18 14L12 8L6 14L7.41 15.41Z" fill="currentColor"/>
          </svg>
          收起设置
        </button>
      </div>
    </div>
    
    <div class="note-content" :class="{ 'settings-open': showSettings }">
      <textarea 
        v-model="content" 
        @input="onContentChange"
        :style="{ fontSize: fontSize + 'px', fontFamily: fontFamily }"
        placeholder="在这里输入内容..."
      ></textarea>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { PhysicalSize } from '@tauri-apps/api/window'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

const content = ref('')
const isAlwaysOnTop = ref(true) // 默认置顶
const showSettings = ref(false)
const fontSize = ref(14)
const fontFamily = ref('Microsoft YaHei')
const currentTheme = ref({ name: '默认', background: 'rgba(255, 255, 200, 0.95)', header: 'rgba(255, 255, 150, 0.95)' })
const inheritSettings = ref(false)

// 定义初始窗口大小常量
const INITIAL_WINDOW_WIDTH = 300
const INITIAL_WINDOW_HEIGHT = 400

// 存储实际的初始窗口大小（考虑边框等因素）
let actualInitialSize: { width: number; height: number } | null = null

const colorThemes = ref([
  { name: '默认', background: 'rgba(255, 255, 200, 0.95)', header: 'rgba(255, 255, 150, 0.95)' },
  { name: '粉色', background: 'rgba(255, 182, 193, 0.95)', header: 'rgba(255, 160, 180, 0.95)' },
  { name: '蓝色', background: 'rgba(173, 216, 230, 0.95)', header: 'rgba(135, 206, 235, 0.95)' },
  { name: '绿色', background: 'rgba(144, 238, 144, 0.95)', header: 'rgba(124, 252, 124, 0.95)' },
  { name: '紫色', background: 'rgba(221, 160, 221, 0.95)', header: 'rgba(218, 112, 214, 0.95)' }
])

const fontFamilies = ref([
  { name: '微软雅黑', value: 'Microsoft YaHei' },
  { name: '苹方', value: 'PingFang SC' },
  { name: '宋体', value: 'SimSun' },
  { name: '楷体', value: 'KaiTi' },
  { name: '等宽字体', value: 'Consolas, Monaco, monospace' }
])

const noteStyle = computed(() => ({
  '--note-bg': currentTheme.value.background,
  '--header-bg': currentTheme.value.header
}))

const startDrag = async (e: MouseEvent) => {
  try {
    // 阻止事件传播，确保只在标题区域触发拖拽
    e.preventDefault()
    e.stopPropagation()
    
    const currentWindow = WebviewWindow.getCurrent()
    // 使用 Tauri 的 startDragging API 来拖拽整个窗口
    await currentWindow.startDragging()
  } catch (error) {
    console.error('拖拽初始化失败:', error)
  }
}

const createNewNote = async () => {
  try {
    console.log('点击了创建新便签按钮')
    const label = 'note-' + Date.now()
    console.log('窗口标签:', label)
    
    // 准备传递给新便签的设置
    const settings = inheritSettings.value ? {
      theme: currentTheme.value,
      fontSize: fontSize.value,
      fontFamily: fontFamily.value
    } : null
    
    const newWindow = new WebviewWindow(label, {
      title: '新便签',
      width: INITIAL_WINDOW_WIDTH,
      height: INITIAL_WINDOW_HEIGHT,
      x: Math.random() * 200 + 100,
      y: Math.random() * 200 + 100,
      alwaysOnTop: true,
      decorations: false,
      transparent: false,
      resizable: true,
      visible: true,
    })
    
    // 如果启用了继承设置，等待窗口创建完成后传递设置
    if (settings) {
      newWindow.once('tauri://created', async () => {
        try {
          // 使用 Tauri 的事件系统发送设置
          await newWindow.emit('apply-settings', settings)
          console.log('设置已发送到新便签')
        } catch (error) {
          console.error('发送设置失败:', error)
        }
      })
    }
    
    console.log('新窗口创建成功:', label)
  } catch (error) {
    console.error('创建新窗口失败:', error)
    const errorMessage = error instanceof Error ? error.message : String(error)
    alert('创建新窗口失败: ' + errorMessage)
  }
}

const toggleAlwaysOnTop = async () => {
  try {
    console.log('点击了置顶按钮')
    const currentWindow = WebviewWindow.getCurrent()
    isAlwaysOnTop.value = !isAlwaysOnTop.value
    await currentWindow.setAlwaysOnTop(isAlwaysOnTop.value)
    console.log('窗口置顶状态:', isAlwaysOnTop.value)
  } catch (error) {
    console.error('切换置顶状态失败:', error)
    // 如果失败，恢复状态
    isAlwaysOnTop.value = !isAlwaysOnTop.value
  }
}

const closeNote = async () => {
  try {
    console.log('点击了关闭按钮')
    const currentWindow = WebviewWindow.getCurrent()
    console.log('正在关闭窗口:', currentWindow.label)
    await currentWindow.close()
  } catch (error) {
    console.error('关闭窗口失败:', error)
  }
}

const changeTheme = (color: any) => {
  currentTheme.value = color
}

const changeFontSize = (delta: number) => {
  const newSize = fontSize.value + delta
  if (newSize >= 10 && newSize <= 24) {
    fontSize.value = newSize
  }
}

const setFontSize = (size: number) => {
  fontSize.value = size
}

const changeFontFamily = (font: any) => {
  fontFamily.value = font.value
}

const shrinkWindow = async () => {
  try {
    console.log('点击了缩小按钮')
    const currentWindow = WebviewWindow.getCurrent()
    
    // 使用物理尺寸避免 DPI 缩放问题
    const currentSize = await currentWindow.outerSize()
    
    console.log(`当前窗口大小: ${currentSize.width}x${currentSize.height}`)
    
    // 确保窗口不会变得太小
    const minWidth = 200
    const minHeight = 150
    
    const calculatedWidth = currentSize.width - 50
    const calculatedHeight = currentSize.height - 50
    
    console.log(`计算后的大小: ${calculatedWidth}x${calculatedHeight}`)
    
    const newWidth = Math.max(minWidth, calculatedWidth)
    const newHeight = Math.max(minHeight, calculatedHeight)
    
    console.log(`最终设置的大小: ${newWidth}x${newHeight}`)
    
    await currentWindow.setSize(new PhysicalSize(newWidth, newHeight))
    
    // 验证设置后的实际大小
    const verifySize = await currentWindow.outerSize()
    console.log(`设置后的实际大小: ${verifySize.width}x${verifySize.height}`)
    
  } catch (error) {
    console.error('调整窗口大小失败:', error)
  }
}

const expandWindow = async () => {
  try {
    console.log('点击了放大按钮')
    const currentWindow = WebviewWindow.getCurrent()
    
    // 使用物理尺寸避免 DPI 缩放问题
    const currentSize = await currentWindow.outerSize()
    
    console.log(`当前窗口大小: ${currentSize.width}x${currentSize.height}`)
    
    // 确保窗口不会变得太大
    const maxWidth = 800
    const maxHeight = 600
    
    const calculatedWidth = currentSize.width + 50
    const calculatedHeight = currentSize.height + 50
    
    console.log(`计算后的大小: ${calculatedWidth}x${calculatedHeight}`)
    
    const newWidth = Math.min(maxWidth, calculatedWidth)
    const newHeight = Math.min(maxHeight, calculatedHeight)
    
    console.log(`最终设置的大小: ${newWidth}x${newHeight}`)
    
    await currentWindow.setSize(new PhysicalSize(newWidth, newHeight))
    
    // 验证设置后的实际大小
    const verifySize = await currentWindow.outerSize()
    console.log(`设置后的实际大小: ${verifySize.width}x${verifySize.height}`)
    
  } catch (error) {
    console.error('调整窗口大小失败:', error)
  }
}

const resetWindowSize = async () => {
  try {
    console.log('点击了重置按钮')
    const currentWindow = WebviewWindow.getCurrent()
    
    // 获取当前大小
    const currentSize = await currentWindow.outerSize()
    console.log(`重置前窗口大小: ${currentSize.width}x${currentSize.height}`)
    
    if (actualInitialSize) {
      // 使用记录的实际初始大小
      console.log(`重置为记录的初始大小: ${actualInitialSize.width}x${actualInitialSize.height}`)
      await currentWindow.setSize(new PhysicalSize(actualInitialSize.width, actualInitialSize.height))
    } else {
      // 如果没有记录，使用默认值
      console.log(`重置为默认大小: ${INITIAL_WINDOW_WIDTH}x${INITIAL_WINDOW_HEIGHT}`)
      await currentWindow.setSize(new PhysicalSize(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT))
    }
    
    // 验证重置后的大小
    const verifySize = await currentWindow.outerSize()
    console.log(`重置后的实际大小: ${verifySize.width}x${verifySize.height}`)
    
  } catch (error) {
    console.error('重置窗口大小失败:', error)
  }
}

const onContentChange = async () => {
  // 自动保存功能 - 延迟保存避免频繁写入
  if (saveTimeout) {
    clearTimeout(saveTimeout)
  }
  saveTimeout = setTimeout(async () => {
    try {
      const currentWindow = WebviewWindow.getCurrent()
      const fileName = `note_${currentWindow.label}.txt`
      await invoke('save_note_content', { fileName, content: content.value })
      console.log('便签内容已自动保存')
    } catch (error) {
      console.error('自动保存失败:', error)
    }
  }, 1000) // 1秒后保存
}

let saveTimeout: number | null = null

const toggleSettings = () => {
  showSettings.value = !showSettings.value
}

onMounted(async () => {
  try {
    const currentWindow = WebviewWindow.getCurrent()
    
    // 记录实际的初始窗口大小
    const initialSize = await currentWindow.outerSize()
    actualInitialSize = { width: initialSize.width, height: initialSize.height }
    console.log(`记录实际初始窗口大小: ${actualInitialSize.width}x${actualInitialSize.height}`)
    
    // 自动设置为置顶
    await currentWindow.setAlwaysOnTop(true)
    isAlwaysOnTop.value = true
    
    // 加载保存的内容
    const fileName = `note_${currentWindow.label}.txt`
    const savedContent = await invoke('load_note_content', { fileName }) as string
    content.value = savedContent
    console.log('便签内容已加载')

    // 只监听新窗口创建时的设置应用事件
    const unlisten = await listen('apply-settings', (event) => {
      const settings = event.payload as any
      if (settings) {
        console.log('收到设置:', settings)
        // 应用主题设置
        if (settings.theme) {
          currentTheme.value = settings.theme
        }
        // 应用字体大小设置
        if (settings.fontSize) {
          fontSize.value = settings.fontSize
        }
        // 应用字体样式设置
        if (settings.fontFamily) {
          fontFamily.value = settings.fontFamily
        }
      }
    })

    // 在组件卸载时取消监听
    onUnmounted(() => {
      unlisten()
    })
  } catch (error) {
    console.error('初始化便签失败:', error)
  }
})

onUnmounted(() => {
  // 移除不需要的事件监听器
})
</script>

<style scoped>
.note {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background: var(--note-bg, rgba(255, 255, 200, 0.95));
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
  border: 1px solid rgba(0, 0, 0, 0.1);
  position: relative;
}

.note-header {
  background: var(--header-bg, rgba(255, 255, 150, 0.95));
  padding: 8px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  cursor: move;
  user-select: none;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  position: relative;
  z-index: 1000;
}

.note-title {
  font-weight: bold;
  color: #333;
  cursor: move;
  user-select: none;
  flex: 1;
  padding: 4px 0;
}

.note-controls {
  display: flex;
  gap: 8px;
  position: relative;
  z-index: 1001;
}

.note-controls button {
  background: rgba(255, 255, 255, 0.7);
  border: 1px solid rgba(0, 0, 0, 0.1);
  cursor: pointer;
  font-size: 14px;
  padding: 4px 6px;
  border-radius: 6px;
  color: #333;
  transition: all 0.2s;
  position: relative;
  z-index: 1001;
  pointer-events: auto;
  min-width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.note-controls button:hover {
  background: rgba(255, 255, 255, 0.9);
  border-color: rgba(0, 0, 0, 0.2);
  transform: scale(1.05);
}

.note-controls button.active {
  background: rgba(255, 215, 0, 0.9);
  border-color: rgba(255, 165, 0, 0.6);
  color: #000;
  box-shadow: 0 2px 4px rgba(255, 215, 0, 0.3);
}

.note-content {
  flex: 1;
  padding: 8px;
  background: rgba(255, 255, 255, 0.5);
  transition: all 0.3s ease;
  overflow-y: overlay;
  min-height: 200px;
  position: relative;
}

.note-content.settings-open {
  margin-top: 0;
  padding-top: 8px;
}

textarea {
  width: 100%;
  height: 100%;
  border: none;
  resize: none;
  background: transparent;
  font-size: 14px;
  font-family: 'Microsoft YaHei', 'PingFang SC', 'Helvetica Neue', Arial, sans-serif;
  line-height: 1.6;
  letter-spacing: 0.5px;
  outline: none;
  color: #333;
  padding: 8px;
  box-sizing: border-box;
  font-weight: 400;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-rendering: optimizeLegibility;
  overflow-y: overlay;
}

textarea::placeholder {
  color: rgba(0, 0, 0, 0.4);
  font-style: italic;
  font-weight: 300;
}

/* 设置面板样式 */
.settings-panel {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.85);
  backdrop-filter: blur(12px) saturate(180%);
  -webkit-backdrop-filter: blur(12px) saturate(180%);
  z-index: 998;
  display: flex;
  flex-direction: column;
  padding-top: 40px; /* 为标题栏留出空间 */
  cursor: default; /* 添加默认光标样式 */
}

.settings-content {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
  margin: 0 auto;
  width: 100%;
  max-width: 600px;
}

.settings-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  width: 100%;
}

.setting-group {
  margin-bottom: 0;
  padding-bottom: 12px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.2);
  break-inside: avoid;
}

.setting-group:last-child {
  margin-bottom: 0;
  padding-bottom: 0;
  border-bottom: none;
}

.setting-group label {
  display: block;
  margin-bottom: 6px;
  font-weight: 500;
  font-size: 12px;
  color: rgba(0, 0, 0, 0.8);
}

.color-options {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
  justify-content: flex-start;
}

.color-btn {
  width: 20px;
  height: 20px;
  border: 1px solid rgba(255, 255, 255, 0.3);
  border-radius: 50%;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 0;
  position: relative;
  backdrop-filter: blur(4px);
  -webkit-backdrop-filter: blur(4px);
}

.color-btn:hover {
  transform: scale(1.1);
  border-color: rgba(255, 255, 255, 0.5);
}

.color-btn.active {
  border-color: rgba(255, 255, 255, 0.8);
  border-width: 2px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.color-btn.active::after {
  content: '✓';
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  color: rgba(0, 0, 0, 0.7);
  font-size: 12px;
  font-weight: bold;
}

.font-size-controls {
  display: flex;
  align-items: center;
  gap: 4px;
  margin-bottom: 6px;
  flex-wrap: wrap;
}

.font-size-controls button {
  flex: 1;
  min-width: 36px;
  max-width: 50px;
  font-size: 11px;
  padding: 4px 6px;
  height: 24px;
}

.font-size-slider {
  display: flex;
  align-items: center;
  gap: 6px;
}

.font-size-slider button {
  width: 24px;
  height: 24px;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}

.font-size-slider span {
  font-size: 11px;
  color: #666;
  min-width: 36px;
  text-align: center;
}

.font-family-controls {
  display: flex;
  gap: 4px;
  flex-wrap: nowrap;
  overflow-x: auto;
  padding-bottom: 4px;
  margin: 0 -12px;
  padding: 0 12px;
  scrollbar-width: thin;
  -webkit-overflow-scrolling: touch;
}

.font-family-btn {
  flex: 0 0 auto;
  min-width: 50px;
  max-width: 70px;
  font-size: 11px;
  padding: 4px 6px;
  height: 24px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.size-controls {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
}

.size-controls button {
  flex: 1;
  min-width: 36px;
  max-width: 50px;
  height: 24px;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* 按钮通用样式 */
.font-size-controls button,
.font-family-btn,
.size-controls button {
  background: rgba(255, 255, 255, 0.6);
  border: 1px solid rgba(255, 255, 255, 0.3);
  border-radius: 4px;
  cursor: pointer;
  color: rgba(0, 0, 0, 0.8);
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  backdrop-filter: blur(4px);
  -webkit-backdrop-filter: blur(4px);
}

.font-size-controls button:hover,
.font-family-btn:hover,
.size-controls button:hover {
  background: rgba(255, 255, 255, 0.8);
  border-color: rgba(255, 255, 255, 0.5);
  transform: translateY(-1px);
}

.font-size-controls button.active,
.font-family-btn.active {
  background: rgba(255, 215, 0, 0.6);
  border-color: rgba(255, 165, 0, 0.4);
  color: rgba(0, 0, 0, 0.9);
  font-weight: bold;
  box-shadow: 0 2px 4px rgba(255, 215, 0, 0.2);
}

/* 优化滚动条样式 */
.note-content::-webkit-scrollbar,
textarea::-webkit-scrollbar {
  width: 6px;
  background: transparent;
}

.note-content::-webkit-scrollbar-track,
textarea::-webkit-scrollbar-track {
  background: transparent;
}

.note-content::-webkit-scrollbar-thumb,
textarea::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 3px;
  border: 1px solid rgba(255, 255, 255, 0.3);
}

.note-content::-webkit-scrollbar-thumb:hover,
textarea::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.3);
}

/* 设置面板滚动条样式 */
.settings-content::-webkit-scrollbar {
  width: 4px;
}

.settings-content::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 2px;
}

.settings-content::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 2px;
  border: 1px solid rgba(255, 255, 255, 0.3);
}

.settings-content::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.3);
}

/* SVG 图标样式优化 */
svg {
  transition: all 0.2s ease;
}

button:hover svg {
  transform: scale(1.1);
}

button:active svg {
  transform: scale(0.95);
}

/* 确保 SVG 图标颜色继承按钮颜色 */
.note-controls button svg,
.font-size-controls button svg,
.font-size-slider button svg,
.size-controls button svg {
  color: inherit;
}

/* 优化横向滚动条样式 */
.font-family-controls::-webkit-scrollbar {
  height: 4px;
}

.font-family-controls::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 2px;
}

.font-family-controls::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 2px;
  border: 1px solid rgba(255, 255, 255, 0.3);
}

.font-family-controls::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.3);
}

.settings-footer {
  padding: 12px;
  display: flex;
  justify-content: center;
  border-top: 1px solid rgba(255, 255, 255, 0.2);
  background: rgba(255, 255, 255, 0.5);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
}

.collapse-btn {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 8px 16px;
  border-radius: 20px;
  background: rgba(255, 255, 255, 0.8);
  border: 1px solid rgba(255, 255, 255, 0.3);
  color: rgba(0, 0, 0, 0.8);
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s ease;
  backdrop-filter: blur(4px);
  -webkit-backdrop-filter: blur(4px);
}

.collapse-btn:hover {
  background: rgba(255, 255, 255, 0.9);
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.collapse-btn:active {
  transform: translateY(0);
}

.collapse-btn svg {
  transition: transform 0.2s ease;
}

.collapse-btn:hover svg {
  transform: translateY(2px);
}

.inherit-controls {
  display: flex;
  align-items: center;
  gap: 8px;
}

.inherit-label {
  font-size: 12px;
  color: rgba(0, 0, 0, 0.8);
}

/* 开关样式 */
.switch {
  position: relative;
  display: inline-block;
  width: 36px;
  height: 20px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.1);
  transition: .4s;
  border-radius: 20px;
}

.slider:before {
  position: absolute;
  content: "";
  height: 16px;
  width: 16px;
  left: 2px;
  bottom: 2px;
  background-color: white;
  transition: .4s;
  border-radius: 50%;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

input:checked + .slider {
  background-color: rgba(255, 215, 0, 0.6);
}

input:checked + .slider:before {
  transform: translateX(16px);
  background-color: white;
}

input:focus + .slider {
  box-shadow: 0 0 1px rgba(255, 215, 0, 0.6);
}
</style> 