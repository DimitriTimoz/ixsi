use crate::prelude::*;

#[derive(Debug)]
pub enum IxisEvent {
    
}

pub struct IxisBehavior<T> {
    my_peer_id: PeerId,
    db: Arc<Db<T>>,
}

impl<T> IxisBehavior<T> {
    pub fn new(my_peer_id: PeerId, tastes: T) -> Self {
        Self {
            my_peer_id,
            db: Arc::new(Db::new(IxsiConfig::new(), tastes)),
        }
    }
}

impl<T> NetworkBehaviour for IxisBehavior<T>{
    type ConnectionHandler = IxsiHandler;
    type ToSwarm = IxisBehavior<T>;

    fn handle_established_inbound_connection(
        &mut self,
        _connection_id: ConnectionId,
        peer: PeerId,
        local_addr: &libp2p::Multiaddr,
        remote_addr: &libp2p::Multiaddr,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        Ok(IxsiHandler::new(self.my_peer_id, peer))
    }

    fn handle_established_outbound_connection(
        &mut self,
        _connection_id: ConnectionId,
        remote_addr: PeerId,
        addr: &libp2p::Multiaddr,
        role_override: libp2p::core::Endpoint,
    ) -> Result<libp2p::swarm::THandler<Self>, ConnectionDenied> {
        Ok(IxsiHandler::new(self.my_peer_id, remote_addr))
    }

    fn on_swarm_event(&mut self, event: FromSwarm<Self::ConnectionHandler>) {
        match event {
            FromSwarm::ConnectionEstablished(connection) => {

            },
            FromSwarm::ConnectionClosed(_) => todo!(),
            FromSwarm::AddressChange(_) => todo!(),
            FromSwarm::DialFailure(_) => todo!(),
            FromSwarm::ListenFailure(_) => todo!(),
            FromSwarm::NewListener(_) => todo!(),
            FromSwarm::NewListenAddr(_) => todo!(),
            FromSwarm::ExpiredListenAddr(_) => todo!(),
            FromSwarm::ListenerError(_) => todo!(),
            FromSwarm::ListenerClosed(_) => todo!(),
            FromSwarm::NewExternalAddrCandidate(_) => todo!(),
            FromSwarm::ExternalAddrConfirmed(_) => todo!(),
            FromSwarm::ExternalAddrExpired(_) => todo!(),
        }
    }

    fn on_connection_handler_event(
        &mut self,
        _peer_id: PeerId,
        _connection_id: libp2p::swarm::ConnectionId,
        _event: libp2p::swarm::THandlerOutEvent<Self>,
    ) {
        todo!()
    }

    fn poll(
        &mut self,
        cx: &mut std::task::Context<'_>,
        params: &mut impl libp2p::swarm::PollParameters,
    ) -> std::task::Poll<libp2p::swarm::ToSwarm<Self::ToSwarm, libp2p::swarm::THandlerInEvent<Self>>> {
        todo!()
    }


}