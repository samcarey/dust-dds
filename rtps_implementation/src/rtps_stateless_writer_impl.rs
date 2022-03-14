use std::cell::RefCell;

use rtps_pim::{
    behavior::{
        stateless_writer_behavior::{
            BestEffortStatelessWriterBehavior, ReliableStatelessWriterBehavior,
        },
        types::Duration,
        writer::{
            reader_locator::RtpsReaderLocatorAttributes,
            stateless_writer::{
                RtpsStatelessWriterAttributes, RtpsStatelessWriterConstructor,
                RtpsStatelessWriterOperations,
            },
            writer::{RtpsWriterAttributes, RtpsWriterOperations},
        },
    },
    messages::{
        submessage_elements::Parameter,
        submessages::{DataSubmessage, GapSubmessage},
    },
    structure::{
        endpoint::RtpsEndpointAttributes,
        entity::RtpsEntityAttributes,
        types::{
            ChangeKind, Guid, InstanceHandle, Locator, ReliabilityKind, SequenceNumber, TopicKind,
        },
    },
};

use crate::rtps_reader_locator_impl::RtpsReaderLocatorOperationsImpl;

use super::{
    rtps_endpoint_impl::RtpsEndpointImpl,
    rtps_history_cache_impl::{RtpsCacheChangeImpl, RtpsHistoryCacheImpl},
    rtps_reader_locator_impl::RtpsReaderLocatorAttributesImpl,
    rtps_writer_impl::RtpsWriterImpl,
};

pub struct RtpsStatelessWriterImpl {
    pub writer: RtpsWriterImpl,
    pub reader_locators: Vec<RtpsReaderLocatorAttributesImpl>,
}

pub enum RtpsStatelessSubmessage<'a> {
    Data(DataSubmessage<Vec<Parameter<'a>>, &'a [u8]>),
    Gap(GapSubmessage<Vec<SequenceNumber>>),
}

impl RtpsStatelessWriterImpl {
    pub fn produce_destined_submessages<'a>(
        &'a mut self,
    ) -> Vec<(Locator, Vec<RtpsStatelessSubmessage<'a>>)> {
        let mut destined_submessages = Vec::new();

        for reader_locator in self.reader_locators.iter_mut() {
            match self.writer.endpoint.reliability_level {
                ReliabilityKind::BestEffort => {
                    let submessages = RefCell::new(Vec::new());
                    let writer_cache = &self.writer.writer_cache;
                    BestEffortStatelessWriterBehavior::send_unsent_changes(
                        &mut RtpsReaderLocatorOperationsImpl::new(reader_locator, writer_cache),
                        writer_cache,
                        |data| {
                            submessages
                                .borrow_mut()
                                .push(RtpsStatelessSubmessage::Data(data))
                        },
                        |gap| {
                            submessages
                                .borrow_mut()
                                .push(RtpsStatelessSubmessage::Gap(gap))
                        },
                    );

                    let submessages = submessages.take();
                    if !submessages.is_empty() {
                        destined_submessages.push((reader_locator.locator(), submessages));
                    }
                }

                ReliabilityKind::Reliable => {
                    let submessages = RefCell::new(Vec::new());
                    let writer_cache = &self.writer.writer_cache;
                    ReliableStatelessWriterBehavior::send_unsent_changes(
                        &mut RtpsReaderLocatorOperationsImpl::new(reader_locator, writer_cache),
                        writer_cache,
                        |data| {
                            submessages
                                .borrow_mut()
                                .push(RtpsStatelessSubmessage::Data(data))
                        },
                        |gap| {
                            submessages
                                .borrow_mut()
                                .push(RtpsStatelessSubmessage::Gap(gap))
                        },
                    );

                    let submessages = submessages.take();
                    if !submessages.is_empty() {
                        destined_submessages.push((reader_locator.locator(), submessages));
                    }
                }
            }
        }

        destined_submessages
    }
}

impl RtpsEntityAttributes for RtpsStatelessWriterImpl {
    fn guid(&self) -> Guid {
        self.writer.guid()
    }
}

impl RtpsEndpointAttributes for RtpsStatelessWriterImpl {
    fn topic_kind(&self) -> TopicKind {
        self.writer.endpoint.topic_kind
    }

    fn reliability_level(&self) -> ReliabilityKind {
        self.writer.endpoint.reliability_level
    }

    fn unicast_locator_list(&self) -> &[Locator] {
        &self.writer.endpoint.unicast_locator_list
    }

    fn multicast_locator_list(&self) -> &[Locator] {
        &self.writer.endpoint.multicast_locator_list
    }
}

impl RtpsWriterAttributes for RtpsStatelessWriterImpl {
    type HistoryCacheType = RtpsHistoryCacheImpl;

    fn push_mode(&self) -> bool {
        self.writer.push_mode
    }

    fn heartbeat_period(&self) -> Duration {
        self.writer.heartbeat_period
    }

    fn nack_response_delay(&self) -> Duration {
        self.writer.nack_response_delay
    }

    fn nack_suppression_duration(&self) -> Duration {
        self.writer.nack_suppression_duration
    }

    fn last_change_sequence_number(&self) -> SequenceNumber {
        self.writer.last_change_sequence_number
    }

    fn data_max_size_serialized(&self) -> Option<i32> {
        self.writer.data_max_size_serialized
    }

    fn writer_cache(&mut self) -> &mut Self::HistoryCacheType {
        &mut self.writer.writer_cache
    }
}

impl RtpsStatelessWriterAttributes for RtpsStatelessWriterImpl {
    type ReaderLocatorType = RtpsReaderLocatorAttributesImpl;

    fn reader_locators(&self) -> &[Self::ReaderLocatorType] {
        &self.reader_locators
    }
}

