/** 展示相关的小工具 */

/** 由字符串稳定散列出封面底色，取值来自设计稿色板 */
const COVERS = ['#e3c9b6', '#d9b79d', '#c8cfbf', '#cfc6d8', '#b9cbd6', '#d6e0e8']

export function coverColor(seed: string): string {
  if (!seed) return COVERS[0]

  let h = 0
  for (let i = 0; i < seed.length; i += 1) {
    h = (h * 31 + seed.charCodeAt(i)) >>> 0
  }
  return COVERS[h % COVERS.length]
}

/** 秒 → `m:ss` */
export function formatTime(seconds: number): string {
  if (!Number.isFinite(seconds) || seconds < 0) return '0:00'

  const m = Math.floor(seconds / 60)
  const s = Math.floor(seconds % 60)
  return `${m}:${s.toString().padStart(2, '0')}`
}

/** 按当前时间给出问候语 */
export function greeting(): string {
  const hour = new Date().getHours()
  if (hour < 6) return '夜深了'
  if (hour < 12) return '早上好'
  if (hour < 18) return '下午好'
  return '晚上好'
}
