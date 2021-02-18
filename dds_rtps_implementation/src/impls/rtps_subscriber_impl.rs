use std::sync::Mutex;

use rust_dds_api::{
    dcps_psm::StatusMask,
    infrastructure::qos::{DataReaderQos, SubscriberQos},
    subscription::subscriber_listener::SubscriberListener,
};
use rust_rtps::structure::Group;

pub struct RtpsSubscriberImpl {
    group: Group,
    default_datareader_qos: DataReaderQos,
    qos: Mutex<SubscriberQos>,
    listener: Option<Box<dyn SubscriberListener>>,
    status_mask: StatusMask,
}

impl RtpsSubscriberImpl {
    pub fn new(
        group: Group,
        qos: SubscriberQos,
        listener: Option<Box<dyn SubscriberListener>>,
        status_mask: StatusMask,
    ) -> Self {
        Self {
            group,
            default_datareader_qos: DataReaderQos::default(),
            qos: Mutex::new(qos),
            listener,
            status_mask,
        }
    }
}

// enum EntityType {
//     BuiltIn,
//     UserDefined,
// }
// pub struct RtpsSubscriberInner {
//     group: Group,
//     entity_type: EntityType,
//     reader_list: MaybeValidList<Mutex<RtpsDataReaderFlavor>>,
//     reader_count: atomic::AtomicU8,
//     default_datareader_qos: Mutex<DataReaderQos>,
//     qos: Mutex<SubscriberQos>,
//     listener: Option<Box<dyn SubscriberListener>>,
//     status_mask: StatusMask,
// }

// impl RtpsSubscriberInner {
//     pub fn new_builtin(
//         guid_prefix: GuidPrefix,
//         entity_key: [u8; 3],
//         qos: SubscriberQos,
//         listener: Option<Box<dyn SubscriberListener>>,
//         status_mask: StatusMask,
//     ) -> Self {
//         Self::new(
//             guid_prefix,
//             entity_key,
//             qos,
//             listener,
//             status_mask,
//             EntityType::BuiltIn,
//         )
//     }

//     pub fn new_user_defined(
//         guid_prefix: GuidPrefix,
//         entity_key: [u8; 3],
//         qos: SubscriberQos,
//         listener: Option<Box<dyn SubscriberListener>>,
//         status_mask: StatusMask,
//     ) -> Self {
//         Self::new(
//             guid_prefix,
//             entity_key,
//             qos,
//             listener,
//             status_mask,
//             EntityType::UserDefined,
//         )
//     }

//     fn new(
//         guid_prefix: GuidPrefix,
//         entity_key: [u8; 3],
//         qos: SubscriberQos,
//         listener: Option<Box<dyn SubscriberListener>>,
//         status_mask: StatusMask,
//         entity_type: EntityType,
//     ) -> Self {
//         let entity_id = match entity_type {
//             EntityType::BuiltIn => EntityId::new(entity_key, ENTITY_KIND_BUILT_IN_READER_GROUP),
//             EntityType::UserDefined => {
//                 EntityId::new(entity_key, ENTITY_KIND_USER_DEFINED_READER_GROUP)
//             }
//         };
//         let guid = GUID::new(guid_prefix, entity_id);

//         Self {
//             group: Group::new(guid),
//             entity_type,
//             reader_list: Default::default(),
//             reader_count: atomic::AtomicU8::new(0),
//             default_datareader_qos: Mutex::new(DataReaderQos::default()),
//             qos: Mutex::new(qos),
//             listener,
//             status_mask,
//         }
//     }
// }

// pub type RtpsSubscriberInnerRef<'a> = MaybeValidRef<'a, Box<RtpsSubscriberInner>>;

// impl<'a> RtpsSubscriberInnerRef<'a> {
//     pub fn get(&self) -> DDSResult<&Box<RtpsSubscriberInner>> {
//         MaybeValid::get(self).ok_or(DDSError::AlreadyDeleted)
//     }

//     pub fn delete(&self) -> DDSResult<()> {
//         if self.get()?.reader_list.is_empty() {
//             MaybeValid::delete(self);
//             Ok(())
//         } else {
//             Err(DDSError::PreconditionNotMet(
//                 "Subscriber still contains data readers",
//             ))
//         }
//     }

