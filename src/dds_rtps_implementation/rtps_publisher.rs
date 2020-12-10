use crate::dds_infrastructure::entity::{Entity, StatusCondition};
use crate::dds_infrastructure::publisher_listener::PublisherListener;
use crate::dds_infrastructure::qos::{DataWriterQos, PublisherQos, TopicQos};
use crate::dds_infrastructure::status::StatusMask;
use crate::dds_rtps_implementation::rtps_data_writer::RtpsDataWriter;
use crate::dds_rtps_implementation::rtps_data_writer::RtpsDataWriterInner;
use crate::dds_rtps_implementation::rtps_object::RtpsObject;
use crate::dds_rtps_implementation::rtps_object::RtpsObjectReference;
use crate::types::{DDSType, Duration, InstanceHandle, ReturnCode};

#[derive(Default)]
pub struct RtpsPublisherInner {
    writer_list: [RtpsObject<RtpsDataWriterInner>; 32],
}

pub type RtpsPublisher<'a> = RtpsObjectReference<'a, RtpsPublisherInner>;

impl RtpsPublisherInner {
    pub fn create_datawriter(&self) -> Option<RtpsDataWriter> {
        let datawriter_object = self
            .writer_list
            .iter()
            .find(|x| x.is_empty())?;
        let new_datawriter_inner = RtpsDataWriterInner::new();
        datawriter_object.initialize(new_datawriter_inner);
        datawriter_object
            .get_reference()
            .ok()
    }

    pub fn delete_datawriter(&self, _a_datawriter: &RtpsDataWriter) -> ReturnCode<()> {
        todo!()
    }

    pub fn lookup_datawriter(&self, _topic_name: &str) -> Option<RtpsDataWriter> {
        todo!()
    }

    pub fn suspend_publications(&self) -> ReturnCode<()> {
        todo!()
    }

    pub fn resume_publications(&self) -> ReturnCode<()> {
        todo!()
    }

    pub fn begin_coherent_changes(&self) -> ReturnCode<()> {
        todo!()
    }

    pub fn end_coherent_changes(&self) -> ReturnCode<()> {
        todo!()
    }

    pub fn wait_for_acknowledgments(&self, _max_wait: Duration) -> ReturnCode<()> {
        todo!()
    }

    pub fn delete_contained_entities(&self) -> ReturnCode<()> {
        todo!()
    }

    pub fn set_default_datawriter_qos(&self, _qos: DataWriterQos) -> ReturnCode<()> {
        todo!()
    }

    pub fn get_default_datawriter_qos(&self) -> ReturnCode<DataWriterQos> {
        todo!()
    }

    pub fn copy_from_topic_qos(
        &self,
        _a_datawriter_qos: &mut DataWriterQos,
        _a_topic_qos: &TopicQos,
    ) -> ReturnCode<()> {
        todo!()
    }
}
