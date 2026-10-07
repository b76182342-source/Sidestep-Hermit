import { useCallback, useState } from 'react'

/**
 * 进度条拖拽。
 *
 * 按住后指针位置直接驱动 UI；只有松手时才真正下发 seek，
 * 拖拽期间忽略播放器回传的进度，避免指针被进度回调拽回去。
 */
export function useSeekDrag(duration: number, onSeek: (ratio: number) => void) {
  const [dragRatio, setDragRatio] = useState<number | null>(null)

  const ratioAt = (el: HTMLElement, clientX: number) => {
    const rect = el.getBoundingClientRect()
    if (rect.width === 0) return 0
    return Math.min(1, Math.max(0, (clientX - rect.left) / rect.width))
  }

  const handlers = {
    onPointerDown: useCallback((e: React.PointerEvent<HTMLDivElement>) => {
      e.preventDefault()
      e.currentTarget.setPointerCapture(e.pointerId)
      setDragRatio(ratioAt(e.currentTarget, e.clientX))
    }, []),

    onPointerMove: useCallback((e: React.PointerEvent<HTMLDivElement>) => {
      if (!e.currentTarget.hasPointerCapture(e.pointerId)) return
      setDragRatio(ratioAt(e.currentTarget, e.clientX))
    }, []),

    onPointerUp: useCallback(
      (e: React.PointerEvent<HTMLDivElement>) => {
        if (!e.currentTarget.hasPointerCapture(e.pointerId)) return
        const ratio = ratioAt(e.currentTarget, e.clientX)
        e.currentTarget.releasePointerCapture(e.pointerId)
        setDragRatio(null)
        if (duration > 0) onSeek(ratio)
      },
      [duration, onSeek]
    ),

    onPointerCancel: useCallback((e: React.PointerEvent<HTMLDivElement>) => {
      if (e.currentTarget.hasPointerCapture(e.pointerId)) {
        e.currentTarget.releasePointerCapture(e.pointerId)
      }
      setDragRatio(null)
    }, []),
  }

  return { dragRatio, handlers, isDragging: dragRatio !== null }
}
