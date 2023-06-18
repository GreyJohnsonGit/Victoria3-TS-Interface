use colored::Colorize;

/// Severity of logged data.
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
  /// Routine and inconsequential.
  Debug,

  /// Routine but important.
  Info,

  /// Unexpected but acceptable.
  Warning,

  /// Potentially fatal.
  Error,

  /// Unrecoverable.
  Fatal
}

/// Describes structs which implement basic logging functionality.
pub trait ILogger {
  /// Logs given `message` at given `level` to the loggers output.
  fn log(&self, level: LogLevel, message: &String);
  /// str slice version of `log`.
  fn log_str(&self, level: LogLevel, message: &str);
  /// array version of `log`.
  fn log_array(&self, level: LogLevel, messages: &Vec<String>);
  
  /// Create a copy of the current logger.
  fn clone_boxed(&self) -> Box<dyn ILogger>;

  /// Log message at Debug level.
  fn debug(&self, message: &String) {
    self.log(LogLevel::Debug, message);
  }

  /// str slice version of `debug`.
  fn debug_str(&self, message: &str) {
    self.log_str(LogLevel::Debug, message);
  }

  /// array version of `debug`.
  fn debug_array(&self, messages: &Vec<String>) {
    self.log_array(LogLevel::Debug, messages);
  }

  /// Log message at Info level.
  fn info(&self, message: &String) {
    self.log(LogLevel::Info, message);
  }

  /// str slice version of `info`.
  fn info_str(&self, message: &str) {
    self.log_str(LogLevel::Info, message);
  }

  /// array version of `info`.
  fn info_array(&self, messages: &Vec<String>) {
    self.log_array(LogLevel::Info, messages);
  }

  /// Log message at Warning level.
  fn warning(&self, message: &String) {
    self.log(LogLevel::Warning, message);
  }

  /// str slice version of `warning`.
  fn warning_str(&self, message: &str) {
    self.log_str(LogLevel::Warning, message);
  }

  /// array version of `warning`.
  fn warning_array(&self, messages: &Vec<String>) {
    self.log_array(LogLevel::Warning, messages);
  }

  /// Log message at Error level.
  fn error(&self, message: &String) {
    self.log(LogLevel::Error, message);
  }

  /// str slice version of `error`.
  fn error_str(&self, message: &str) {
    self.log_str(LogLevel::Error, message);
  }

  /// array version of `error`.
  fn error_array(&self, messages: &Vec<String>) {
    self.log_array(LogLevel::Error, messages);
  }

  /// Log message at Fatal level.
  fn fatal(&self, message: &String) {
    self.log(LogLevel::Fatal, message);
  }

  /// str slice version of `fatal`.
  fn fatal_str(&self, message: &str) {
    self.log_str(LogLevel::Fatal, message);
  }

  /// array version of `fatal`.
  fn fatal_array(&self, messages: &Vec<String>) {
    self.log_array(LogLevel::Fatal, messages);
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Logger {}

impl Logger {
  pub fn new() -> Logger {
    Logger {}
  }

  pub fn new_boxed() -> Box<dyn ILogger> {
    Box::new(Self::new())
  }
}

impl ILogger for Logger {
  fn log(&self, level: LogLevel, message: &String) {
    let level_str = match level {
      LogLevel::Debug => "Debug".blue().to_string(),
      LogLevel::Info => "Info".green().to_string(),
      LogLevel::Warning => "Warning".yellow().to_string(),
      LogLevel::Error => "Error".red().to_string(),
      LogLevel::Fatal => "Fatal".bright_red().to_string()
    };
    
    println!("[{}] {}", level_str, message);
  }
  
  fn log_str(&self, level: LogLevel, message: &str) {
    self.log(level, &message.to_string());
  }
  
  fn log_array(&self, level: LogLevel, messages: &Vec<String>) {
    for message in messages {
      self.log(level, message);
    }
  }
  
  fn clone_boxed(&self) -> Box<dyn ILogger> {
    Box::from(Clone::clone(self as &Logger))
  }
}