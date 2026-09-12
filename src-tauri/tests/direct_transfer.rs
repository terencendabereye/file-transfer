//! Exercises the real sender/receiver over an actual TCP loopback connection —
//! milestone 2's "first real P2P bytes moved" goal, verified without needing two GUI
//! processes.

use file_transfer_lib::engine::receiver::receive_one;
use file_transfer_lib::engine::sender::send_file;
use file_transfer_lib::net::framing::new_framed;
use file_transfer_lib::net::transport::Transport;
use tokio::net::{TcpListener, TcpStream};
use uuid::Uuid;

#[tokio::test]
async fn sends_and_receives_a_file_over_loopback_tcp() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let src_dir = tempfile::tempdir().unwrap();
    let src_path = src_dir.path().join("payload.bin");
    let payload: Vec<u8> = (0..3_500_000u32).map(|i| (i % 251) as u8).collect();
    tokio::fs::write(&src_path, &payload).await.unwrap();

    let dest_dir = tempfile::tempdir().unwrap();
    let dest_dir_path = dest_dir.path().to_path_buf();

    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut framed = new_framed(Transport::Plain(stream));
        receive_one(&mut framed, &dest_dir_path, |_, _, _| {}).await.unwrap()
    });

    let stream = TcpStream::connect(addr).await.unwrap();
    let mut framed = new_framed(Transport::Plain(stream));
    let transfer_id = Uuid::new_v4();
    let mut last_progress = (0u64, 0u64);
    let success = send_file(&mut framed, transfer_id, &src_path, |done, total| {
        last_progress = (done, total);
    })
    .await
    .unwrap();

    assert!(success, "sender should observe the receiver confirming the hash matched");
    assert_eq!(last_progress, (payload.len() as u64, payload.len() as u64));

    let received = server.await.unwrap().expect("receiver should have gotten a file");
    assert_eq!(received.0, transfer_id);
    assert!(received.2, "receiver should report a hash match");

    let received_bytes = tokio::fs::read(&received.1).await.unwrap();
    assert_eq!(received_bytes, payload);
}
