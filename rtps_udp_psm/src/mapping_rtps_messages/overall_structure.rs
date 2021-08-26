use std::io::{BufRead, Write};

use byteorder::ByteOrder;
use rust_rtps_pim::{
    messages::{
        submessage_elements::Parameter, submessages::RtpsSubmessageType, RtpsMessage,
        RtpsSubmessageHeader,
    },
    structure::types::SequenceNumber,
};

use crate::{deserialize::Deserialize, serialize::Serialize};

use super::submessages::submessage_header::{
    ACKNACK, DATA, DATA_FRAG, GAP, HEARTBEAT, HEARTBEAT_FRAG, INFO_DST, INFO_REPLY, INFO_SRC,
    INFO_TS, NACK_FRAG, PAD,
};

type RtpsSubmessageWrite<'a> =
    RtpsSubmessageType<'a, Vec<SequenceNumber>, &'a [Parameter<'a>], (), ()>;
type RtpsSubmessageRead<'a> =
    RtpsSubmessageType<'a, Vec<SequenceNumber>, Vec<Parameter<'a>>, (), ()>;

pub type RtpsMessageWrite<'a> = RtpsMessage<Vec<RtpsSubmessageWrite<'a>>>;
pub type RtpsMessageRead<'a> = RtpsMessage<Vec<RtpsSubmessageRead<'a>>>;

impl<'a> Serialize for RtpsSubmessageWrite<'_> {
    fn serialize<W: Write, B: ByteOrder>(&self, mut writer: W) -> crate::serialize::Result {
        match self {
            RtpsSubmessageType::AckNack(s) => s.serialize::<_, B>(&mut writer)?,
            RtpsSubmessageType::Data(s) => s.serialize::<_, B>(&mut writer)?,
            RtpsSubmessageType::DataFrag(s) => s.serialize::<_, B>(&mut writer)?,
            RtpsSubmessageType::Gap(s) => s.serialize::<_, B>(&mut writer)?,
            RtpsSubmessageType::Heartbeat(s) => s.serialize::<_, B>(&mut writer)?,
            RtpsSubmessageType::HeartbeatFrag(s) => s.serialize::<_, B>(&mut writer)?,
            RtpsSubmessageType::InfoDestination(s) => s.serialize::<_, B>(&mut writer)?,
            RtpsSubmessageType::InfoReply(s) => s.serialize::<_, B>(&mut writer)?,
            RtpsSubmessageType::InfoSource(s) => s.serialize::<_, B>(&mut writer)?,
            RtpsSubmessageType::InfoTimestamp(s) => s.serialize::<_, B>(&mut writer)?,
            RtpsSubmessageType::NackFrag(s) => s.serialize::<_, B>(&mut writer)?,
            RtpsSubmessageType::Pad(s) => s.serialize::<_, B>(&mut writer)?,
        };
        Ok(())
    }
}

impl Serialize for RtpsMessageWrite<'_> {
    fn serialize<W: Write, B: ByteOrder>(&self, mut writer: W) -> crate::serialize::Result {
        self.header.serialize::<_, B>(&mut writer)?;
        for submessage in &self.submessages {
            submessage.serialize::<_, B>(&mut writer)?;
        }
        Ok(())
    }
}

