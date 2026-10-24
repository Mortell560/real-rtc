use webrtc::peer_connection::{
    PeerConnection, PeerConnectionBuilder, PeerConnectionEventHandler,
    RTCConfigurationBuilder, RTCIceServer, RTCPeerConnectionIceEvent,
};
use std::sync::Arc;

// 1. Implement the PeerConnectionEventHandler trait to handle events
#[derive(Clone)]
struct MyHandler;

#[async_trait::async_trait]
impl PeerConnectionEventHandler for MyHandler {
    async fn on_ice_candidate(&self, event: RTCPeerConnectionIceEvent) {
        println!("New local ICE candidate gathered: {}", event.candidate);
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 2. Configure the peer connection
    let config = RTCConfigurationBuilder::default()
        .with_ice_servers(vec![RTCIceServer {
            urls: vec![
                "stun:stun.l.google.com:19302".to_owned(),
                "stun:stun1.l.google.com:19302".to_owned(),
                "stun:stun2.l.google.com:19302".to_owned(),
                "stun:stun3.l.google.com:19302".to_owned(),
                "stun:stun4.l.google.com:19302".to_owned()
            ],
            ..Default::default()
        }])
        .build();

    // 3. Build the PeerConnection
    let pc = PeerConnectionBuilder::new()
        .with_configuration(config)
        .with_handler(Arc::new(MyHandler))
        .with_udp_addrs(vec!["0.0.0.0:0"])
        .build()
        .await?;

    // 4. Create an SDP offer and set it as local description
    let offer = pc.create_offer(None).await?;
    pc.set_local_description(offer).await?;
     
    println!("Local description set successfully!");
    Ok(())
}
