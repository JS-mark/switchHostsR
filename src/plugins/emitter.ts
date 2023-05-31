/*
 * @Author: Mark
 * @Date: 2022-03-03 16:16:03
 * @LastEditors: Mark
 * @LastEditTime: 2023-02-20 00:57:19
 * @Description: mitt
 */

import mitt from "mitt";
import { type RouteRecordRaw } from "vue-router";

export default mitt<{
  [key: string]: any;
  onRestoreHistoryRoute: RouteRecordRaw[];
}>();
