import type { App } from 'vue'

export function useAppHandler(app: App) {
  // error
  app.config.errorHandler = (err, instance, info) => {
    // handle error, e.g. report to a service
    console.error('errorHandler', err, instance, info)
    if (import.meta.env.PROD) {
      // @ts-expect-error
      window._ERR_REP && window._ERR_REP.reportCustomError(err)
    }
  }

  // warn
  app.config.warnHandler = (msg, instance, trace) => {
    // `trace` is the component hierarchy trace
    console.warn(msg, instance, trace)
  }

  app.config.performance = import.meta.env.DEV
}
