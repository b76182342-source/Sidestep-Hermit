import { useCallback, useState } from 'react'
import { actions } from '../actions'

/**
 * 音量条拖拽。拖拽期间用本地值渲染，松手才下发，避免与后端回传值打架。
 */
export function useVolumeDrag(storeVolume: number) {
  const [dragValue, setDragValue] = useState<number | null>(null)

  const ratioAt = (el: HTMLElement, clientX: number) => {
    const rect = el.getBoundingClientRect()
    if (rect.width === 0) return 0
    return Math.min(1, Math.max(0, (clientX - rect.left) / rect.width))
  }

  const handlers = {
    onPointerDown: useCallback((e: React.PointerEvent<HTMLDivElement>) => {
      e.preventDefault()
      e.currentTarget.setPointerCapture(e.pointerId)
      setDragValue(ratioAt(e.currentTarget, e.clientX))
    }, []),

    onPointerMove: useCallback((e: React.PointerEvent<HTMLDivElement>) => {
      if (!e.currentTarget.hasPointerCapture(e.pointerId)) return
      setDragValue(ratioAt(e.currentTarget, e.clientX))
    }, []),

    onPointerUp: useCallback((e: React.PointerEvent<HTMLDivElement>) => {
      if (!e.currentTarget.hasPointerCapture(e.pointerId)) return
      const ratio = ratioAt(e.currentTarget, e.clientX)
      e.currentTarget.releasePointerCapture(e.pointerId)
      setDragValue(null)
      void actions.setVolume(ratio)
    }, []),

    onPointerCancel: useCallback((e: React.PointerEvent<HTMLDivElement>) => {
      if (e.currentTarget.hasPointerCapture(e.pointerId)) {
        e.currentTarget.releasePointerCapture(e.pointerId)
      }
      setDragValue(null)
    }, []),
  }

  return {
    value: dragValue ?? storeVolume,
    handlers,
    isDragging: dragValue !== null,
  }
}
