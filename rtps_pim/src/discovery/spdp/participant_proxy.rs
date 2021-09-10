use crate::{
    discovery::types::{BuiltinEndpointQos, BuiltinEndpointSet, DomainId},
    messages::types::Count,
    structure::types::{GuidPrefix, ProtocolVersion, VendorId},
};

#[derive(Debug, PartialEq)]
pub struct ParticipantProxy<S, L> {
    pub domain_id: DomainId,
    pub domain_tag: S,
    pub protocol_version: ProtocolVersion,
    pub guid_prefix: GuidPrefix,
    pub vendor_id: VendorId,
    pub expects_inline_qos: bool,
    pub metatraffic_unicast_locator_list: L,
    pub metatraffic_multicast_locator_list: L,
    pub default_unicast_locator_list: L,
    pub default_multicast_locator_list: L,
    pub available_builtin_endpoints: BuiltinEndpointSet,
    pub manual_liveliness_count: Count,
    pub builtin_endpoint_qos: BuiltinEndpointQos,
}