impl<'a, 'de: 'a> Deserialize<'de> for RtpsMessageRead<'a> {
    fn deserialize<B: ByteOrder>(buf: &mut &'de [u8]) -> crate::deserialize::Result<Self> {
        let header = Deserialize::deserialize::<B>(buf)?;
        const MAX_SUBMESSAGES: usize = 2_usize.pow(16);
        let mut submessages = vec![];
        for _ in 0..MAX_SUBMESSAGES {
            if buf.len() < 4 {
                break;
            }
            // Preview byte only (to allow full deserialization of submessage header)
            let submessage_id = buf[0];
            let submessage = match submessage_id {
                ACKNACK => RtpsSubmessageType::AckNack(Deserialize::deserialize::<B>(buf)?),
                DATA => RtpsSubmessageType::Data(Deserialize::deserialize::<B>(buf)?),
                DATA_FRAG => RtpsSubmessageType::DataFrag(Deserialize::deserialize::<B>(buf)?),
                GAP => RtpsSubmessageType::Gap(Deserialize::deserialize::<B>(buf)?),
                HEARTBEAT => RtpsSubmessageType::Heartbeat(Deserialize::deserialize::<B>(buf)?),
                HEARTBEAT_FRAG => {
                    RtpsSubmessageType::HeartbeatFrag(Deserialize::deserialize::<B>(buf)?)
                }
                INFO_DST => {
                    RtpsSubmessageType::InfoDestination(Deserialize::deserialize::<B>(buf)?)
                }
                INFO_REPLY => RtpsSubmessageType::InfoReply(Deserialize::deserialize::<B>(buf)?),
                INFO_SRC => RtpsSubmessageType::InfoSource(Deserialize::deserialize::<B>(buf)?),
                INFO_TS => RtpsSubmessageType::InfoTimestamp(Deserialize::deserialize::<B>(buf)?),
                NACK_FRAG => RtpsSubmessageType::NackFrag(Deserialize::deserialize::<B>(buf)?),
                PAD => RtpsSubmessageType::Pad(Deserialize::deserialize::<B>(buf)?),
                _ => {
                    let submessage_header: RtpsSubmessageHeader =
                        Deserialize::deserialize::<B>(buf)?;
                    buf.consume(submessage_header.submessage_length as usize);
                    continue;
                }
            };
            submessages.push(submessage);
        }
        Ok(Self {
            header,
            submessages,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::deserialize::from_bytes_le;
    use crate::serialize::to_bytes_le;
    use rust_rtps_pim::messages::submessage_elements::{
        EntityIdSubmessageElement, Parameter, ParameterListSubmessageElement,
        SequenceNumberSubmessageElement, SerializedDataSubmessageElement,
    };

    use rust_rtps_pim::messages::submessages::DataSubmessage;
    use rust_rtps_pim::messages::types::ParameterId;
    use rust_rtps_pim::messages::{types::ProtocolId, RtpsMessageHeader};
    use rust_rtps_pim::structure::types::{EntityId, EntityKind, ProtocolVersion};

    #[test]
    fn serialize_rtps_message_no_submessage() {
        let header = RtpsMessageHeader {
            protocol: ProtocolId::PROTOCOL_RTPS,
            version: ProtocolVersion { major: 2, minor: 3 },
            vendor_id: [9, 8],
            guid_prefix: [3; 12],
        };
        let value = RtpsMessage {
            header,
            submessages: Vec::new(),
        };
        #[rustfmt::skip]
        assert_eq!(to_bytes_le(&value).unwrap(), vec![
            b'R', b'T', b'P', b'S', // Protocol
            2, 3, 9, 8, // ProtocolVersion | VendorId
            3, 3, 3, 3, // GuidPrefix
            3, 3, 3, 3, // GuidPrefix
            3, 3, 3, 3, // GuidPrefix
        ]);
    }

    #[test]
    fn serialize_rtps_message() {
        let header = RtpsMessageHeader {
            protocol: ProtocolId::PROTOCOL_RTPS,
            version: ProtocolVersion { major: 2, minor: 3 },
            vendor_id: [9, 8],
            guid_prefix: [3; 12],
        };
        let endianness_flag = true;
        let inline_qos_flag = true;
        let data_flag = false;
        let key_flag = false;
        let non_standard_payload_flag = false;
        let reader_id = EntityIdSubmessageElement {
            value: EntityId::new([1, 2, 3], EntityKind::UserDefinedReaderNoKey),
        };
        let writer_id = EntityIdSubmessageElement {
            value: EntityId::new([6, 7, 8], EntityKind::UserDefinedReaderGroup),
        };
        let writer_sn = SequenceNumberSubmessageElement { value: 5 };
        let parameter_1 = Parameter::new(ParameterId(6), &[10, 11, 12, 13]);
        let parameter_2 = Parameter::new(ParameterId(7), &[20, 21, 22, 23]);
        let parameter_list = [parameter_1, parameter_2];
        let inline_qos = ParameterListSubmessageElement {
            parameter: parameter_list.as_ref(),
        };
        let serialized_payload = SerializedDataSubmessageElement { value: &[] };

        let submessage = RtpsSubmessageType::Data(DataSubmessage {
            endianness_flag,
            inline_qos_flag,
            data_flag,
            key_flag,
            non_standard_payload_flag,
            reader_id,
            writer_id,
            writer_sn,
            inline_qos,
            serialized_payload,
        });
        let value = RtpsMessage {
            header,
            submessages: vec![submessage],
        };
        #[rustfmt::skip]
        assert_eq!(to_bytes_le(&value).unwrap(), vec![
            b'R', b'T', b'P', b'S', // Protocol
            2, 3, 9, 8, // ProtocolVersion | VendorId
            3, 3, 3, 3, // GuidPrefix
            3, 3, 3, 3, // GuidPrefix
            3, 3, 3, 3, // GuidPrefix
            0x15, 0b_0000_0011, 40, 0, // Submessage header
            0, 0, 16, 0, // extraFlags, octetsToInlineQos
            1, 2, 3, 4, // readerId: value[4]
            6, 7, 8, 9, // writerId: value[4]
            0, 0, 0, 0, // writerSN: high
            5, 0, 0, 0, // writerSN: low
            6, 0, 4, 0, // inlineQos: parameterId_1, length_1
            10, 11, 12, 13, // inlineQos: value_1[length_1]
            7, 0, 4, 0, // inlineQos: parameterId_2, length_2
            20, 21, 22, 23, // inlineQos: value_2[length_2]
            1, 0, 0, 0, // inlineQos: Sentinel
        ]);
    }

    #[test]
    fn deserialize_rtps_message_no_submessage() {
        let header = RtpsMessageHeader {
            protocol: ProtocolId::PROTOCOL_RTPS,
            version: ProtocolVersion { major: 2, minor: 3 },
            vendor_id: [9, 8],
            guid_prefix: [3; 12],
        };

        let expected = RtpsMessage {
            header,
            submessages: Vec::new(),
        };
        #[rustfmt::skip]
        let result: RtpsMessageRead = from_bytes_le(&[
            b'R', b'T', b'P', b'S', // Protocol
            2, 3, 9, 8, // ProtocolVersion | VendorId
            3, 3, 3, 3, // GuidPrefix
            3, 3, 3, 3, // GuidPrefix
            3, 3, 3, 3, // GuidPrefix
        ]).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn deserialize_rtps_message() {
        let header = RtpsMessageHeader {
            protocol: ProtocolId::PROTOCOL_RTPS,
            version: ProtocolVersion { major: 2, minor: 3 },
            vendor_id: [9, 8],
            guid_prefix: [3; 12],
        };
        let endianness_flag = true;
        let inline_qos_flag = true;
        let data_flag = false;
        let key_flag = false;
        let non_standard_payload_flag = false;
        let reader_id = EntityIdSubmessageElement {
            value: EntityId::new([1, 2, 3], EntityKind::UserDefinedReaderNoKey),
        };
        let writer_id = EntityIdSubmessageElement {
            value: EntityId::new([6, 7, 8], EntityKind::UserDefinedReaderGroup),
        };
        let writer_sn = SequenceNumberSubmessageElement { value: 5 };
        let parameter_1 = Parameter::new(ParameterId(6), &[10, 11, 12, 13]);
        let parameter_2 = Parameter::new(ParameterId(7), &[20, 21, 22, 23]);
        let inline_qos = ParameterListSubmessageElement {
            parameter: vec![parameter_1, parameter_2],
        };
        let serialized_payload = SerializedDataSubmessageElement { value: &[] };

        let submessage = RtpsSubmessageType::Data(DataSubmessage {
            endianness_flag,
            inline_qos_flag,
            data_flag,
            key_flag,
            non_standard_payload_flag,
            reader_id,
            writer_id,
            writer_sn,
            inline_qos,
            serialized_payload,
        });
        let expected = RtpsMessage {
            header,
            submessages: vec![submessage],
        };
        #[rustfmt::skip]
        let result: RtpsMessageRead = from_bytes_le(&[
            b'R', b'T', b'P', b'S', // Protocol
            2, 3, 9, 8, // ProtocolVersion | VendorId
            3, 3, 3, 3, // GuidPrefix
            3, 3, 3, 3, // GuidPrefix
            3, 3, 3, 3, // GuidPrefix
            0x15, 0b_0000_0011, 40, 0, // Submessage header
            0, 0, 16, 0, // extraFlags, octetsToInlineQos
            1, 2, 3, 4, // readerId: value[4]
            6, 7, 8, 9, // writerId: value[4]
            0, 0, 0, 0, // writerSN: high
            5, 0, 0, 0, // writerSN: low
            6, 0, 4, 0, // inlineQos: parameterId_1, length_1
            10, 11, 12, 13, // inlineQos: value_1[length_1]
            7, 0, 4, 0, // inlineQos: parameterId_2, length_2
            20, 21, 22, 23, // inlineQos: value_2[length_2]
            1, 0, 0, 0, // inlineQos: Sentinel
        ]).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn deserialize_rtps_message_unknown_submessage() {
        let header = RtpsMessageHeader {
            protocol: ProtocolId::PROTOCOL_RTPS,
            version: ProtocolVersion { major: 2, minor: 3 },
            vendor_id: [9, 8],
            guid_prefix: [3; 12],
        };
        let endianness_flag = true;
        let inline_qos_flag = true;
        let data_flag = false;
        let key_flag = false;
        let non_standard_payload_flag = false;
        let reader_id = EntityIdSubmessageElement {
            value: EntityId::new([1, 2, 3], EntityKind::UserDefinedReaderNoKey),
        };
        let writer_id = EntityIdSubmessageElement {
            value: EntityId::new([6, 7, 8], EntityKind::UserDefinedReaderGroup),
        };
        let writer_sn = SequenceNumberSubmessageElement { value: 5 };
        let parameter_1 = Parameter::new(ParameterId(6), &[10, 11, 12, 13]);
        let parameter_2 = Parameter::new(ParameterId(7), &[20, 21, 22, 23]);
        let inline_qos = ParameterListSubmessageElement {
            parameter: vec![parameter_1, parameter_2],
        };
        let serialized_payload = SerializedDataSubmessageElement { value: &[] };

        let submessage = RtpsSubmessageType::Data(DataSubmessage {
            endianness_flag,
            inline_qos_flag,
            data_flag,
            key_flag,
            non_standard_payload_flag,
            reader_id,
            writer_id,
            writer_sn,
            inline_qos,
            serialized_payload,
        });
        let expected = RtpsMessage {
            header,
            submessages: vec![submessage],
        };
        #[rustfmt::skip]
        let result: RtpsMessageRead = from_bytes_le(&[
            b'R', b'T', b'P', b'S', // Protocol
            2, 3, 9, 8, // ProtocolVersion | VendorId
            3, 3, 3, 3, // GuidPrefix
            3, 3, 3, 3, // GuidPrefix
            3, 3, 3, 3, // GuidPrefix
            0x99, 0b_0101_0011, 4, 0, // Submessage header
            9, 9, 9, 9, // Unkown data
            0x15, 0b_0000_0011, 40, 0, // Submessage header
            0, 0, 16, 0, // extraFlags, octetsToInlineQos
            1, 2, 3, 4, // readerId: value[4]
            6, 7, 8, 9, // writerId: value[4]
            0, 0, 0, 0, // writerSN: high
            5, 0, 0, 0, // writerSN: low
            6, 0, 4, 0, // inlineQos: parameterId_1, length_1
            10, 11, 12, 13, // inlineQos: value_1[length_1]
            7, 0, 4, 0, // inlineQos: parameterId_2, length_2
            20, 21, 22, 23, // inlineQos: value_2[length_2]
            1, 0, 0, 0, // inlineQos: Sentinel
        ]).unwrap();
        assert_eq!(result, expected);
    }
}