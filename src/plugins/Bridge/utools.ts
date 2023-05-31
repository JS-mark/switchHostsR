/**
 * 动态加载库
 * @returns
 */
export default function initUTool() {
  return new Promise((resolve) => {
    window.utools && resolve(window.utools);
  });
}
