use rtps_pim::{
    behavior::{
        types::Duration,
        writer::{RtpsWriterAttributes, RtpsWriterOperations},
    },
    structure::{
        cache_change::RtpsCacheChangeConstructor,
        endpoint::RtpsEndpointAttributes,
        entity::RtpsEntityAttributes,
        history_cache::RtpsHistoryCacheConstructor,
        types::{
            ChangeKind, Guid, InstanceHandle, Locator, ReliabilityKind, SequenceNumber, TopicKind,
        },
    },
};

use super::{
    rtps_endpoint_impl::RtpsEndpointImpl,
    rtps_history_cache_impl::{RtpsCacheChangeImpl, RtpsHistoryCacheImpl},
};

pub struct RtpsWriterImpl {
    endpoint: RtpsEndpointImpl,
    push_mode: bool,
    heartbeat_period: Duration,
    nack_response_delay: Duration,
    nack_suppression_duration: Duration,
    last_change_sequence_number: SequenceNumber,
    data_max_size_serialized: Option<i32>,
    pub writer_cache: RtpsHistoryCacheImpl,
}

impl RtpsWriterImpl {
    pub fn new(
        endpoint: RtpsEndpointImpl,
        push_mode: bool,
        heartbeat_period: Duration,
        nack_response_delay: Duration,
        nack_suppression_duration: Duration,
        data_max_size_serialized: Option<i32>,
    ) -> Self {
        Self {
            endpoint,
            push_mode,
            heartbeat_period,
            nack_response_delay,
            nack_suppression_duration,
            last_change_sequence_number: 0,
            data_max_size_serialized,
            writer_cache: RtpsHistoryCacheImpl::new(),
        }
    }
}

impl RtpsEntityAttributes for RtpsWriterImpl {
    fn guid(&self) -> Guid {
        self.endpoint.guid()
    }
}

impl RtpsEndpointAttributes for RtpsWriterImpl {
    fn topic_kind(&self) -> TopicKind {
        self.endpoint.topic_kind()
    }

    fn reliability_level(&self) -> ReliabilityKind {
        self.endpoint.reliability_level()
    }

    fn unicast_locator_list(&self) -> &[Locator] {
        self.endpoint.unicast_locator_list()
    }

    fn multicast_locator_list(&self) -> &[Locator] {
        self.endpoint.multicast_locator_list()
    }
}

impl RtpsWriterAttributes for RtpsWriterImpl {
    type HistoryCacheType = RtpsHistoryCacheImpl;

    fn push_mode(&self) -> bool {
        self.push_mode
    }

    fn heartbeat_period(&self) -> Duration {
        self.heartbeat_period
    }

    fn nack_response_delay(&self) -> Duration {
        self.nack_response_delay
    }

    fn nack_suppression_duration(&self) -> Duration {
        self.nack_suppression_duration
    }

    fn last_change_sequence_number(&self) -> SequenceNumber {
        self.last_change_sequence_number
    }

    fn data_max_size_serialized(&self) -> Option<i32> {
        self.data_max_size_serialized
    }

    fn writer_cache(&mut self) -> &mut Self::HistoryCacheType {
        &mut self.writer_cache
    }
}

impl RtpsWriterOperations for RtpsWriterImpl {
    type CacheChangeType = RtpsCacheChangeImpl;
    fn new_change(
        &mut self,
        kind: ChangeKind,
        data: <Self::CacheChangeType as RtpsCacheChangeConstructor>::DataType,
        inline_qos: <Self::CacheChangeType as RtpsCacheChangeConstructor>::ParameterListType,
        handle: InstanceHandle,
    ) -> Self::CacheChangeType {
        self.last_change_sequence_number += 1;
        RtpsCacheChangeImpl::new(
            kind,
            self.guid(),
            handle,
            self.last_change_sequence_number,
            data,
            inline_qos,
        )
    }
}
