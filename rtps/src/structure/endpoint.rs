use crate::structure::{RtpsEntity, RtpsRun, RtpsCommunication, RtpsMessageSender};
use crate::types::{Locator,GuidPrefix, ReliabilityKind, TopicKind};
use crate::messages::RtpsSubmessage;

pub trait RtpsEndpoint : RtpsEntity + RtpsRun + RtpsCommunication + RtpsMessageSender {
    fn unicast_locator_list(&self) -> Vec<Locator>;
    fn multicast_locator_list(&self) -> Vec<Locator>;
    fn reliability_level(&self) -> ReliabilityKind;
    fn topic_kind(&self) -> &TopicKind;
}