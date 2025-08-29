use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemStruct};

/// 应用程序入口注解
/// 
/// 标记一个结构体为 RSpring 应用程序入口点，会自动生成 run 方法
/// 
/// # 示例
/// 
/// ```rust
/// use rspring_core::*;
/// 
/// #[rspring_application]
/// pub struct Application;
/// 
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     Application::run().await
/// }
/// ```
#[proc_macro_attribute]
pub fn rspring_application(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let struct_name = &input.ident;

    let expanded = quote! {
        #input

        impl #struct_name {
            pub async fn run() -> crate::Result<()> {
                // 初始化日志系统
                tracing_subscriber::fmt()
                    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
                    .init();

                tracing::info!("启动 RSpring 应用程序");

                // 创建应用上下文
                let context = crate::ApplicationContext::new().await?;
                
                tracing::info!("应用上下文初始化完成");

                // 保持运行（非Web应用需要自定义实现）
                tokio::signal::ctrl_c().await.map_err(|e| {
                    crate::Error::runtime(format!("等待关闭信号失败: {}", e))
                })?;

                tracing::info!("应用程序已停止");
                Ok(())
            }
        }

        impl crate::RSpringApplication for #struct_name {
            async fn run() -> crate::Result<()> {
                Self::run().await
            }
        }
    };

    TokenStream::from(expanded)
}

/// 组件注解
/// 
/// 标记一个结构体为通用组件，可以被依赖注入容器管理
/// 
/// # 示例
/// 
/// ```rust
/// #[derive(Component)]
/// pub struct MyComponent {
///     // 组件字段
/// }
/// ```
#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl crate::Component for #name {
            fn component_name(&self) -> &'static str {
                stringify!(#name)
            }
        }
    };

    TokenStream::from(expanded)
}

/// 服务组件注解
/// 
/// 标记一个结构体为服务组件，通常包含业务逻辑
/// 
/// # 示例
/// 
/// ```rust
/// #[derive(Service)]
/// pub struct UserService {
///     user_repository: Arc<UserRepository>,
/// }
/// ```
#[proc_macro_derive(Service)]
pub fn service_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl crate::Component for #name {
            fn component_name(&self) -> &'static str {
                stringify!(#name)
            }
        }
        
        impl crate::Service for #name {}
    };

    TokenStream::from(expanded)
}

/// 仓储组件注解
/// 
/// 标记一个结构体为仓储组件，负责数据访问
/// 
/// # 示例
/// 
/// ```rust
/// #[derive(Repository)]
/// pub struct UserRepository {
///     db_pool: Arc<DbPool>,
/// }
/// ```
#[proc_macro_derive(Repository)]
pub fn repository_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl crate::Component for #name {
            fn component_name(&self) -> &'static str {
                stringify!(#name)
            }
        }
        
        impl crate::Repository for #name {}
    };

    TokenStream::from(expanded)
}

/// 配置类注解
/// 
/// 标记一个结构体为配置类，可以从配置文件中自动绑定值
/// 
/// # 示例
/// 
/// ```rust
/// #[derive(Configuration, Deserialize)]
/// pub struct DatabaseConfig {
///     pub host: String,
///     pub port: u16,
/// }
/// ```
#[proc_macro_derive(Configuration)]
pub fn configuration_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl crate::Configuration for #name {
            fn config_prefix(&self) -> &'static str {
                // 将驼峰命名转换为小写下划线命名
                // DatabaseConfig -> database_config
                stringify!(#name)
                    .chars()
                    .enumerate()
                    .fold(String::new(), |mut acc, (i, c)| {
                        if c.is_uppercase() && i > 0 {
                            acc.push('_');
                        }
                        acc.push(c.to_lowercase().next().unwrap());
                        acc
                    })
                    .leak()
            }
        }
    };

    TokenStream::from(expanded)
}