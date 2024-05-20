# 作业

为 simple-redis 实现你想实现的命令，比如：
1. echo command:  https://redis.io/commands/echo/
2. hmget command:  https://redis.io/commands/hmget/
3. sadd/sismember  https://redis.io/commands/sismember/

重构代码：
 - 删除 NullBulkString / NullArray
 - 重构 BulkString / RespArray 代码，使其直接处理上面两种情况
