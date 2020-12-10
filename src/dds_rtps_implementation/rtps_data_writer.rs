use crate::types::{DDSType, ReturnCode, InstanceHandle, Time, Duration};
use crate::dds_infrastructure::status::{LivelinessLostStatus, OfferedDeadlineMissedStatus, OfferedIncompatibleQosStatus, PublicationMatchedStatus};
use crate::dds_rtps_implementation::rtps_object::RtpsObjectReference;
use crate::builtin_topics::SubscriptionBuiltinTopicData;
pub struct RtpsDataWriterInner<T: DDSType>{
    marker: std::marker::PhantomData<T>
}

impl<T: DDSType> Default for RtpsDataWriterInner<T> {
    fn default() -> Self {
        Self {
            marker: std::marker::PhantomData
        }
    }
}

pub type RtpsDataWriter<'a, T> = RtpsObjectReference<'a, RtpsDataWriterInner<T>>;

impl<'a, T:DDSType> RtpsDataWriter<'a,T> {
    pub fn register_instance(
        &self,
        _instance: T
    ) -> ReturnCode<Option<InstanceHandle>> {
        todo!()
    }

    pub fn register_instance_w_timestamp(
        &self,
        _instance: T,
        _timestamp: Time,
    ) -> ReturnCode<Option<InstanceHandle>> {
        todo!()
    }

    pub fn unregister_instance(
        &self,
        _instance: T,
        _handle: Option<InstanceHandle>
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn unregister_instance_w_timestamp(
        &self,
        _instance: T,
        _handle: Option<InstanceHandle>,
        _timestamp: Time,
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_key_value(
        &self,
        _key_holder: &mut T,
        _handle: InstanceHandle
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn lookup_instance(
        &self,
        _instance: &T,
    ) -> ReturnCode<Option<InstanceHandle>> {
        todo!()
    }

    pub fn write (
        &self,
        _data: T,
        _handle: Option<InstanceHandle>,
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn write_w_timestamp(
        &self,
        _data: T,
        _handle: Option<InstanceHandle>,
        _timestamp: Time,
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn dispose(
        &self,
        _data: T,
        _handle: Option<InstanceHandle>,
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn dispose_w_timestamp(
        &self,
        _data: T,
        _handle: Option<InstanceHandle>,
        _timestamp: Time,
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn wait_for_acknowledgments(
        &self,
        _max_wait: Duration
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_liveliness_lost_status(
        &self,
        _status: &mut LivelinessLostStatus
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_offered_deadline_missed_status(
        &self,
        _status: &mut OfferedDeadlineMissedStatus
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_offered_incompatible_qos_status(
        &self,
        _status: &mut OfferedIncompatibleQosStatus
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_publication_matched_status(
        &self,
        _status: &mut PublicationMatchedStatus
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn assert_liveliness(&self) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_matched_subscription_data(
        &self,
        _subscription_data: SubscriptionBuiltinTopicData,
        _subscription_handle: InstanceHandle,
    ) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_matched_subscriptions(
        &self,
        _subscription_handles: &mut [InstanceHandle],
    ) -> ReturnCode<()> {
        todo!()
    }
}
