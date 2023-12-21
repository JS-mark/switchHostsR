import dayjs from 'dayjs'
import { onUnmounted, ref } from 'vue'

function paddingZero(n: number) {
  return n < 10 ? `0${n}` : n
}

export function formatTime(val: Date | number | string, fullTime = true) {
  val = new Date(val)

  const [year, month, date, hours, minutes, seconds] = [
    val.getFullYear(),
    val.getMonth() + 1,
    val.getDate(),
    val.getHours(),
    val.getMinutes(),
    val.getSeconds(),
  ].map(paddingZero)

  const _date = `${year}/${month}/${date}`

  return fullTime ? `${_date} ${hours}:${minutes}:${seconds}` : _date
}

export function getNowtime() {
  const now = dayjs()
  const formatTime = 'HH:mm:ss'
  const formatDate = 'YYYY-MM-DD'
  const week = [
    '星期日',
    '星期一',
    '星期二',
    '星期三',
    '星期四',
    '星期五',
    '星期六',
  ]
  const clock = ref({
    date: `${now.format(formatDate)} ${week[now.day()]}`,
    time: now.format(formatTime),
  })

  const updateTime = () => {
    const now = dayjs()
    clock.value.date = `${now.format(formatDate)} ${week[now.day()]}`
    clock.value.time = now.format(formatTime)
  }
  const timerID: any = setInterval(updateTime, 1000)
  onUnmounted(() => {
    clearInterval(timerID)
  })
  return clock
}
