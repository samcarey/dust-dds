use crate::structure::{RtpsEntity, RtpsCommunication, RtpsMessageSender};
use crate::types::{Locator, ReliabilityKind, TopicKind};

pub trait RtpsEndpoint : RtpsEntity + RtpsCommunication + RtpsMessageSender {
    fn unicast_locator_list(&self) -> Vec<Locator>;
    fn multicast_locator_list(&self) -> Vec<Locator>;
    fn reliability_level(&self) -> ReliabilityKind;
    fn topic_kind(&self) -> &TopicKind;
}