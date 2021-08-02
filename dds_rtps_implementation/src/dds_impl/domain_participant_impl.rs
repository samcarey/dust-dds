use std::{
    sync::{
        atomic::{self, AtomicBool},
        Arc,
    },
    thread::JoinHandle,
};

use rust_dds_api::{
    builtin_topics::{ParticipantBuiltinTopicData, TopicBuiltinTopicData},
    dcps_psm::{DomainId, Duration, InstanceHandle, StatusMask, Time},
    domain::domain_participant_listener::DomainParticipantListener,
    infrastructure::{
        entity::{Entity, StatusCondition},
        qos::{DomainParticipantQos, PublisherQos, SubscriberQos, TopicQos},
    },
    publication::{publisher::Publisher, publisher_listener::PublisherListener},
    return_type::{DDSError, DDSResult},
    subscription::subscriber_listener::SubscriberListener,
    topic::{topic_description::TopicDescription, topic_listener::TopicListener},
};
use rust_rtps_pim::structure::{
    types::{EntityId, EntityKind, Guid},
    RtpsEntity,
};

use crate::{
    rtps_impl::{rtps_group_impl::RtpsGroupImpl, rtps_participant_impl::RtpsParticipantImpl},
    utils::shared_object::RtpsShared,
};

use super::{
    publisher_impl::{PublisherImpl, PublisherStorage},
    subscriber_impl::SubscriberImpl,
    subscriber_storage::SubscriberStorage,
    topic_impl::TopicImpl,
};

pub struct DomainParticipantStorage {
    rtps_participant: RtpsParticipantImpl,
    domain_participant_qos: DomainParticipantQos,
    builtin_subscriber_storage: Vec<RtpsShared<SubscriberStorage>>,
    builtin_publisher_storage: Vec<RtpsShared<PublisherStorage>>,
    user_defined_subscriber_storage: Vec<RtpsShared<SubscriberStorage>>,
    user_defined_subscriber_counter: u8,
    default_subscriber_qos: SubscriberQos,
    user_defined_publisher_storage: Vec<RtpsShared<PublisherStorage>>,
    user_defined_publisher_counter: u8,
    default_publisher_qos: PublisherQos,
}

impl DomainParticipantStorage {
    pub fn new(
        domain_participant_qos: DomainParticipantQos,
        rtps_participant: RtpsParticipantImpl,
        builtin_subscriber_storage: Vec<RtpsShared<SubscriberStorage>>,
        builtin_publisher_storage: Vec<RtpsShared<PublisherStorage>>,
    ) -> Self {
        Self {
            rtps_participant,
            domain_participant_qos,
            builtin_subscriber_storage,
            builtin_publisher_storage,
            user_defined_subscriber_storage: Vec::new(),
            user_defined_subscriber_counter: 0,
            default_subscriber_qos: SubscriberQos::default(),
            user_defined_publisher_storage: Vec::new(),
            user_defined_publisher_counter: 0,
            default_publisher_qos: PublisherQos::default(),
        }
    }

    /// Get a reference to the domain participant storage's builtin subscriber storage.
    pub fn builtin_subscriber_storage(&self) -> &[RtpsShared<SubscriberStorage>] {
        &self.builtin_subscriber_storage
    }

    /// Get a reference to the domain participant storage's rtps participant.
    pub fn rtps_participant(&self) -> &RtpsParticipantImpl {
        &self.rtps_participant
    }

    /// Get a reference to the domain participant storage's builtin publisher storage.
    pub fn builtin_publisher_storage(&self) -> &[RtpsShared<PublisherStorage>] {
        &self.builtin_publisher_storage
    }

    /// Get a reference to the domain participant storage's user defined subscriber storage.
    pub fn user_defined_subscriber_storage(&self) -> &[RtpsShared<SubscriberStorage>] {
        self.user_defined_subscriber_storage.as_slice()
    }

    /// Get a reference to the domain participant storage's user defined publisher storage.
    pub fn user_defined_publisher_storage(&self) -> &[RtpsShared<PublisherStorage>] {
        self.user_defined_publisher_storage.as_slice()
    }
}

