import { router as _router, create, defaults } from 'json-server'

const server = create()
const router = _router('db.json')
const middlewares = defaults()

server.use(middlewares)
server.use(router)
server.listen(3000, () => {
  console.warn('JSON Server is running')
})
