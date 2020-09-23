use std::sync::Weak;
use std::marker::PhantomData;

use crate::types::{ReturnCode, SampleStateKind, ViewStateKind, InstanceStateKind, InstanceHandle};
use crate::subscription::read_condition::ReadCondition;
use crate::subscription::query_condition::QueryCondition;
use crate::infrastructure::status::{LivelinessChangedStatus, RequestedDeadlineMissedStatus, RequestedIncompatibleQosStatus, SampleLostStatus, SampleRejectedStatus, SubscriptionMatchedStatus};
use crate::topic::topic_description::TopicDescription;
use crate::subscription::subscriber::Subscriber;
use crate::subscription::sample_info::SampleInfo;
use crate::subscription::data_reader_listener::DataReaderListener;
use crate::builtin_topics::PublicationBuiltinTopicData;

use crate::implementation::subscriber_impl::SubscriberImpl;

use rust_dds_interface::qos::DataReaderQos;

pub(crate) struct DataReaderImpl<T>{
    parent_subscriber: Weak<SubscriberImpl>,
    value: PhantomData<T>,
}

impl<T> DataReaderImpl<T> {
    pub fn read(
        _this: &Weak<DataReaderImpl<T>>,
        _data_values: &mut [T],
        _sample_infos: &mut [SampleInfo],
        _max_samples: i32,
        _sample_states: &[SampleStateKind],
        _view_states: &[ViewStateKind],
        _instance_states: &[InstanceStateKind],
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn take(
        _this: &Weak<DataReaderImpl<T>>,
        _data_values: &mut [T],
        _sample_infos: &mut [SampleInfo],
        _max_samples: i32,
        _sample_states: &[SampleStateKind],
        _view_states: &[ViewStateKind],
        _instance_states: &[InstanceStateKind],
    ) -> ReturnCode<()> {
        todo!()
    }
    
    pub fn read_w_condition(
        _this: &Weak<DataReaderImpl<T>>,
        _data_values: &mut [T],
        _sample_infos: &mut [SampleInfo],
        _max_samples: i32,
        _a_condition: ReadCondition,
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn take_w_condition(
        _this: &Weak<DataReaderImpl<T>>,
        _data_values: &mut [T],
        _sample_infos: &mut [SampleInfo],
        _max_samples: i32,
        _a_condition: ReadCondition,
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn read_next_sample(
        _this: &Weak<DataReaderImpl<T>>,
        _data_value: &mut [T],
        _sample_info: &mut [SampleInfo],
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn take_next_sample(
        _this: &Weak<DataReaderImpl<T>>,
        _data_value: &mut [T],
        _sample_info: &mut [SampleInfo],
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn read_instance(
        _this: &Weak<DataReaderImpl<T>>,
        _data_values: &mut [T],
        _sample_infos: &mut [SampleInfo],
        _max_samples: i32,
        _a_handle: InstanceHandle,
        _sample_states: &[SampleStateKind],
        _view_states: &[ViewStateKind],
        _instance_states: &[InstanceStateKind],
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn take_instance(
        _this: &Weak<DataReaderImpl<T>>,
        _data_values: &mut [T],
        _sample_infos: &mut [SampleInfo],
        _max_samples: i32,
        _a_handle: InstanceHandle,
        _sample_states: &[SampleStateKind],
        _view_states: &[ViewStateKind],
        _instance_states: &[InstanceStateKind],
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn read_next_instance(
        _this: &Weak<DataReaderImpl<T>>,
        _data_values: &mut [T],
        _sample_infos: &mut [SampleInfo],
        _max_samples: i32,
        _previous_handle: InstanceHandle,
        _sample_states: &[SampleStateKind],
        _view_states: &[ViewStateKind],
        _instance_states: &[InstanceStateKind],
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn take_next_instance(
        _this: &Weak<DataReaderImpl<T>>,
        _data_values: &mut [T],
        _sample_infos: &mut [SampleInfo],
        _max_samples: i32,
        _previous_handle: InstanceHandle,
        _sample_states: &[SampleStateKind],
        _view_states: &[ViewStateKind],
        _instance_states: &[InstanceStateKind],
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn read_next_instance_w_condition(
        _this: &Weak<DataReaderImpl<T>>,
        _data_values: &mut [T],
        _sample_infos: &mut [SampleInfo],
        _max_samples: i32,
        _previous_handle: InstanceHandle,
        _a_condition: ReadCondition,
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn take_next_instance_w_condition(
        _this: &Weak<DataReaderImpl<T>>,
        _data_values: &mut [T],
        _sample_infos: &mut [SampleInfo],
        _max_samples: i32,
        _previous_handle: InstanceHandle,
        _a_condition: ReadCondition,
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn return_loan(
        _this: &Weak<DataReaderImpl<T>>,
        _data_values: &mut [T],
        _sample_infos: &mut [SampleInfo],
     ) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_key_value(
        _this: &Weak<DataReaderImpl<T>>,
        _key_holder: &mut T,
        _handle: InstanceHandle
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn lookup_instance(
        _this: &Weak<DataReaderImpl<T>>,
        _instance: &T,
    ) -> InstanceHandle {
        todo!()
    }

    pub fn create_readcondition(
        _this: &Weak<DataReaderImpl<T>>,
        _sample_states: &[SampleStateKind],
        _view_states: &[ViewStateKind],
        _instance_states: &[InstanceStateKind],
    ) -> ReadCondition {
        todo!()
    }

    pub fn create_querycondition(
        _this: &Weak<DataReaderImpl<T>>,
        _sample_states: &[SampleStateKind],
        _view_states: &[ViewStateKind],
        _instance_states: &[InstanceStateKind],
        _query_expression: String,
        _query_parameters: &[String],
    ) -> QueryCondition {
        todo!()
    }

    pub fn delete_readcondition(
        _this: &Weak<DataReaderImpl<T>>,
        _a_condition: ReadCondition
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_liveliness_changed_status(
        _this: &Weak<DataReaderImpl<T>>,
        _status: &mut LivelinessChangedStatus
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_requested_deadline_missed_status(
        _this: &Weak<DataReaderImpl<T>>,
        _status: &mut RequestedDeadlineMissedStatus
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_requested_incompatible_qos_status(
        _this: &Weak<DataReaderImpl<T>>,
        _status: &mut RequestedIncompatibleQosStatus
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_sample_lost_status(
        _this: &Weak<DataReaderImpl<T>>,
        _status: &mut SampleLostStatus
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_sample_rejected_status(
        _this: &Weak<DataReaderImpl<T>>,
        _status: &mut SampleRejectedStatus
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_subscription_matched_status(
        _this: &Weak<DataReaderImpl<T>>,
        _status: &mut SubscriptionMatchedStatus
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_topicdescription(
        _this: &Weak<DataReaderImpl<T>>
    ) -> &dyn TopicDescription {
        todo!()
    }

    pub fn get_subscriber(
        _this: &Weak<DataReaderImpl<T>>,
    ) -> Subscriber {
        todo!()
    }

    pub fn delete_contained_entities(
        _this: &Weak<DataReaderImpl<T>>,
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn wait_for_historical_data(
        _this: &Weak<DataReaderImpl<T>>,
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_matched_publication_data(
        _this: &Weak<DataReaderImpl<T>>,
        _publication_data: &mut PublicationBuiltinTopicData,
        _publication_handle: InstanceHandle,
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_match_publication(
        _this: &Weak<DataReaderImpl<T>>,
        _publication_handles: &mut [InstanceHandle],
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn set_qos(
        _this: &Weak<DataReaderImpl<T>>,
        _qos_list: DataReaderQos
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_qos(
        _this: &Weak<DataReaderImpl<T>>,
        _qos_list: &mut DataReaderQos
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn set_listener(
        _this: &Weak<DataReaderImpl<T>>,
        _a_listener: Box<dyn DataReaderListener<T>>, _mask: &[crate::types::StatusKind]
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_listener(
        _this: &Weak<DataReaderImpl<T>>,
    ) -> Box<dyn DataReaderListener<T>> {
        todo!()
    }

    pub fn get_statuscondition(
        _this: &Weak<DataReaderImpl<T>>,
    ) -> crate::infrastructure::entity::StatusCondition {
        todo!()
    }

    pub fn get_status_changes(
        _this: &Weak<DataReaderImpl<T>>,
    ) -> crate::types::StatusKind {
        todo!()
    }

    pub fn enable(
        _this: &Weak<DataReaderImpl<T>>,
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_instance_handle(
        _this: &Weak<DataReaderImpl<T>>,
    ) -> InstanceHandle {
        todo!()
    }

    //////////////// From here on are the functions that do not belong to the standard API
    pub(crate) fn new(parent_subscriber: Weak<SubscriberImpl>
    ) -> Self {
        Self{
            parent_subscriber,
            value: PhantomData
        }
    }
}