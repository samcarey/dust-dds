use crate::implementation::rtps::types::{Locator, SequenceNumber};

use super::types::FragmentNumber;

///
/// This files shall only contain the types as listed in the DDSI-RTPS Version 2.3
/// 8.3.5 RTPS SubmessageElements
///

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UShortSubmessageElement {
    pub value: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ULongSubmessageElement {
    pub value: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SequenceNumberSet {
    pub base: SequenceNumber,
    pub set: Vec<SequenceNumber>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FragmentNumberSet {
    pub base: FragmentNumber,
    pub set: Vec<FragmentNumber>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimestampSubmessageElement {
    pub value: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter<'a> {
    pub parameter_id: u16,
    pub length: i16,
    pub value: &'a [u8],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParameterListSubmessageElement<'a> {
    pub parameter: Vec<Parameter<'a>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocatorListSubmessageElement {
    pub value: Vec<Locator>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SerializedDataSubmessageElement<'a> {
    pub value: &'a [u8],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SerializedDataFragmentSubmessageElement<'a> {
    pub value: &'a [u8],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupDigestSubmessageElement {
    pub value: [u8; 4],
}
