# Geektime Rust Simple-redis

一个简单的 redis server 实现

Task:
1. 如何解析(serialize) Frame
2. enum RespFream {}
3. trait RespEncode / RespDecode (enum dispatch)
4. bytes trait

Command
客户端的命令是一个RespArray 特定的组合是一个个 command
- SET key val   `"*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n"`
- GET key       `"*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"`
- HGET key field val    `"*4\r\n$4\r\nhset\r\n$3\r\nmap\r\n$5\r\nhello\r\n$5\r\nworld\r\n"`
- HGET key field        `"*3\r\n$4\r\nhget\r\n$3\r\nmap\r\n$5\r\nhello\r\n"`
- HGETALL key   `"*2\r\n$7\r\nhgetall\r\n$3\r\nmap\r\n"`

CommandExecutor trait -> 对于每一个 Command, 处理并返回一个 RespFrame
