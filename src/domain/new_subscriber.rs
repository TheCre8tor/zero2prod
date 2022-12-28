//! src/domain/new_subscribe.rs

use crate::routes::FormData;

use super::{SubscriberEmail, SubscriberName};

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<NewSubscriber, Self::Error> {
        let email = SubscriberEmail::parse(value.email)?;
        let name = SubscriberName::parse(value.name)?;

        Ok(Self { email, name })
    }
}
