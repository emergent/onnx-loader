use anyhow::Result;
use prost::Message;
use std::path::Path;

include!(concat!(env!("OUT_DIR"), "/onnx.rs"));

pub fn load_onnx<P: AsRef<Path>>(path: P) -> Result<ModelProto> {
    let data = std::fs::read(path)?;
    let model = ModelProto::decode(&*data)?;
    Ok(model)
}
