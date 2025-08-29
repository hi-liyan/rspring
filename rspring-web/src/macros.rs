use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemStruct};

/// REST 控制器注解
/// 
/// 标记一个结构体为 REST 控制器，会自动注册到路由中
/// 
/// # 示例
/// 
/// ```rust
/// #[derive(RestController)]
/// #[RequestMapping("/api/users")]
/// pub struct UserController {
///     user_service: Arc<UserService>,
/// }
/// ```
#[proc_macro_derive(RestController, attributes(RequestMapping))]
pub fn rest_controller_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // TODO: 解析 RequestMapping 属性获取路由前缀
    
    let expanded = quote! {
        impl rspring_core::Component for #name {
            fn component_name(&self) -> &'static str {
                stringify!(#name)
            }
        }
        
        impl crate::Controller for #name {
            fn base_path(&self) -> &'static str {
                // TODO: 从 RequestMapping 属性中提取路径
                "/"
            }
        }
    };

    TokenStream::from(expanded)
}

/// 请求映射注解（用作属性）
/// 
/// 定义控制器的基础路由路径
/// 
/// # 示例
/// 
/// ```rust
/// #[derive(RestController)]
/// #[RequestMapping("/api/users")]
/// pub struct UserController;
/// ```
#[proc_macro_attribute]
pub fn RequestMapping(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    
    // TODO: 解析路径参数并存储到结构体中
    // 现在简单地保持原结构体不变
    
    let expanded = quote! {
        #input
    };

    TokenStream::from(expanded)
}

/// GET 请求映射注解
/// 
/// 标记方法处理 GET 请求
/// 
/// # 示例
/// 
/// ```rust
/// #[GetMapping("/{id}")]
/// pub async fn get_user(&self, id: u64) -> Result<ApiResponse<User>> {
///     // 处理逻辑
/// }
/// ```
#[proc_macro_attribute]
pub fn GetMapping(_args: TokenStream, input: TokenStream) -> TokenStream {
    // TODO: 实现路由方法生成
    // 现在简单地保持原方法不变
    input
}

/// POST 请求映射注解
#[proc_macro_attribute] 
pub fn PostMapping(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// PUT 请求映射注解
#[proc_macro_attribute]
pub fn PutMapping(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// DELETE 请求映射注解
#[proc_macro_attribute]
pub fn DeleteMapping(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// PATCH 请求映射注解
#[proc_macro_attribute]
pub fn PatchMapping(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// 路径变量注解
#[proc_macro_attribute]
pub fn PathVariable(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// 请求体注解
#[proc_macro_attribute]
pub fn RequestBody(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// 请求参数注解
#[proc_macro_attribute]
pub fn RequestParam(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// 请求头注解
#[proc_macro_attribute]
pub fn RequestHeader(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}