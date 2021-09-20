#[derive(Debug)]
pub enum EngineError {
    ShaderCompile(String),
    ShaderLink(String),
    RendererInit(String),
}

