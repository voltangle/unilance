#![no_std]

use core::error::Error;

// TODO: Add shit

#[derive(Debug, Copy, Clone)]
pub enum CoreLinkMessage {
    Heartbeat,
}

/// Link between two core modules, control and supervisor
#[allow(async_fn_in_trait)]
pub trait CoreLink {
    async fn core_recv(&mut self) -> CoreLinkMessage;
    // FIXME: No result type for return. For now it's like this because I couldn't be
    // bothered to figure it out yet, and for now it's only used for in memory channels,
    // which have only one reason to fail, so a timeout will catch something like this.
    async fn core_send(&mut self, msg: &CoreLinkMessage);
}

#[cfg(test)]
mod test {}
