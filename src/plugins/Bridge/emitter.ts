import mitt, { EventType, Handler } from "mitt";
export namespace Bridge {
  export type Events<T = unknown> = Record<EventType, T>;
}

export default class Emitter {
  private emitter = mitt<{
    [key: keyof Bridge.Events]: any;
  }>();

  /**
   * on
   * @param type
   * @param handler
   */
  on<Key extends keyof Bridge.Events>(
    type: Key,
    handler: Handler<Bridge.Events[Key]>,
  ) {
    this.emitter.on(type, handler);
  }

  /**
   * once
   * @param type
   * @param handler
   */
  once<Key extends keyof Bridge.Events>(
    type: Key,
    handler: Handler<Bridge.Events[Key]>,
  ) {
    this.emitter.on(type, handler);
    this.emitter.off(type, handler);
  }

  /**
   * off
   * @param type
   * @param handler
   */
  off<Key extends keyof Bridge.Events>(
    type: Key,
    handler?: Handler<Bridge.Events[Key]>,
  ) {
    this.emitter.off(type, handler);
  }

  /**
   * emit
   * @param type
   * @param event
   */
  emit<Key extends keyof Bridge.Events>(type: Key, event: Bridge.Events[Key]) {
    this.emitter.emit(type, event);
  }
}
