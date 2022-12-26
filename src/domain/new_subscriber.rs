//! src/domain/new_subscribe.rs

use super::SubscriberName;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
