#[repr(u32)]
pub enum StringName {
    Vendor = 0x1F00,
    Renderer = 0x1F01,
    Version = 0x1F02,
    Extensions = 0x1F03,
    ShadingLanguageVersion = 0x8B8C,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    FragmentShader = 0x8B30,
    VertexShader = 0x8B31,
}