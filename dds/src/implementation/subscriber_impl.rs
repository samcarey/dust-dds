use std::sync::{Arc, Weak, Mutex};
use std::any::Any;

use crate::types::{
    ReturnCode,
    StatusKind,
    SampleStateKind,
    ViewStateKind,
    InstanceStateKind,
    StatusMask,
    ReturnCodes
};

use crate::infrastructure::status::SampleLostStatus;
use crate::domain::domain_participant::DomainParticipant;
use crate::topic::topic_description::TopicDescription;
use crate::subscription::data_reader::{DataReader, AnyDataReader};
use crate::subscription::data_reader_listener::DataReaderListener;
use crate::subscription::subscriber_listener::SubscriberListener;

use crate::implementation::domain_participant_impl::DomainParticipantImpl;
use crate::implementation::data_reader_impl::DataReaderImpl;

use rust_dds_interface::qos::{TopicQos, SubscriberQos, DataReaderQos};

pub struct SubscriberImpl{
    parent_participant: Weak<DomainParticipantImpl>,
    datareader_list: Mutex<Vec<AnyDataReader>>,
    default_datareader_qos: Mutex<DataReaderQos>,
}

impl SubscriberImpl {
    pub(crate) fn create_datareader<T: Any+Send+Sync>(
        this: &Weak<SubscriberImpl>,
        _a_topic: &dyn TopicDescription,
        _qos: DataReaderQos,
        _a_listener: Box<dyn DataReaderListener<T>>,
        _mask: StatusMask
    ) -> Option<DataReader<T>> {
        let datareader_impl = Arc::new(DataReaderImpl::new(this.clone()));
        let datareader = DataReader(Arc::downgrade(&datareader_impl));        

        this.upgrade()?.datareader_list.lock().ok()?.push(AnyDataReader(datareader_impl));

        Some(datareader)
    }

    pub(crate) fn delete_datareader<T: Any+Send+Sync>(
        this: &Weak<SubscriberImpl>,
        a_datareader: &DataReader<T>
    ) -> ReturnCode<()> {
        let subscriber = this.upgrade().unwrap();
        let mut datareader_list = subscriber.datareader_list.lock().unwrap();
        let index = datareader_list.iter().position(|x| 
            match x.get::<T>() {
                Some(dr) => dr.0.ptr_eq(&a_datareader.0),
                None => false,
        });
        
        if let Some(index) = index{
            datareader_list.swap_remove(index);
            Ok(())
        } else {
            Err(ReturnCodes::PreconditionNotMet)
        }
    }

    pub(crate) fn lookup_datareader<T>(
        _this: &Weak<SubscriberImpl>,
        _topic_name: String
    ) -> DataReader<T> {
        todo!()
    }

    pub(crate) fn begin_access(
        _this: &Weak<SubscriberImpl>,
    ) -> ReturnCode<()> {
        todo!()
    }

    pub(crate) fn end_access(
        _this: &Weak<SubscriberImpl>,
    ) -> ReturnCode<()> {
        todo!()
    }

   
    pub(crate) fn get_datareaders<T>(
        _this: &Weak<SubscriberImpl>,
        _readers: &mut [DataReader<T>],
        _sample_states: &[SampleStateKind],
        _view_states: &[ViewStateKind],
        _instance_states: &[InstanceStateKind],
    ) -> ReturnCode<()> {
        todo!()
    }

    pub(crate) fn notify_datareaders(
        _this: &Weak<SubscriberImpl>,
    ) -> ReturnCode<()> {
        todo!()
    }

    pub(crate) fn get_sample_lost_status(
        _this: &Weak<SubscriberImpl>,
        _status: &mut SampleLostStatus) -> ReturnCode<()> {
        todo!()
    }

    pub(crate) fn get_participant(
        this: &Weak<SubscriberImpl>,
    ) -> DomainParticipant {
        DomainParticipant(this.upgrade().unwrap().parent_participant.upgrade().unwrap())
    }

    pub(crate) fn delete_contained_entities(
        _this: &Weak<SubscriberImpl>,
    ) -> ReturnCode<()> {
        todo!()
    }

    pub(crate) fn set_default_datareader_qos(
        this: &Weak<SubscriberImpl>,
        qos: DataReaderQos,
    ) -> ReturnCode<()> {
        let subscriber = SubscriberImpl::upgrade_subscriber(this)?;

        if qos.is_consistent() {
            *subscriber.default_datareader_qos.lock().unwrap() = qos;
        } else {
            return Err(ReturnCodes::InconsistentPolicy);
        }
        
        Ok(())
    }

    pub(crate) fn get_default_datareader_qos(
        this: &Weak<SubscriberImpl>,
        qos: &mut DataReaderQos,
    ) -> ReturnCode<()> {
        let subscriber = SubscriberImpl:: upgrade_subscriber(this)?;

        qos.clone_from(&subscriber.default_datareader_qos.lock().unwrap());
        Ok(())
    }

