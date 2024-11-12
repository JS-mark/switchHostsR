import type { editor } from 'monaco-editor'
import type { Emitter, EventType } from 'mitt'
import type { RouteRecordRaw } from 'vue-router'

import mitt from 'mitt'
import { keys, unset } from 'lodash-es'

export interface EmitterEvents {
  'editor-init': { editor: editor.IStandaloneCodeEditor, id: string }
  'on-add-route': void
  'on-restore-history-route': RouteRecordRaw[]
}

/**
 * mitt 事件监听
 */
export function useEmitter<T extends Record<EventType, unknown>>(): Emitter<EmitterEvents & T>
export function useEmitter<T extends Record<EventType, unknown>>(cb: (data: Emitter<EmitterEvents & T>) => void): undefined
export function useEmitter<T extends Record<EventType, unknown>>(cb?: (data: Emitter<EmitterEvents & T>) => void): Emitter<EmitterEvents & T> | undefined {
  const emitterIns = mitt<EmitterEvents & T>()
  if (cb) {
    cb(emitterIns)
    return
  }
  return emitterIns
}

/**
 * 全局事件
 */
export const globalEventEmitter = useEmitter()

export type Listener<T = any> = (data?: T) => void

export class EventEmitter<T = any> {
  private events: { [eventName: string]: Listener<T>[] } = {}

  public on(eventName: string, listener: Listener<T>) {
    if (!this.events[eventName]) {
      this.events[eventName] = []
    }
    this.events[eventName].push(listener)
  }

  public emit(eventName: string, data?: T) {
    const listeners = this.events[eventName]
    if (listeners && listeners.length > 0) {
      listeners.forEach(listener => listener(data))
    }
  }

  public off(eventName: string, listener: Listener<T>) {
    const listeners = this.events[eventName]
    if (listeners) {
      const index = listeners.indexOf(listener)
      if (index !== -1) {
        listeners.splice(index, 1)
      }
    }
  }

  public offAll(eventName: string) {
    if (this.events[eventName]) {
      delete this.events[eventName]
    }
  }

  public removeAllListener() {
    keys(this.events).map((key) => {
      unset(this.events, key)
    })
  }

  public once(eventName: string, listener: Listener<T>) {
    const onceWrapper: Listener<T> = (data) => {
      listener(data)
      this.off(eventName, onceWrapper)
    }
    this.on(eventName, onceWrapper)
  }
}
