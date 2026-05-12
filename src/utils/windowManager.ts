import { WebviewWindow, getAllWebviewWindows, getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { emit, listen } from '@tauri-apps/api/event'

// 窗口配置类型定义
export interface WindowConfig {
  label: string                // 窗口唯一标识（必填，不可重复）
  title?: string               // 窗口标题
  url?: string                 // 窗口对应的Vue路由地址
  width?: number | null        // 窗口宽度
  height?: number | null       // 窗口高度
  minWidth?: number | null     // 最小宽度
  minHeight?: number | null    // 最小高度
  x?: number | null            // 窗口X坐标
  y?: number | null            // 窗口Y坐标
  center?: boolean             // 是否居中显示
  resizable?: boolean          // 是否允许缩放
  maximized?: boolean          // 是否默认最大化
  decorations?: boolean        // 是否显示窗口边框/标题栏
  alwaysOnTop?: boolean        // 是否置顶
  dragDropEnabled?: boolean    // 是否允许拖放
  visible?: boolean            // 是否默认可见
  skipTaskbar?: boolean        // 是否隐藏任务栏图标
  devtools?: boolean           // 是否可以开启开调试窗口
  [key: string]: any          // 扩展其他配置
}

// 默认窗口配置
const defaultWindowConfig: WindowConfig = {
  label: '',
  title: '',
  url: '',
  width: null,
  height: null,
  minWidth: null,
  minHeight: null,
  x: null,
  y: null,
  center: true,
  resizable: true,
  maximized: false,
  decorations: false,
  alwaysOnTop: false,
  dragDropEnabled: false,
  visible: true,
  skipTaskbar: false,
  devtools:false
}

class WindowsManager {
  constructor() {
    // 初始化全局事件监听
    this.initListen()
  }

  /**
   * 创建新窗口
   * @param options 窗口配置
   */
  async createWin(options: WindowConfig) {
    const args = Object.assign({}, defaultWindowConfig, options)

    // 检查窗口是否已存在，存在则直接激活
    const existWin = await this.getWin(args.label)
    if (existWin) {
      console.log('窗口已存在，直接激活>>', args.label)
      await existWin.show()
      await existWin.unminimize()
      await existWin.setFocus()
      return
    }

    // 创建新窗口
    const win = new WebviewWindow(args.label, args.defaultWindowConfig)

    // 窗口创建成功回调
    win.once('tauri://created', async () => {
      console.log('窗口创建成功>>', args.label)
      // 如果配置了最大化，自动最大化
      if (args.maximized && args.resizable) {
        await win.maximize()
      }
    })

    // 窗口创建失败回调
    win.once('tauri://error', async (error) => {
      console.error('窗口创建失败!', error)
    })
  }

  /**
   * 根据label获取窗口
   */
  async getWin(label: string) {
    return await WebviewWindow.getByLabel(label)
  }

  /**
   * 获取所有窗口
   */
  async getAllWin() {
    return await getAllWebviewWindows()
  }

  /**
   * 初始化全局事件监听，处理跨窗口通信
   */
  private async initListen() {
    const appWindow = getCurrentWebviewWindow()
    console.log('初始化窗口事件监听>>', appWindow.label)

    // 监听创建窗口事件
    await listen<WindowConfig>('win-create', async (event) => {
      this.createWin(event.payload)
    })

    // 监听显示主窗口事件
    await listen('win-show', async () => {
      if (appWindow.label === 'main') {
        await appWindow.show()
        await appWindow.unminimize()
        await appWindow.setFocus()
      }
    })

    // 监听隐藏主窗口事件
    await listen('win-hide', async () => {
      if (appWindow.label === 'main') {
        await appWindow.hide()
      }
    })

    // 监听关闭窗口事件
    await listen('win-close', async () => {
      await appWindow.close()
    })
  }
}

// 全局事件触发方法，供页面调用
export async function createWin(options: WindowConfig) {
  await emit('win-create', options)
}

// 导出单例
export default new WindowsManager()