    pub(crate) fn copy_from_topic_qos(
        _this: &Weak<SubscriberImpl>,
        _a_datareader_qos: &mut DataReaderQos,
        _a_topic_qos: &TopicQos,
    ) -> ReturnCode<()> {
        todo!()
    }

    //////////////// Entity trait methods
    pub(crate) fn set_qos(_this: &Weak<SubscriberImpl>, _qos_list: SubscriberQos) -> ReturnCode<()> {
        todo!()
    }

    pub(crate) fn get_qos(_this: &Weak<SubscriberImpl>, _qos_list: &mut SubscriberQos) -> ReturnCode<()> {
        todo!()
    }

    pub(crate) fn set_listener(_this: &Weak<SubscriberImpl>, _a_listener: Box<dyn SubscriberListener>, _mask: &[StatusKind]) -> ReturnCode<()> {
        todo!()
    }

    pub(crate) fn get_listener(_this: &Weak<SubscriberImpl>) -> Box<dyn SubscriberListener> {
        todo!()
    }

    pub(crate) fn get_statuscondition(_this: &Weak<SubscriberImpl>) -> crate::infrastructure::entity::StatusCondition {
        todo!()
    }

    pub(crate) fn get_status_changes(_this: &Weak<SubscriberImpl>) -> StatusKind {
        todo!()
    }

    pub(crate) fn enable(_this: &Weak<SubscriberImpl>) -> ReturnCode<()> {
        todo!()
    }

    pub(crate) fn get_instance_handle(_this: &Weak<SubscriberImpl>) -> crate::types::InstanceHandle {
        todo!()
    }

    //////////////// From here on are the functions that do not belong to the standard API
    pub(crate) fn new(parent_participant: Weak<DomainParticipantImpl>
    ) -> Self {
        Self{
            parent_participant,
            datareader_list: Mutex::new(Vec::new()),
            default_datareader_qos: Mutex::new(DataReaderQos::default()),
        }
    }

    fn upgrade_subscriber(this: &Weak<SubscriberImpl>) -> ReturnCode<Arc<SubscriberImpl>> {
        this.upgrade().ok_or(ReturnCodes::AlreadyDeleted)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::listener::NoListener;
    use crate::topic::topic::Topic;
    use rust_dds_interface::qos_policy::ReliabilityQosPolicyKind;
    #[derive(Debug)]
    struct  Foo {
        value: bool
    }

    #[test]
    fn create_delete_datareader() {
        let subscriber_impl = Arc::new(SubscriberImpl::new(Weak::new()));
        let topic = Topic(Weak::new());
        
        assert_eq!(subscriber_impl.datareader_list.lock().unwrap().len(), 0);
        let datareader = SubscriberImpl::create_datareader::<Foo>(&Arc::downgrade(&subscriber_impl),&topic, DataReaderQos::default(), Box::new(NoListener), 0).unwrap();
        assert_eq!(subscriber_impl.datareader_list.lock().unwrap().len(), 1);
        
        SubscriberImpl::delete_datareader(&Arc::downgrade(&subscriber_impl), &datareader).unwrap();
        assert_eq!(subscriber_impl.datareader_list.lock().unwrap().len(), 0);
    }

    #[test]
    fn set_and_get_default_datareader_qos() {
        let subscriber_impl = Arc::new(SubscriberImpl::new(Weak::new()));
        let subscriber = Arc::downgrade(&subscriber_impl);

        let mut datareader_qos = DataReaderQos::default();
        datareader_qos.user_data.value = vec![1,2,3,4];
        datareader_qos.reliability.kind = ReliabilityQosPolicyKind::ReliableReliabilityQos;

        SubscriberImpl::set_default_datareader_qos(&subscriber, datareader_qos.clone()).unwrap();
        assert_eq!(*subscriber_impl.default_datareader_qos.lock().unwrap(), datareader_qos);

        let mut read_datareader_qos = DataReaderQos::default();
        SubscriberImpl::get_default_datareader_qos(&subscriber, &mut read_datareader_qos).unwrap();

        assert_eq!(read_datareader_qos, datareader_qos);
    }

    #[test]
    fn inconsistent_datareader_qos() {
        let subscriber_impl = Arc::new(SubscriberImpl::new(Weak::new()));
        let subscriber = Arc::downgrade(&subscriber_impl);

        let mut datareader_qos = DataReaderQos::default();
        datareader_qos.resource_limits.max_samples = 5;
        datareader_qos.resource_limits.max_samples_per_instance = 15;

        let error = SubscriberImpl::set_default_datareader_qos(&subscriber, datareader_qos.clone());
        assert_eq!(error, Err(ReturnCodes::InconsistentPolicy));

        assert_eq!(*subscriber_impl.default_datareader_qos.lock().unwrap(), DataReaderQos::default());
    }
}