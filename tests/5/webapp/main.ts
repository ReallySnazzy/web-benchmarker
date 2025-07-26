import { Hono } from 'hono'

const app = new Hono()

app.get('/', (c) => {
  return c.text('Hello, World!')
})

Deno.serve({ port: 3000, hostname: '0.0.0.0' }, app.fetch)
