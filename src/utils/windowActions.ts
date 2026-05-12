import { createWin } from './windowManager'

/**
 * 打开关于窗口
 */
export async function openAboutWindow() {
  await createWin({
    label: 'about',
    title: '关于我们',
    url: '#/about',
    width: 450,
    height: 360,
    resizable: false
  })
}

/**
 * 打开主页窗口
 */
export async function openMainWindow() {
  await createWin({
    label: 'main',
    title: '主页',
    url: '#/',
    width: 1200,
    height: 800,
    minWidth: 800,
    minHeight: 600
  })
}