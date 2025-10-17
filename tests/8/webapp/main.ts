import { Hono } from 'hono'

const app = new Hono()

app.get('/', (c) => {
  return c.text('Hello, World!')
})

export default {
  port: 3000,
  hostname: '0.0.0.0',
  fetch: app.fetch,
}
