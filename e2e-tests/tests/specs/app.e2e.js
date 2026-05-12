import assert from 'node:assert/strict'

let createdHostName = ''

const wait = async (ms) => {
  await browser.pause(ms)
}

const clickByTestId = async (testId) => {
  const el = await $(`[data-testid="${testId}"]`)
  await el.waitForExist({ timeout: 60000 })
  await el.click()
}

const typeByTestId = async (testId, value) => {
  const el = await $(`[data-testid="${testId}"]`)
  await el.waitForExist({ timeout: 60000 })
  await el.click()
  await el.setValue(value)
}

const clickButtonByText = async (text) => {
  const el = await $(`//button[contains(normalize-space(.), "${text}")]`)
  await el.waitForExist({ timeout: 60000 })
  await el.click()
}

describe('Switch Hosts R E2E', () => {
  it('注册 -> 进入首页', async () => {
    const registerTab = await $('//div[contains(@class,"n-tabs-tab") and contains(normalize-space(.), "注册")]')
    await registerTab.waitForExist({ timeout: 60000 })
    await registerTab.click()

    const uid = `${Date.now()}`
    await typeByTestId('register-nickname', `u_${uid}`)
    await typeByTestId('register-email', `u_${uid}@example.com`)
    await typeByTestId('register-password', 'Passw0rd!')
    await clickByTestId('register-submit')

    const title = await $('//h1[contains(normalize-space(.), "Switch Hosts R")]')
    await title.waitForExist({ timeout: 60000 })
    assert.ok(await title.isDisplayed())
  })

  it('创建 Hosts -> 出现在列表中', async () => {
    await clickByTestId('home-action-manage-hosts')

    await clickByTestId('hosts-new')

    const drawerHeader = await $('//div[contains(@class,"n-drawer-header") and contains(normalize-space(.), "创建 Hosts")]')
    await drawerHeader.waitForExist({ timeout: 60000 })

    await clickByTestId('addhosts-type')
    const localOption = await $('//div[contains(@class,"n-base-select-option")][.//*[contains(normalize-space(.), "本地")]]')
    await localOption.waitForExist({ timeout: 60000 })
    await localOption.click()

    createdHostName = `hosts_${Date.now()}`
    await typeByTestId('addhosts-title', createdHostName)

    await clickByTestId('addhosts-confirm')

    await wait(500)
    const hostsTitle = await $(`//h3[contains(normalize-space(.), "${createdHostName}")]`)
    await hostsTitle.waitForExist({ timeout: 60000 })
    assert.ok(await hostsTitle.isDisplayed())
  })

  it('启用/禁用 Hosts', async () => {
    const toggleBtn = await $('[data-testid="hosts-toggle-active"]')
    await toggleBtn.waitForExist({ timeout: 60000 })
    const beforeText = await toggleBtn.getText()
    await toggleBtn.click()

    await clickButtonByText('确定')

    await browser.waitUntil(async () => {
      const t = await toggleBtn.getText()
      return t !== beforeText
    }, { timeout: 60000 })
  })

  it('删除 Hosts', async () => {
    const deleteBtn = await $('[data-testid="hosts-delete"]')
    await deleteBtn.waitForExist({ timeout: 60000 })
    await deleteBtn.click()

    await clickButtonByText('确定删除')

    await browser.waitUntil(async () => {
      const el = await $(`//h3[contains(normalize-space(.), "${createdHostName}")]`)
      return !(await el.isExisting())
    }, { timeout: 60000 })
  })
})
