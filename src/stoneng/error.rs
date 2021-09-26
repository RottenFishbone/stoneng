#[derive(Debug)]
pub enum EngineError {
    ShaderCompile(String),
    ShaderLink(String),
    RendererInit(String),
    IOError(std::io::Error),
    SheetParseError(ron::error::Error),
    SheetSizeError(String),
}

impl From<ron::error::Error> for EngineError {
    fn from(error: ron::error::Error) -> Self {
        Self::SheetParseError(error)
    }
}

impl From<std::io::Error> for EngineError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}