pub struct DomainParticipantImpl {
    is_enabled: Arc<AtomicBool>,
    domain_participant_storage: RtpsShared<DomainParticipantStorage>,
    worker_threads: Vec<JoinHandle<()>>,
}

impl DomainParticipantImpl {
    pub fn new(
        is_enabled: Arc<AtomicBool>,
        domain_participant_storage: RtpsShared<DomainParticipantStorage>,
        worker_threads: Vec<JoinHandle<()>>,
    ) -> Self {
        Self {
            is_enabled,
            domain_participant_storage,
            worker_threads,
        }
    }
}

impl<'p> rust_dds_api::domain::domain_participant::PublisherFactory<'p> for DomainParticipantImpl {
    type PublisherType = PublisherImpl<'p>;
    fn create_publisher(
        &'p self,
        qos: Option<PublisherQos>,
        _a_listener: Option<&'static dyn PublisherListener>,
        _mask: StatusMask,
    ) -> Option<Self::PublisherType> {
        let mut domain_participant_lock = self.domain_participant_storage.lock();
        let publisher_qos = qos.unwrap_or(domain_participant_lock.default_publisher_qos.clone());
        domain_participant_lock.user_defined_publisher_counter += 1;
        let entity_id = EntityId::new(
            [domain_participant_lock.user_defined_publisher_counter, 0, 0],
            EntityKind::UserDefinedWriterGroup,
        );
        let guid = Guid::new(
            *domain_participant_lock.rtps_participant.guid().prefix(),
            entity_id,
        );
        let rtps_group = RtpsGroupImpl::new(guid);
        let data_writer_storage_list = Vec::new();
        let publisher_storage =
            PublisherStorage::new(publisher_qos, rtps_group, data_writer_storage_list);
        let publisher_storage_shared = RtpsShared::new(publisher_storage);
        let publisher = PublisherImpl::new(self, &publisher_storage_shared);
        domain_participant_lock
            .user_defined_publisher_storage
            .push(publisher_storage_shared);
        Some(publisher)
    }

    fn delete_publisher(&self, a_publisher: &Self::PublisherType) -> DDSResult<()> {
        if std::ptr::eq(a_publisher.get_participant(), self) {
            todo!()
            // self.rtps_participant_impl
            //     .lock()
            //     .delete_writer_group(a_publisher.get_instance_handle()?)
        } else {
            Err(DDSError::PreconditionNotMet(
                "Publisher can only be deleted from its parent participant",
            ))
        }
    }
}

impl<'s> rust_dds_api::domain::domain_participant::SubscriberFactory<'s> for DomainParticipantImpl {
    type SubscriberType = SubscriberImpl<'s>;

    fn create_subscriber(
        &'s self,
        _qos: Option<SubscriberQos>,
        _a_listener: Option<&'static dyn SubscriberListener>,
        _mask: StatusMask,
    ) -> Option<Self::SubscriberType> {
        todo!()
        //         // let impl_ref = self
        //         //     .0
        //         //     .lock()
        //         //     .unwrap()
        //         //     .create_subscriber(qos, a_listener, mask)
        //         //     .ok()?;

        //         // Some(Subscriber(Node {
        //         //     parent: self,
        //         //     impl_ref,
        //         // }))
    }

    fn delete_subscriber(&self, _a_subscriber: &Self::SubscriberType) -> DDSResult<()> {
        todo!()
        //         // if std::ptr::eq(a_subscriber.parent, self) {
        //         //     self.0
        //         //         .lock()
        //         //         .unwrap()
        //         //         .delete_subscriber(&a_subscriber.impl_ref)
        //         // } else {
        //         //     Err(DDSError::PreconditionNotMet(
        //         //         "Subscriber can only be deleted from its parent participant",
        //         //     ))
        //         // }
    }

    fn get_builtin_subscriber(&'s self) -> Self::SubscriberType {
        todo!()
        //         //     self.builtin_entities
        //         //         .subscriber_list()
        //         //         .into_iter()
        //         //         .find(|x| {
        //         //             if let Some(subscriber) = x.get().ok() {
        //         //                 subscriber.group.entity.guid.entity_id().entity_kind()
        //         //                     == ENTITY_KIND_BUILT_IN_READER_GROUP
        //         //             } else {
        //         //                 false
        //         //             }
        //         //         })
        //         // }
    }
}

