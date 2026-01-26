import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

interface VersionInfo {
  current: string
  latest: string
  hasUpdate: boolean
  releaseUrl: string
  releaseNotes: string
}

// 全局版本状态
const versionInfo = ref<VersionInfo | null>(null)

// 获取当前版本
async function getCurrentVersion(): Promise<string> {
  try {
    const appInfo = await invoke('get_app_info') as string
    const match = appInfo.match(/v(\d+\.\d+\.\d+)/)
    const version = match ? match[1] : '0.2.0'
    return version
  }
  catch (error) {
    console.error('获取当前版本失败:', error)
    return '0.2.0'
  }
}

// 获取版本信息（如果没有则初始化）
async function getVersionInfo(): Promise<VersionInfo | null> {
  if (!versionInfo.value) {
    const currentVersion = await getCurrentVersion()
    versionInfo.value = {
      current: currentVersion,
      latest: currentVersion,
      hasUpdate: false,
      releaseUrl: '',
      releaseNotes: '',
    }
  }
  return versionInfo.value
}

// 简化的安全打开链接函数
async function safeOpenUrl(url: string): Promise<void> {
  try {
    await invoke('open_external_url', { url })
  }
  catch {
    try {
      await navigator.clipboard.writeText(url)
      throw new Error(`无法自动打开链接，已复制到剪贴板，请手动打开: ${url}`)
    }
    catch {
      throw new Error(`无法打开链接，请手动访问: ${url}`)
    }
  }
}

export function useVersionCheck() {
  return {
    versionInfo,
    getVersionInfo,
    safeOpenUrl,
  }
}
