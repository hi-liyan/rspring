/// Web 控制器 trait
pub trait Controller: axum_boot_core::Component {
    /// 获取控制器的基础路径
    fn base_path(&self) -> &'static str;
}