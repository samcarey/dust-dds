use crate::structure::RtpsEntity;
use crate::types::{ReliabilityKind, GUID};
use rust_dds_api::types::TopicKind;

pub struct RtpsEndpoint {
    pub entity: RtpsEntity,
    pub topic_kind: TopicKind,
    pub reliability_level: ReliabilityKind,
}

impl RtpsEndpoint {
    pub fn new(guid: GUID, topic_kind: TopicKind, reliability_level: ReliabilityKind) -> Self {
        let entity = RtpsEntity::new(guid);
        Self {
            entity,
            topic_kind,
            reliability_level,
        }
    }
}