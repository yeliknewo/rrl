use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};

pub type BothChannel<ID, T, F> = (DuoChannel<ID, T, F>, DuoChannel<ID, F, T>);

#[derive(Debug)]
pub struct DuoChannel<ID: Eq, S, R> {
    id: ID,
    send: Sender<S>,
    recv: Receiver<R>,
}

impl<ID: Eq, S, R> DuoChannel<ID, S, R> {
    pub fn new_both(id1: ID, id2: ID) -> BothChannel<ID, S, R> {
        let (send_to, recv_to) = channel();
        let (send_from, recv_from) = channel();

        (DuoChannel::new(id1, send_to, recv_from), DuoChannel::new(id2, send_from, recv_to))
    }

    fn new(id: ID, send: Sender<S>, recv: Receiver<R>) -> DuoChannel<ID, S, R> {
        DuoChannel {
            id: id,
            send: send,
            recv: recv,
        }
    }

    pub fn send(&mut self, event: S) {
        self.send.send(event).unwrap_or_else(|err| panic!("Failed to send because: {}", err));
    }

    pub fn recv(&mut self) -> R {
        self.recv.recv().unwrap_or_else(|err| panic!("Recv from error: {}", err))
    }

    pub fn try_recv(&mut self) -> Option<R> {
        match self.recv.try_recv() {
            Ok(event) => Some(event),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => panic!("Try recv from was disconnected"),
        }
    }

    pub fn get_id(&self) -> &ID {
        &self.id
    }
}