//     pub fn create_datareader<T: DDSType>(
//         &self,
//         a_topic: &RtpsTopicInnerRef,
//         qos: Option<DataReaderQos>,
//         a_listener: Option<Box<dyn DataReaderListener<DataType = T>>>,
//         status_mask: StatusMask,
//     ) -> Option<RtpsAnyDataReaderInnerRef> {
//         let this = self.get().ok()?;
//         let topic = a_topic.get().ok()?;
//         let entity_key = [
//             0,
//             self.get()
//                 .ok()?
//                 .reader_count
//                 .fetch_add(1, atomic::Ordering::Relaxed),
//             0,
//         ];
//         let guid_prefix = this.group.entity.guid.prefix();
//         let qos = qos.unwrap_or(this.default_datareader_qos.lock().unwrap().clone());

//         let data_reader_inner = RtpsStatefulDataReaderInner::new_user_defined(
//             guid_prefix,
//             entity_key,
//             vec![],
//             vec![],
//             topic,
//             qos,
//             a_listener,
//             status_mask,
//         );

//         self.get()
//             .ok()?
//             .reader_list
//             .add(Mutex::new(RtpsDataReaderFlavor::Stateful(
//                 data_reader_inner,
//             )))
//     }

//     pub fn get_qos(&self) -> DDSResult<SubscriberQos> {
//         Ok(self.get()?.qos.lock().unwrap().clone())
//     }

//     pub fn get_default_datareader_qos(&self) -> DDSResult<DataReaderQos> {
//         Ok(self.get()?.default_datareader_qos.lock().unwrap().clone())
//     }

//     pub fn set_default_datareader_qos(&self, qos: Option<DataReaderQos>) -> DDSResult<()> {
//         let datareader_qos = qos.unwrap_or_default();
//         datareader_qos.is_consistent()?;
//         *self.get()?.default_datareader_qos.lock().unwrap() = datareader_qos;
//         Ok(())
//     }
// }

// pub fn create_builtin_datareader<T: DDSType>(
//     &self,
//     a_topic: Arc<dyn RtpsAnyTopicInner>,
//     qos: Option<DataReaderQos>,
//     // _a_listener: impl DataReaderListener<T>,
//     // _mask: StatusMask
// ) -> Option<RtpsAnyDataReaderRef> {
//     self.create_datareader::<T>(a_topic, qos, EntityType::BuiltIn)
// }

// pub fn create_user_defined_datareader<T: DDSType>(
//     &self,
//     a_topic: Arc<dyn RtpsAnyTopicInner>,
//     qos: Option<DataReaderQos>,
//     // _a_listener: impl DataReaderListener<T>,
//     // _mask: StatusMask
// ) -> Option<RtpsAnyDataReaderRef> {
//     self.create_datareader::<T>(a_topic, qos, EntityType::BuiltIn)
// }

// fn create_datareader<T: DDSType>(
//     &self,
//     a_topic: Arc<dyn RtpsAnyTopicInner>,
//     qos: Option<DataReaderQos>,
//     entity_type: EntityType,
//     // _a_listener: impl DataReaderListener<T>,
//     // _mask: StatusMask
// ) -> Option<RtpsAnyDataReaderRef> {
//     let guid_prefix = self.group.entity.guid.prefix();
//     let entity_key = [
//         0,
//         self.reader_count.fetch_add(1, atomic::Ordering::Relaxed),
//         0,
//     ];
//     let entity_kind = match (a_topic.topic_kind(), entity_type) {
//         (TopicKind::WithKey, EntityType::UserDefined) => {
//             ENTITY_KIND_USER_DEFINED_READER_WITH_KEY
//         }
//         (TopicKind::NoKey, EntityType::UserDefined) => ENTITY_KIND_USER_DEFINED_READER_NO_KEY,
//         (TopicKind::WithKey, EntityType::BuiltIn) => ENTITY_KIND_BUILT_IN_READER_WITH_KEY,
//         (TopicKind::NoKey, EntityType::BuiltIn) => ENTITY_KIND_BUILT_IN_READER_NO_KEY,
//     };
//     let entity_id = EntityId::new(entity_key, entity_kind);
//     let new_reader_guid = GUID::new(guid_prefix, entity_id);
//     let new_reader_qos = qos.unwrap_or(self.get_default_datareader_qos());
//     let new_reader: Box<RtpsDataReaderInner<T>> = Box::new(RtpsDataReaderInner::new(
//         new_reader_guid,
//         a_topic,
//         new_reader_qos,
//         None,
//         0,
//     ));
//     self.reader_list.add(new_reader)
// }
