/**
 * 动态加载库
 * @returns
 */
export function initUTool() {
  return new Promise((resolve) => {
    window.utools && resolve(window.utools);
  });
}
