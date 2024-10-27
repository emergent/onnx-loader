use onnx_loader::load_onnx;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("models/mnist-opt.onnx");
    println!("{:?}", path);
    let model = load_onnx(path)?;

    println!("{:#?}", model);

    Ok(())
}
