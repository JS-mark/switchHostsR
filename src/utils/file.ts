import { useBridgeFunc } from "./common"

/**
 * 打开文件
 * @param file
 */
export function openFile(file?: string) {
  if (!file)
    return

  return useBridgeFunc((bridge, resolve, reject) => {
    try {
      bridge.shellShowItemInFolder(file)
      resolve({})
    }
    catch (error) {
      reject(error)
    }
  }, (bridge, resolve, reject) => {
    bridge.invoke('openFile', {
      file,
    }).then(resolve).catch(reject)
  })
}

/**
 * 打开文件夹
 * @param file
 */
export function openDirectory(file?: string) {
  return useBridgeFunc((bridge, resolve, reject) => {
    try {
      const paths = bridge.showOpenDialog({
        title: '选择储存数据文件夹',
        defaultPath: file || bridge.getPath('home'),
        properties: ['openDirectory'],
      })
      resolve(paths && paths[0])
    }
    catch (error) {
      reject(error)
    }
  }, (bridge, resolve, reject) => {
    bridge.invoke('openDirectory', {
      file,
    }).then(resolve).catch(reject)
  })
}
