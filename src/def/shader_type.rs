#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    FragmentShader = 0x8B30,
    VertexShader = 0x8B31,
}