impl<'t, T: 'static> rust_dds_api::domain::domain_participant::TopicFactory<'t, T>
    for DomainParticipantImpl
{
    type TopicType = TopicImpl<'t, T>;

    fn create_topic(
        &'t self,
        _topic_name: &str,
        _qos: Option<TopicQos>,
        _a_listener: Option<&'static dyn TopicListener<DataPIM = T>>,
        _mask: StatusMask,
    ) -> Option<Self::TopicType> {
        todo!()
    }

    fn delete_topic(&self, _a_topic: &Self::TopicType) -> DDSResult<()> {
        todo!()
    }

    fn find_topic(&self, _topic_name: &str, _timeout: Duration) -> Option<Self::TopicType> {
        todo!()
    }
}

impl rust_dds_api::domain::domain_participant::DomainParticipant for DomainParticipantImpl {
    fn lookup_topicdescription<'t, T>(
        &'t self,
        _name: &'t str,
    ) -> Option<&'t (dyn TopicDescription<T> + 't)> {
        todo!()
    }

    fn ignore_participant(&self, _handle: InstanceHandle) -> DDSResult<()> {
        todo!()
    }

    fn ignore_topic(&self, _handle: InstanceHandle) -> DDSResult<()> {
        todo!()
    }

    fn ignore_publication(&self, _handle: InstanceHandle) -> DDSResult<()> {
        todo!()
    }

    fn ignore_subscription(&self, _handle: InstanceHandle) -> DDSResult<()> {
        todo!()
    }

    fn get_domain_id(&self) -> DomainId {
        // self.domain_id
        todo!()
    }

    fn delete_contained_entities(&self) -> DDSResult<()> {
        todo!()
    }

    fn assert_liveliness(&self) -> DDSResult<()> {
        todo!()
    }

    fn set_default_publisher_qos(&self, qos: Option<PublisherQos>) -> DDSResult<()> {
        self.domain_participant_storage.lock().default_publisher_qos = qos.unwrap_or_default();
        Ok(())
    }

    fn get_default_publisher_qos(&self) -> PublisherQos {
        self.domain_participant_storage
            .lock()
            .default_publisher_qos
            .clone()
    }

    fn set_default_subscriber_qos(&self, qos: Option<SubscriberQos>) -> DDSResult<()> {
        self.domain_participant_storage
            .lock()
            .default_subscriber_qos = qos.unwrap_or_default();
        Ok(())
    }

    fn get_default_subscriber_qos(&self) -> SubscriberQos {
        self.domain_participant_storage
            .lock()
            .default_subscriber_qos
            .clone()
    }

    fn set_default_topic_qos(&self, qos: Option<TopicQos>) -> DDSResult<()> {
        let topic_qos = qos.unwrap_or_default();
        topic_qos.is_consistent()?;
        // *self.default_topic_qos.lock().unwrap() = topic_qos;
        Ok(())
    }

    fn get_default_topic_qos(&self) -> TopicQos {
        // self.default_topic_qos.lock().unwrap().clone()
        todo!()
    }

    fn get_discovered_participants(
        &self,
        _participant_handles: &mut [InstanceHandle],
    ) -> DDSResult<()> {
        todo!()
    }

    fn get_discovered_participant_data(
        &self,
        _participant_data: ParticipantBuiltinTopicData,
        _participant_handle: InstanceHandle,
    ) -> DDSResult<()> {
        todo!()
    }

    fn get_discovered_topics(&self, _topic_handles: &mut [InstanceHandle]) -> DDSResult<()> {
        todo!()
    }

    fn get_discovered_topic_data(
        &self,
        _topic_data: TopicBuiltinTopicData,
        _topic_handle: InstanceHandle,
    ) -> DDSResult<()> {
        todo!()
    }

    fn contains_entity(&self, _a_handle: InstanceHandle) -> bool {
        todo!()
    }

    fn get_current_time(&self) -> DDSResult<Time> {
        todo!()
    }
}

impl Entity for DomainParticipantImpl {
    type Qos = DomainParticipantQos;
    type Listener = &'static dyn DomainParticipantListener;

