wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"
wrk.body = [[
{
  "email": "kayano@example.com",
  "password": "123456"
}
]]
--注意，这个wrk是一个测试工具，如果你的ide显示语法错误，请忽略它。