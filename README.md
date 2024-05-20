# Geektime Rust Simple-redis

一个简单的 redis server 实现

## Task:
1. 如何解析(serialize) Frame
2. enum RespFream {}
3. trait RespEncode / RespDecode (enum dispatch)
4. bytes trait

## Command
客户端的命令是一个RespArray 特定的组合是一个个 command
- SET key val   `"*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n"`
- GET key       `"*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"`
- HGET key field val    `"*4\r\n$4\r\nhset\r\n$3\r\nmap\r\n$5\r\nhello\r\n$5\r\nworld\r\n"`
- HGET key field        `"*3\r\n$4\r\nhget\r\n$3\r\nmap\r\n$5\r\nhello\r\n"`
- HGETALL key   `"*2\r\n$7\r\nhgetall\r\n$3\r\nmap\r\n"`

CommandExecutor trait -> 对于每一个 Command, 处理并返回一个 RespFrame

## Project 总结
- RespFrame
  - 使用 RespEncoder / RespDecoder trait 统一行为
  - 大量 unit Test
- Command
  - 使用 CommandExecutor 统一行为
  - 使用 TryForm 从 RespFrame 获取 Command ("Command must be RespArray")
  - 各种各样的数据结构转换 (byte stream <-> RespFrame <-> Command)
- TcpListener / TcpStream
  - 从核心数据层到网络层开发的原因
  - accept() -> tokio::spawn 开启新任务处理 TcpStream
  - Framed: 把 TcpStream 按帧处理（桥接网络层和应用层）
  - Codec: 实现 Encoder / Decoder
- 共享状态 Arc<T> 内部根据需要使用 DashMap

- 代码重构：resp 目录
- 多线程/异步处理总结
