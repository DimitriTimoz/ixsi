use crate::prelude::*;

pub struct IxsiHandler {
    my_peer_id: PeerId,
    remote_peer_id: PeerId,
}

impl ConnectionHandler for IxsiHandler {
    type FromBehaviour;

    type ToBehaviour;

    type Error;

    type InboundProtocol;

    type OutboundProtocol;

    type InboundOpenInfo;

    type OutboundOpenInfo;

    fn listen_protocol(&self) -> libp2p::swarm::SubstreamProtocol<Self::InboundProtocol, Self::InboundOpenInfo> {
        todo!()
    }

    fn connection_keep_alive(&self) -> libp2p::swarm::KeepAlive {
        todo!()
    }

    fn poll(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<
        libp2p::swarm::ConnectionHandlerEvent<
            Self::OutboundProtocol,
            Self::OutboundOpenInfo,
            Self::ToBehaviour,
            Self::Error,
        >,
    > {
        todo!()
    }

    fn on_behaviour_event(&mut self, _event: Self::FromBehaviour) {
        todo!()
    }

    fn on_connection_event(
        &mut self,
        event: libp2p::swarm::handler::ConnectionEvent<
            Self::InboundProtocol,
            Self::OutboundProtocol,
            Self::InboundOpenInfo,
            Self::OutboundOpenInfo,
        >,
    ) {
        todo!()
    }
}

impl IxsiHandler {
    pub fn new(my_peer_id: PeerId, remote_peer_id: PeerId) -> Self {
        Self {
            my_peer_id,
            remote_peer_id,
        }
    }
}