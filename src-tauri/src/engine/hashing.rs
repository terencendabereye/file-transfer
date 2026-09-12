use tokio::io::AsyncReadExt;

use super::CHUNK_SIZE;

/// Hashes a whole file up front so the manifest sent ahead of the chunk stream carries
/// the final BLAKE3 digest for end-to-end verification on the receiving side.
pub async fn hash_file(path: &std::path::Path) -> std::io::Result<[u8; 32]> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut hasher = blake3::Hasher::new();
    let mut buf = vec![0u8; CHUNK_SIZE];
    loop {
        let n = file.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(*hasher.finalize().as_bytes())
}
