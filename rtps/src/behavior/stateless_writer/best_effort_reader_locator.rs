use std::collections::BTreeSet;

use crate::{
    behavior::{data_from_cache_change, BEHAVIOR_ENDIANNESS},
    messages::{submessages::Gap, RtpsSubmessage},
    structure::HistoryCache,
    types::{constants::ENTITYID_UNKNOWN, EntityId, SequenceNumber},
};

use super::reader_locator::ReaderLocator;

pub struct BestEffortReaderLocatorBehavior;

impl BestEffortReaderLocatorBehavior {
    pub fn produce_messages(
        reader_locator: &mut ReaderLocator,
        history_cache: &HistoryCache,
        writer_entity_id: EntityId,
        last_change_sequence_number: SequenceNumber,
    ) -> Vec<RtpsSubmessage> {
        let mut message_queue = Vec::new();
        if !reader_locator.unsent_changes(last_change_sequence_number).is_empty() {
            Self::pushing_state(
                reader_locator,
                history_cache,
                writer_entity_id,
                last_change_sequence_number,
                &mut message_queue,
            );
        }
        message_queue
    }

    fn pushing_state(
        reader_locator: &mut ReaderLocator,
        history_cache: &HistoryCache,
        writer_entity_id: EntityId,
        last_change_sequence_number: SequenceNumber,
        message_queue: &mut Vec<RtpsSubmessage>,
    ) {
        while let Some(next_unsent_seq_num) = reader_locator.next_unsent_change(last_change_sequence_number) {
            Self::transition_t4(
                reader_locator,
                history_cache,
                writer_entity_id,
                next_unsent_seq_num,
                message_queue,
            );
        }
    }

    fn transition_t4(
        _reader_locator: &mut ReaderLocator,
        history_cache: &HistoryCache,
        writer_entity_id: EntityId,
        next_unsent_seq_num: SequenceNumber,
        message_queue: &mut Vec<RtpsSubmessage>,
    ) {
        if let Some(cache_change) = history_cache.get_change(next_unsent_seq_num) {
            let data = data_from_cache_change(cache_change, ENTITYID_UNKNOWN);
            message_queue.push(RtpsSubmessage::Data(data));
        } else {
            let gap = Gap::new(
                BEHAVIOR_ENDIANNESS,
                ENTITYID_UNKNOWN,
                writer_entity_id,
                next_unsent_seq_num,
                BTreeSet::new(),
            );

            message_queue.push(RtpsSubmessage::Gap(gap));
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{GUID, Locator};
    use crate::types::{constants::ENTITYID_BUILTIN_PARTICIPANT_MESSAGE_WRITER, ChangeKind};

    use crate::structure::CacheChange;

    #[test]
    fn produce_empty() {
        let locator = Locator::new_udpv4(7400, [127, 0, 0, 1]);
        let mut reader_locator = ReaderLocator::new(locator);
        let writer_entity_id = ENTITYID_BUILTIN_PARTICIPANT_MESSAGE_WRITER;
        let history_cache = HistoryCache::default();

        // Run without any change being created or added in the cache
        let last_change_sequence_number = 0;
        let messages_vec = BestEffortReaderLocatorBehavior::produce_messages(
            &mut reader_locator,
            &history_cache,
            writer_entity_id,
            last_change_sequence_number,
        );

        assert!(messages_vec.is_empty());
    }

    #[test]
    fn produce_data_message() {
        let locator = Locator::new_udpv4(7400, [127, 0, 0, 1]);
        let mut reader_locator = ReaderLocator::new(locator);
        let writer_entity_id = ENTITYID_BUILTIN_PARTICIPANT_MESSAGE_WRITER;
        let mut history_cache = HistoryCache::default();

        // Add one change to the history cache
        let writer_guid = GUID::new([5; 12], writer_entity_id);
        let instance_handle = [1; 16];
        let cache_change1 = CacheChange::new(
            ChangeKind::Alive,
            writer_guid.into(),
            instance_handle,
            1,
            Some(vec![1, 2, 3]),
            None,
        );
        history_cache.add_change(cache_change1.clone());

        // Run with the last change sequence number equal to the added cache change
        let last_change_sequence_number = 1;
        let messages_vec = BestEffortReaderLocatorBehavior::produce_messages(
            &mut reader_locator,
            &history_cache,
            writer_entity_id,
            last_change_sequence_number,
        );

        let expected_data_submessage =
            RtpsSubmessage::Data(data_from_cache_change(&cache_change1, ENTITYID_UNKNOWN));
        assert_eq!(messages_vec.len(), 1);
        assert!(messages_vec.contains(&expected_data_submessage));
    }

    #[test]
    fn produce_gap_message() {
        let locator = Locator::new_udpv4(7400, [127, 0, 0, 1]);
        let mut reader_locator = ReaderLocator::new(locator);
        let writer_entity_id = ENTITYID_BUILTIN_PARTICIPANT_MESSAGE_WRITER;
        let history_cache = HistoryCache::default();

        // Run with the a sequence number of 1 without adding any change to the history cache
        let last_change_sequence_number = 1;
        let messages_vec = BestEffortReaderLocatorBehavior::produce_messages(
            &mut reader_locator,
            &history_cache,
            writer_entity_id,
            last_change_sequence_number,
        );

        let expected_gap_submessage = RtpsSubmessage::Gap(Gap::new(
            BEHAVIOR_ENDIANNESS,
            ENTITYID_UNKNOWN,
            writer_entity_id,
            1,
            BTreeSet::new(),
        ));
        assert_eq!(messages_vec.len(), 1);
        assert!(messages_vec.contains(&expected_gap_submessage));
    }

    #[test]
    fn produce_data_and_gap_messages() {
        let locator = Locator::new_udpv4(7400, [127, 0, 0, 1]);
        let mut reader_locator = ReaderLocator::new(locator);
        let writer_entity_id = ENTITYID_BUILTIN_PARTICIPANT_MESSAGE_WRITER;
        let mut history_cache = HistoryCache::default();

        // Add one change to the history cache
        let writer_guid = GUID::new([5; 12], writer_entity_id);
        let instance_handle = [1; 16];
        let cache_change1 = CacheChange::new(
            ChangeKind::Alive,
            writer_guid.into(),
            instance_handle,
            1,
            Some(vec![1, 2, 3]),
            None,
        );
        history_cache.add_change(cache_change1.clone());

        // Run with the last change sequence number one above the added cache change
        let last_change_sequence_number = 2;
        let messages_vec = BestEffortReaderLocatorBehavior::produce_messages(
            &mut reader_locator,
            &history_cache,
            writer_entity_id,
            last_change_sequence_number,
        );

        let expected_data_submessage =
            RtpsSubmessage::Data(data_from_cache_change(&cache_change1, ENTITYID_UNKNOWN));
        let expected_gap_submessage = RtpsSubmessage::Gap(Gap::new(
            BEHAVIOR_ENDIANNESS,
            ENTITYID_UNKNOWN,
            writer_entity_id,
            2,
            BTreeSet::new(),
        ));
        assert_eq!(messages_vec.len(), 2);
        assert!(messages_vec.contains(&expected_data_submessage));
        assert!(messages_vec.contains(&expected_gap_submessage));
    }
}