impl RtpsStatelessWriterConstructor for RtpsStatelessWriterImpl {
    fn new(
        guid: Guid,
        topic_kind: TopicKind,
        reliability_level: ReliabilityKind,
        unicast_locator_list: &[Locator],
        multicast_locator_list: &[Locator],
        push_mode: bool,
        heartbeat_period: Duration,
        nack_response_delay: Duration,
        nack_suppression_duration: Duration,
        data_max_size_serialized: Option<i32>,
    ) -> Self {
        Self {
            writer: RtpsWriterImpl::new(
                RtpsEndpointImpl::new(
                    guid,
                    topic_kind,
                    reliability_level,
                    unicast_locator_list,
                    multicast_locator_list,
                ),
                push_mode,
                heartbeat_period,
                nack_response_delay,
                nack_suppression_duration,
                data_max_size_serialized,
            ),
            reader_locators: Vec::new(),
        }
    }
}

impl RtpsStatelessWriterOperations for RtpsStatelessWriterImpl {
    type ReaderLocatorType = RtpsReaderLocatorAttributesImpl;

    fn reader_locator_add(&mut self, a_locator: Self::ReaderLocatorType) {
        self.reader_locators.push(a_locator);
    }

    fn reader_locator_remove<F>(&mut self, mut f: F)
    where
        F: FnMut(&Self::ReaderLocatorType) -> bool,
    {
        self.reader_locators.retain(|x| !f(x))
    }

    fn unsent_changes_reset(&mut self) {
        for reader_locator in &mut self.reader_locators {
            reader_locator.unsent_changes_reset()
        }
    }
}

impl RtpsWriterOperations for RtpsStatelessWriterImpl {
    type DataType = Vec<u8>;
    type ParameterListType = Vec<u8>;
    type CacheChangeType = RtpsCacheChangeImpl;
    fn new_change(
        &mut self,
        kind: ChangeKind,
        data: Self::DataType,
        _inline_qos: Self::ParameterListType,
        handle: InstanceHandle,
    ) -> Self::CacheChangeType {
        self.writer.new_change(kind, data, _inline_qos, handle)
    }
}

#[cfg(test)]
mod tests {
    use rtps_pim::{
        behavior::writer::reader_locator::RtpsReaderLocatorConstructor,
        messages::{
            submessage_elements::{
                EntityIdSubmessageElement, ParameterListSubmessageElement,
                SequenceNumberSubmessageElement, SerializedDataSubmessageElement,
            },
            types::ParameterId,
        },
        structure::{
            cache_change::{RtpsCacheChangeAttributes, RtpsCacheChangeConstructor},
            history_cache::RtpsHistoryCacheOperations,
            types::{
                EntityId, GuidPrefix, LOCATOR_KIND_UDPv4, ENTITYID_UNKNOWN,
                USER_DEFINED_WRITER_NO_KEY,
            },
        },
    };

    use crate::rtps_history_cache_impl::{RtpsData, RtpsParameter, RtpsParameterList};

    use super::*;

    #[test]
    fn produce_destined_submessages_one_locator_one_submessage() {
        let guid = Guid::new(
            GuidPrefix([0; 12]),
            EntityId::new([1, 2, 3], USER_DEFINED_WRITER_NO_KEY),
        );

        let mut writer = RtpsStatelessWriterImpl {
            writer: RtpsWriterImpl::new(
                RtpsEndpointImpl::new(
                    guid,
                    TopicKind::NoKey,
                    ReliabilityKind::BestEffort,
                    &[],
                    &[],
                ),
                false,
                Duration::new(0, 0),
                Duration::new(0, 0),
                Duration::new(0, 0),
                None,
            ),
            reader_locators: vec![RtpsReaderLocatorAttributesImpl::new(
                Locator::new(LOCATOR_KIND_UDPv4, 1234, [6; 16]),
                false,
            )],
        };

        let change = RtpsCacheChangeImpl::new(
            ChangeKind::Alive,
            guid,
            0,
            1,
            RtpsData(vec![4, 1, 3]),
            RtpsParameterList(vec![RtpsParameter {
                parameter_id: ParameterId(8),
                value: vec![6, 1, 2],
            }]),
        );

        writer.writer.writer_cache.add_change(change);

        let change = RtpsCacheChangeImpl::new(
            ChangeKind::Alive,
            guid,
            0,
            1,
            RtpsData(vec![4, 1, 3]),
            RtpsParameterList(vec![RtpsParameter {
                parameter_id: ParameterId(8),
                value: vec![6, 1, 2],
            }]),
        );

        let destined_submessages = writer.produce_destined_submessages();
        assert_eq!(1, destined_submessages.len());
        let (locator, submessages) = &destined_submessages[0];
        assert_eq!(&Locator::new(LOCATOR_KIND_UDPv4, 1234, [6; 16]), locator);
        assert_eq!(1, submessages.len());

        if let RtpsStatelessSubmessage::Data(data) = &submessages[0] {
            assert_eq!(true, data.endianness_flag);
            assert_eq!(
                &DataSubmessage {
                    endianness_flag: true,
                    inline_qos_flag: true,
                    data_flag: true,
                    key_flag: false,
                    non_standard_payload_flag: false,
                    reader_id: EntityIdSubmessageElement {
                        value: ENTITYID_UNKNOWN,
                    },
                    writer_id: EntityIdSubmessageElement {
                        value: change.writer_guid.entity_id,
                    },
                    writer_sn: SequenceNumberSubmessageElement {
                        value: change.sequence_number
                    },
                    inline_qos: ParameterListSubmessageElement {
                        parameter: change.inline_qos().into()
                    },
                    serialized_payload: SerializedDataSubmessageElement {
                        value: change.data_value().into()
                    }
                },
                data
            )
        }
    }
}
