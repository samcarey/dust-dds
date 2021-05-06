use crate::{
    domain::domain_participant::DomainParticipant,
    infrastructure::{entity::Entity, qos::TopicQos},
    return_type::DDSResult,
};

use super::topic_listener::TopicListener;

/// TopicDescription represents the fact that both publications and subscriptions are tied to a single data-type. Its attribute
/// type_name defines a unique resulting type for the publication or the subscription and therefore creates an implicit association
/// with a TypeSupport. TopicDescription has also a name that allows it to be retrieved locally.
/// This class is an abstract class. It is the base class for Topic, ContentFilteredTopic, and MultiTopic.
pub trait TopicDescription<'t, 'dp: 't, T: 't>:
    Entity<Qos = TopicQos<'t>, Listener = &'t (dyn TopicListener<DataType = T> + 't)>
{
    /// This operation returns the DomainParticipant to which the Topic Description belongs.
    fn get_participant(&self) -> &dyn DomainParticipant<'dp>;

    /// The type_name used to create the TopicDescription
    fn get_type_name(&self) -> DDSResult<&'static str>;

    /// The name used to create the TopicDescription
    fn get_name(&self) -> DDSResult<&'t str>;
}

pub trait AnyTopicDescription {}
