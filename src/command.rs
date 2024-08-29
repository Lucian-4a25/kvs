/// Redis 支持的所有指令
#[derive(Debug)]
pub enum ClientCommand {
    // 设置一个key 的值
    Set { key: String, value: String },
    // 获取一个 key 的值
    Get { key: String },
    // 移除一个 key 的值
    Remove { key: String },
    // 测试命令
    PING,
}
