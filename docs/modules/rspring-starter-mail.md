# rspring-starter-mail

[![crates.io](https://img.shields.io/crates/v/rspring-starter-mail.svg)](https://crates.io/crates/rspring-starter-mail)
[![docs.rs](https://img.shields.io/docsrs/rspring-starter-mail)](https://docs.rs/rspring-starter-mail)

**rspring-starter-mail** 是 RSpring 框架的邮件发送 Starter，集成 Rust 邮件客户端库并提供了简化的邮件发送功能。通过简单的配置即可实现邮件发送，支持邮件模板和类似 Spring Boot 的邮件功能。

## 🎯 核心功能

- **邮件客户端集成** - 集成 Rust 优秀的邮件客户端库
- **自动装配** - 自动配置 SMTP 连接和邮件发送器
- **简单配置** - 通过配置文件完成邮件服务设置
- **邮件模板** - 支持 HTML 和文本邮件模板
- **附件支持** - 支持邮件附件发送
- **异步发送** - 基于 tokio 的异步邮件发送

## 📦 安装

```toml
[dependencies]
rspring-starter-mail = "0.1.0"
rspring-core = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## 🚀 快速开始

### 基础配置

```toml
# application.toml
[mail]
host = "smtp.gmail.com"
port = 587
username = "your-email@gmail.com"
password = "your-app-password"
from = "your-email@gmail.com"
from_name = "Your App"

# 可选配置
[mail.advanced]
use_tls = true
timeout = 30
pool_size = 10
```

### 发送简单邮件

```rust
use rspring_core::*;
use rspring_starter_mail::*;

#[derive(Service)]
pub struct NotificationService {
    mail_sender: Arc<MailSender>,
}

impl NotificationService {
    pub fn new(mail_sender: Arc<MailSender>) -> Self {
        Self { mail_sender }
    }
    
    pub async fn send_welcome_email(&self, to: &str, username: &str) -> Result<()> {
        let mail = SimpleMailBuilder::new()
            .to(to)
            .subject("欢迎使用我们的服务")
            .text(&format!("你好 {}，欢迎使用我们的服务！", username))
            .build();
            
        self.mail_sender.send(mail).await
    }
    
    pub async fn send_html_email(&self, to: &str) -> Result<()> {
        let html_content = r#"
        <html>
        <body>
            <h1>欢迎！</h1>
            <p>这是一封 <strong>HTML</strong> 邮件。</p>
        </body>
        </html>
        "#;
        
        let mail = SimpleMailBuilder::new()
            .to(to)
            .subject("HTML 邮件")
            .html(html_content)
            .build();
            
        self.mail_sender.send(mail).await
    }
}
```

### 使用邮件模板

```rust
use rspring_starter_mail::template::*;

#[derive(Service)]
pub struct EmailTemplateService {
    template_engine: Arc<MailTemplateEngine>,
    mail_sender: Arc<MailSender>,
}

impl EmailTemplateService {
    pub fn new(
        template_engine: Arc<MailTemplateEngine>,
        mail_sender: Arc<MailSender>
    ) -> Self {
        Self { template_engine, mail_sender }
    }
    
    pub async fn send_template_email(
        &self, 
        to: &str, 
        template_name: &str,
        context: serde_json::Value
    ) -> Result<()> {
        // 渲染模板
        let content = self.template_engine.render(template_name, &context)?;
        
        let mail = SimpleMailBuilder::new()
            .to(to)
            .subject(&content.subject)
            .html(&content.html)
            .text(&content.text)
            .build();
            
        self.mail_sender.send(mail).await
    }
}
```

### 发送带附件的邮件

```rust
impl NotificationService {
    pub async fn send_email_with_attachment(&self, to: &str) -> Result<()> {
        let attachment = Attachment::from_file("report.pdf", "application/pdf")?;
        
        let mail = SimpleMailBuilder::new()
            .to(to)
            .subject("月度报告")
            .text("请查收本月度报告。")
            .attachment(attachment)
            .build();
            
        self.mail_sender.send(mail).await
    }
}
```

## 🔧 核心 API

### MailSender

邮件发送器，负责实际的邮件发送。

```rust
impl MailSender {
    /// 发送单个邮件
    pub async fn send(&self, mail: SimpleMail) -> Result<()>;
    
    /// 批量发送邮件
    pub async fn send_batch(&self, mails: Vec<SimpleMail>) -> Result<Vec<Result<()>>>;
    
    /// 异步发送邮件（不等待结果）
    pub async fn send_async(&self, mail: SimpleMail) -> Result<()>;
}
```

### SimpleMailBuilder

邮件构建器，用于构建邮件。

```rust
impl SimpleMailBuilder {
    /// 创建新的邮件构建器
    pub fn new() -> Self;
    
    /// 设置收件人
    pub fn to(mut self, to: &str) -> Self;
    
    /// 设置多个收件人
    pub fn to_multiple(mut self, to: Vec<String>) -> Self;
    
    /// 设置抄送
    pub fn cc(mut self, cc: &str) -> Self;
    
    /// 设置密送
    pub fn bcc(mut self, bcc: &str) -> Self;
    
    /// 设置主题
    pub fn subject(mut self, subject: &str) -> Self;
    
    /// 设置文本内容
    pub fn text(mut self, text: &str) -> Self;
    
    /// 设置 HTML 内容
    pub fn html(mut self, html: &str) -> Self;
    
    /// 添加附件
    pub fn attachment(mut self, attachment: Attachment) -> Self;
    
    /// 构建邮件
    pub fn build(self) -> SimpleMail;
}
```

### MailTemplateEngine

邮件模板引擎，支持模板渲染。

```rust
impl MailTemplateEngine {
    /// 渲染模板
    /// 
    /// # 参数
    /// - `template_name`: 模板名称（不包含扩展名）
    /// - `context`: 模板上下文数据
    pub fn render(&self, template_name: &str, context: &serde_json::Value) -> Result<TemplateContent>;
    
    /// 注册模板
    pub fn register_template(&mut self, name: &str, content: &str) -> Result<()>;
    
    /// 从文件加载模板
    pub fn load_template_from_file(&mut self, name: &str, path: &str) -> Result<()>;
}
```

## ⚙️ 配置详解

### 基础配置

```toml
[mail]
# SMTP 服务器配置
host = "smtp.gmail.com"        # SMTP 主机
port = 587                     # SMTP 端口
username = "user@gmail.com"    # 用户名
password = "app-password"      # 密码或应用密码

# 发件人信息
from = "user@gmail.com"        # 发件人邮箱
from_name = "My Application"   # 发件人名称

# 安全配置
use_tls = true                 # 启用 TLS
use_ssl = false               # 启用 SSL（与 TLS 二选一）

# 连接配置
timeout = 30                   # 连接超时（秒）
pool_size = 10                # 连接池大小
```

### 模板配置

```toml
[mail.templates]
# 模板存储路径
path = "templates/mail"
# 模板引擎（支持 handlebars, tera）
engine = "handlebars"
# 模板缓存
cache_enabled = true
cache_size = 100
```

### 高级配置

```toml
[mail.advanced]
# 重试配置
max_retries = 3
retry_delay = 5

# 批量发送
batch_size = 50
batch_delay = 1000  # 毫秒

# 日志配置
log_emails = true
log_level = "info"

# 开发模式（所有邮件发送到指定地址）
dev_mode = true
dev_email = "dev@example.com"
```

## 📧 邮件模板

### 模板文件结构

```
templates/mail/
├── welcome/
│   ├── subject.txt          # 邮件主题模板
│   ├── content.html         # HTML 内容模板
│   └── content.txt          # 纯文本内容模板
├── reset-password/
│   ├── subject.txt
│   ├── content.html
│   └── content.txt
└── notification/
    ├── subject.txt
    └── content.html
```

### 模板示例

**templates/mail/welcome/subject.txt:**
```
欢迎加入 {{app_name}}，{{username}}！
```

**templates/mail/welcome/content.html:**
```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>欢迎</title>
</head>
<body>
    <h1>欢迎 {{username}}！</h1>
    <p>感谢您注册 {{app_name}}。</p>
    <p>您的账户信息：</p>
    <ul>
        <li>用户名: {{username}}</li>
        <li>邮箱: {{email}}</li>
        <li>注册时间: {{signup_date}}</li>
    </ul>
    <p>
        <a href="{{activation_url}}">点击这里激活您的账户</a>
    </p>
</body>
</html>
```

### 使用模板

```rust
use serde_json::json;

// 准备模板数据
let context = json!({
    "app_name": "我的应用",
    "username": "张三",
    "email": "zhangsan@example.com",
    "signup_date": "2024-01-15",
    "activation_url": "https://example.com/activate?token=xxx"
});

// 发送模板邮件
email_service.send_template_email(
    "zhangsan@example.com",
    "welcome",
    context
).await?;
```

## 🧪 测试支持

### 邮件测试工具

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rspring_starter_mail::test_utils::*;
    
    #[tokio::test]
    async fn test_send_email() {
        // 使用测试邮件发送器
        let mail_sender = TestMailSender::new();
        
        let service = NotificationService::new(Arc::new(mail_sender.clone()));
        
        // 发送测试邮件
        service.send_welcome_email("test@example.com", "测试用户").await.unwrap();
        
        // 验证邮件是否发送
        let sent_mails = mail_sender.get_sent_mails().await;
        assert_eq!(sent_mails.len(), 1);
        
        let mail = &sent_mails[0];
        assert_eq!(mail.to, vec!["test@example.com"]);
        assert!(mail.text.contains("测试用户"));
    }
    
    #[tokio::test]
    async fn test_template_rendering() {
        let template_engine = TestTemplateEngine::new();
        
        let context = json!({
            "username": "测试用户"
        });
        
        let content = template_engine.render("welcome", &context).unwrap();
        assert!(content.html.contains("测试用户"));
    }
}
```

## 🚀 最佳实践

### 邮件服务设计

```rust
// ✅ 好的实践
#[derive(Service)]
pub struct EmailService {
    mail_sender: Arc<MailSender>,
    template_engine: Arc<MailTemplateEngine>,
    config: Arc<ConfigurationManager>,
}

impl EmailService {
    /// 发送欢迎邮件
    pub async fn send_welcome(&self, user: &User) -> Result<()> {
        let context = json!({
            "username": user.name,
            "email": user.email,
            "activation_url": self.generate_activation_url(&user.id)?
        });
        
        self.send_template_email(&user.email, "welcome", context).await
    }
    
    /// 发送密码重置邮件
    pub async fn send_password_reset(&self, email: &str, token: &str) -> Result<()> {
        let reset_url = format!("https://example.com/reset?token={}", token);
        let context = json!({
            "reset_url": reset_url,
            "expires_in": "24小时"
        });
        
        self.send_template_email(email, "reset-password", context).await
    }
    
    /// 通用模板邮件发送
    async fn send_template_email(
        &self, 
        to: &str, 
        template: &str, 
        context: serde_json::Value
    ) -> Result<()> {
        let content = self.template_engine.render(template, &context)?;
        
        let mail = SimpleMailBuilder::new()
            .to(to)
            .subject(&content.subject)
            .html(&content.html)
            .text(&content.text)
            .build();
            
        self.mail_sender.send(mail).await
    }
}
```

### 错误处理

```rust
#[derive(thiserror::Error, Debug)]
pub enum MailError {
    #[error("SMTP 连接失败: {0}")]
    Connection(String),
    
    #[error("邮件发送失败: {0}")]
    SendFailed(String),
    
    #[error("模板渲染失败: {0}")]
    TemplateError(String),
    
    #[error("配置错误: {0}")]
    Configuration(String),
}

// 统一错误处理
impl EmailService {
    pub async fn send_with_retry(&self, mail: SimpleMail) -> Result<()> {
        let max_retries = 3;
        let mut attempt = 0;
        
        while attempt < max_retries {
            match self.mail_sender.send(mail.clone()).await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    attempt += 1;
                    if attempt >= max_retries {
                        return Err(e);
                    }
                    tokio::time::sleep(Duration::from_secs(1 << attempt)).await;
                }
            }
        }
        
        unreachable!()
    }
}
```

## 🔗 相关链接

- [完整 API 文档](../../api/starter-mail.md)
- [邮件模板指南](../../guide/mail-templates.md)
- [SMTP 配置指南](../../guide/smtp-configuration.md)
- [GitHub 仓库](https://github.com/hi-liyan/rspring)
- [示例代码](../../examples/mail-service/)