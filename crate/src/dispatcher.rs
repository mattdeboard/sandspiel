use pub_sub::PubSub;
use species::Species;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Event {
    pub x: i32,
    pub y: i32,
    pub size: i32,
    pub species: Species,
}

pub struct Dispatcher<E: Clone> {
    event_channel: PubSub<E>,
    event_queue: VecDeque<E>,
}

pub trait HandlesEvents {
    fn handle_event(&mut self, event: Event);
}

impl Dispatcher<Event> {
    pub fn new() -> Dispatcher<Event> {
        Dispatcher {
            event_channel: PubSub::<Event>::new(),
            event_queue: VecDeque::new(),
        }
    }
}

pub trait Dispatch {
    fn add_event(&mut self, event: Event);
    fn get_at_offset(&self, offset: usize) -> Option<&Event>;
}

impl Dispatch for Dispatcher<Event> {
    fn add_event(&mut self, event: Event) {
        self.event_channel.send(event);
    }
    fn get_at_offset(&self, offset: usize) -> Option<&Event> {
        self.event_queue.get(offset)
    }
}
