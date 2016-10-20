use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};

pub type TwoWayChannel<T, F> = (FrontChannel<T, F>, BackChannel<T, F>);

pub fn two_way_channel<T, F>() -> TwoWayChannel<T, F> {
    let (send_to, recv_to) = channel();
    let (send_from, recv_from) = channel();

    (FrontChannel::new(send_to, recv_from), BackChannel::new(send_from, recv_to))
}

#[derive(Debug)]
pub struct FrontChannel<T, F> {
    send_to: Sender<T>,
    recv_from: Receiver<F>,
}

impl<T, F> FrontChannel<T, F> {
    fn new(send_to: Sender<T>, recv_from: Receiver<F>) -> FrontChannel<T, F> {
        FrontChannel {
            send_to: send_to,
            recv_from: recv_from,
        }
    }

    pub fn send_to(&mut self, event: T) {
        self.send_to.send(event).unwrap_or_else(|err| panic!("failed to send to because: {}", err));
    }

    pub fn recv_from(&mut self) -> F {
        self.recv_from.recv().unwrap_or_else(|err| panic!("recv from error: {}", err))
    }

    pub fn try_recv_from(&mut self) -> Option<F> {
        match self.recv_from.try_recv() {
            Ok(event) => Some(event),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => panic!("try recv from was disconnected"),
        }
    }
}

#[derive(Debug)]
pub struct BackChannel<T, F> {
    send_from: Sender<F>,
    recv_to: Receiver<T>,
}

impl<T, F> BackChannel<T, F> {
    fn new(send_from: Sender<F>, recv_to: Receiver<T>) -> BackChannel<T, F> {
        BackChannel {
            send_from: send_from,
            recv_to: recv_to,
        }
    }

    pub fn send_from(&mut self, event: F) {
        self.send_from
            .send(event)
            .unwrap_or_else(|err| panic!("failed to send from because: {}", err));
    }

    pub fn recv_to(&mut self) -> T {
        self.recv_to.recv().unwrap_or_else(|err| panic!("recv to error: {}", err))
    }

    pub fn try_recv_to(&mut self) -> Option<T> {
        match self.recv_to.try_recv() {
            Ok(event) => Some(event),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => panic!("try recv to was disconnected"),
        }
    }
}
