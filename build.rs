use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/proto")
        .compile(
            &[
                "milvus-proto/proto/common.proto",
                "milvus-proto/proto/milvus.proto",
                "milvus-proto/proto/schema.proto",
            ],
            &["milvus-proto/proto"],
        )?;
    
    let generated_file = Path::new("src/proto/milvus.proto.milvus.rs");
    if generated_file.exists() {
        let content = fs::read_to_string(generated_file)?;
        // Rename only the RPC connect method (the one with ConnectRequest parameter)
        // to avoid conflict with the gRPC client connect method
        let modified = content
            .replace(
                "pub async fn connect(\n            &mut self,\n            request: impl tonic::IntoRequest<super::ConnectRequest>,",
                "pub async fn connect_rpc(\n            &mut self,\n            request: impl tonic::IntoRequest<super::ConnectRequest>,"
            );
        fs::write(generated_file, modified)?;
    }
    
    Ok(())
}
