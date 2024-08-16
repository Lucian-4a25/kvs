use once_cell::sync::Lazy;
use slog::{o, Drain, Level, Logger};
use slog_async;
use slog_term;

/// 使用 Lazy 全局共享 Logger 实例
pub static LOGGER: Lazy<Logger> = Lazy::new(|| init_logger());

/// 初始化日志
pub fn init_logger() -> Logger {
    // 创建 stderr 的 TermDecorator 和 Drain（用于 Error 级别日志）
    let decorator_stderr = slog_term::TermDecorator::new().stderr().build();
    let drain_stderr = slog_term::CompactFormat::new(decorator_stderr)
        .build()
        .fuse();
    let drain_stderr = slog_async::Async::new(drain_stderr)
        .build()
        .filter_level(Level::Error)
        .fuse();

    // 创建 stdout 的 TermDecorator 和 Drain（用于非 Error 级别的日志）
    let decorator_stdout = slog_term::TermDecorator::new().stdout().build();
    let drain_stdout = slog_term::CompactFormat::new(decorator_stdout)
        .build()
        .fuse();
    let drain_stdout = slog_async::Async::new(drain_stdout)
        .build()
        .filter(|record| record.level() > Level::Error)
        .fuse();

    // 将 stdout 和 stderr 的 Drain 组合在一起
    let drain = slog::Duplicate::new(drain_stdout, drain_stderr).fuse();

    // 创建根日志对象
    let root_logger = slog::Logger::root(drain, o!());

    root_logger
}