    fn set_qos(&self, qos: Option<Self::Qos>) -> DDSResult<()> {
        self.domain_participant_storage
            .lock()
            .domain_participant_qos = qos.unwrap_or_default();
        Ok(())
    }

    fn get_qos(&self) -> DDSResult<Self::Qos> {
        Ok(self
            .domain_participant_storage
            .lock()
            .domain_participant_qos
            .clone())
    }

    fn set_listener(
        &self,
        _a_listener: Option<Self::Listener>,
        _mask: StatusMask,
    ) -> DDSResult<()> {
        todo!()
    }

    fn get_listener(&self) -> DDSResult<Option<Self::Listener>> {
        todo!()
    }

    fn get_statuscondition(&self) -> StatusCondition {
        todo!()
    }

    fn get_status_changes(&self) -> StatusMask {
        todo!()
    }

    fn get_instance_handle(&self) -> DDSResult<InstanceHandle> {
        todo!()
        // Ok(crate::utils::instance_handle_from_guid(
        //     &self.rtps_participant_impl.lock().guid(),
        // ))
    }

    fn enable(&self) -> DDSResult<()> {
        self.is_enabled.store(true, atomic::Ordering::Release);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rust_dds_api::domain::domain_participant::DomainParticipant;

    // struct MockDDSType;

    #[test]
    fn set_default_publisher_qos_some_value() {
        let rtps_participant = RtpsParticipantImpl::new([1; 12]);
        let domain_participant_storage = DomainParticipantStorage::new(
            DomainParticipantQos::default(),
            rtps_participant,
            vec![],
            vec![],
        );
        let domain_participant_impl = DomainParticipantImpl::new(
            Arc::new(AtomicBool::new(false)),
            RtpsShared::new(domain_participant_storage),
            vec![],
        );
        let mut qos = PublisherQos::default();
        qos.group_data.value = &[1, 2, 3, 4];
        domain_participant_impl
            .set_default_publisher_qos(Some(qos.clone()))
            .unwrap();
        assert!(domain_participant_impl.get_default_publisher_qos() == qos);
    }

    #[test]
    fn set_default_publisher_qos_none() {
        let rtps_participant = RtpsParticipantImpl::new([1; 12]);
        let domain_participant_storage = DomainParticipantStorage::new(
            DomainParticipantQos::default(),
            rtps_participant,
            vec![],
            vec![],
        );
        let domain_participant_impl = DomainParticipantImpl::new(
            Arc::new(AtomicBool::new(false)),
            RtpsShared::new(domain_participant_storage),
            vec![],
        );
        let mut qos = PublisherQos::default();
        qos.group_data.value = &[1, 2, 3, 4];
        domain_participant_impl
            .set_default_publisher_qos(Some(qos.clone()))
            .unwrap();

        domain_participant_impl
            .set_default_publisher_qos(None)
            .unwrap();
        assert!(domain_participant_impl.get_default_publisher_qos() == PublisherQos::default());
    }

    #[test]
    fn set_default_subscriber_qos_some_value() {
        let rtps_participant = RtpsParticipantImpl::new([1; 12]);
        let domain_participant_storage = DomainParticipantStorage::new(
            DomainParticipantQos::default(),
            rtps_participant,
            vec![],
            vec![],
        );
        let domain_participant_impl = DomainParticipantImpl::new(
            Arc::new(AtomicBool::new(false)),
            RtpsShared::new(domain_participant_storage),
            vec![],
        );
        let mut qos = SubscriberQos::default();
        qos.group_data.value = &[1, 2, 3, 4];
        domain_participant_impl
            .set_default_subscriber_qos(Some(qos.clone()))
            .unwrap();
        assert_eq!(domain_participant_impl.get_default_subscriber_qos(), qos);
    }

    #[test]
    fn set_default_subscriber_qos_none() {
        let rtps_participant = RtpsParticipantImpl::new([1; 12]);
        let domain_participant_storage = DomainParticipantStorage::new(
            DomainParticipantQos::default(),
            rtps_participant,
            vec![],
            vec![],
        );
        let domain_participant_impl = DomainParticipantImpl::new(
            Arc::new(AtomicBool::new(false)),
            RtpsShared::new(domain_participant_storage),
            vec![],
        );
        let mut qos = SubscriberQos::default();
        qos.group_data.value = &[1, 2, 3, 4];
        domain_participant_impl
            .set_default_subscriber_qos(Some(qos.clone()))
            .unwrap();

        domain_participant_impl
            .set_default_subscriber_qos(None)
            .unwrap();
        assert_eq!(
            domain_participant_impl.get_default_subscriber_qos(),
            SubscriberQos::default()
        );
    }

    // #[test]
    // fn get_default_subscriber_qos() {
    //     let domain_participant_impl: DomainParticipantImpl<RtpsUdpPsm> =
    //         DomainParticipantImpl::new(RTPSParticipantImpl::new([1; 12]));
    //     let mut qos = SubscriberQos::default();
    //     qos.group_data.value = &[1, 2, 3, 4];
    //     domain_participant_impl
    //         .set_default_subscriber_qos(Some(qos.clone()))
    //         .unwrap();
    //     assert!(domain_participant_impl.get_default_subscriber_qos() == qos);
    // }

    // #[test]
    // fn set_default_topic_qos_some_value() {
    //     let domain_participant_impl: DomainParticipantImpl<RtpsUdpPsm> =
    //         DomainParticipantImpl::new(RTPSParticipantImpl::new([1; 12]));
    //     let mut qos = TopicQos::default();
    //     qos.topic_data.value = &[1, 2, 3, 4];
    //     domain_participant_impl
    //         .set_default_topic_qos(Some(qos.clone()))
    //         .unwrap();
    //     assert!(*domain_participant_impl.default_topic_qos.lock().unwrap() == qos);
    // }

    // #[test]
    // fn set_default_topic_qos_inconsistent() {
    //     let domain_participant_impl: DomainParticipantImpl<RtpsUdpPsm> =
    //         DomainParticipantImpl::new(RTPSParticipantImpl::new([1; 12]));
    //     let mut qos = TopicQos::default();
    //     qos.resource_limits.max_samples_per_instance = 2;
    //     qos.resource_limits.max_samples = 1;
    //     let set_default_topic_qos_result =
    //         domain_participant_impl.set_default_topic_qos(Some(qos.clone()));
    //     assert!(set_default_topic_qos_result == Err(DDSError::InconsistentPolicy));
    // }

    // #[test]
    // fn set_default_topic_qos_none() {
    //     let domain_participant_impl: DomainParticipantImpl<RtpsUdpPsm> =
    //         DomainParticipantImpl::new(RTPSParticipantImpl::new([1; 12]));
    //     let mut qos = TopicQos::default();
    //     qos.topic_data.value = &[1, 2, 3, 4];
    //     domain_participant_impl
    //         .set_default_topic_qos(Some(qos.clone()))
    //         .unwrap();

    //     domain_participant_impl.set_default_topic_qos(None).unwrap();
    //     assert!(*domain_participant_impl.default_topic_qos.lock().unwrap() == TopicQos::default());
    // }

    // #[test]
    // fn get_default_topic_qos() {
    //     let domain_participant_impl: DomainParticipantImpl<RtpsUdpPsm> =
    //         DomainParticipantImpl::new(RTPSParticipantImpl::new([1; 12]));
    //     let mut qos = TopicQos::default();
    //     qos.topic_data.value = &[1, 2, 3, 4];
    //     domain_participant_impl
    //         .set_default_topic_qos(Some(qos.clone()))
    //         .unwrap();
    //     assert!(domain_participant_impl.get_default_topic_qos() == qos);
    // }

    // #[test]
    // fn create_publisher() {
    //     let domain_participant_impl: DomainParticipantImpl<RtpsUdpPsm> =
    //         DomainParticipantImpl::new([1; 12]);
    //     let publisher = domain_participant_impl.create_publisher(None, None, 0);

    //     assert!(publisher.is_some())
    // }

    // #[test]
    // fn create_topic() {
    //     let domain_participant_impl: DomainParticipantImpl<RtpsUdpPsm> =
    //         DomainParticipantImpl::new([1; 12]);
    //     let topic =
    //         domain_participant_impl.create_topic::<MockDDSType>("topic_name", None, None, 0);
    //     assert!(topic.is_some());
    // }
}
