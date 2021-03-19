use super::submessage_elements;
use super::Submessage;
use super::SubmessageFlag;

pub trait Gap: Submessage {
    type EntityId: submessage_elements::EntityId;
    type SequenceNumber: submessage_elements::SequenceNumber;
    type SequenceNumberSet: submessage_elements::SequenceNumberSet;

    fn endianness_flag(&self) -> SubmessageFlag;
    // group_info_flag: SubmessageFlag,
    fn reader_id(&self) -> &Self::EntityId;
    fn writer_id(&self) -> &Self::EntityId;
    fn gap_start(&self) -> &Self::SequenceNumber;
    fn gap_list(&self) -> &Self::SequenceNumberSet;
    // gap_start_gsn: submessage_elements::SequenceNumber,
    // gap_end_gsn: submessage_elements::SequenceNumber,
}
