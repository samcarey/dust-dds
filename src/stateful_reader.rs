use std::collections::{BTreeSet, HashMap, };
use std::convert::TryInto;
use std::time::Instant;

use crate::cache::HistoryCache;
use crate::types::{Locator, ReliabilityKind, SequenceNumber, TopicKind, GUID, };
use crate::messages::RtpsMessage;
use crate::behavior::types::Duration;
use crate::behavior::StatefulReaderBehavior;
use crate::messages::types::Count;

pub struct WriterProxy {
    remote_writer_guid: GUID,
    unicast_locator_list: Vec<Locator>,
    multicast_locator_list: Vec<Locator>,
    
    // Optional atribute:
    // data_max_size_serialized: Long,
    
    // The cache changes may be left within the reader itself to avoid unneccesray referencing (or pointing)
    // rather the additional fields are used here
    // changes_from_writer: CacheChange[*],     
    
    // Groups are not supported yet:
    // remoteGroupEntityId: EntityId_t,

    // Additional fields (somewhat acording 8.4.15.1 Implementation of ReaderProxy and WriterProxy)
    highest_processed_sequence_number: SequenceNumber,
    unknown_changes: BTreeSet<SequenceNumber>,
    lost_changes: BTreeSet<SequenceNumber>,
    missing_changes: BTreeSet<SequenceNumber>,

    must_send_ack: bool,
    time_heartbeat_received: Instant,
    ackanck_count: Count,
    highest_received_heartbeat_count: Count,
}

impl WriterProxy {
    pub fn new(
        remote_writer_guid: GUID,
        unicast_locator_list: Vec<Locator>,
        multicast_locator_list: Vec<Locator>, 
        ) -> Self {
            Self {
                remote_writer_guid,
                unicast_locator_list,
                multicast_locator_list,
                highest_processed_sequence_number: SequenceNumber(0),
                unknown_changes: BTreeSet::new(),
                lost_changes: BTreeSet::new(),
                missing_changes: BTreeSet::new(),
                must_send_ack: false,
                time_heartbeat_received: Instant::now(),
                ackanck_count: Count(0),
                highest_received_heartbeat_count: Count(0),
        }
    }

    /// This operation returns the maximum SequenceNumber_t among the changes_from_writer changes in the RTPS WriterProxy
    /// that are available for access by the DDS DataReader. The condition to make any CacheChange ‘a_change’ available 
    /// for ‘access’ by the DDS DataReader is that there are no changes from the RTPS Writer with SequenceNumber_t smaller
    /// than or equal to a_change.sequenceNumber that have status MISSING or UNKNOWN. In other words, the available_changes_max
    /// and all previous changes are either RECEIVED or LOST.
    pub fn available_changes_max(&self) -> SequenceNumber {

        // The assumption is that all the numbers up to the highest processed sequence number
        // are read (which is typically the scenario). Whatever is not read is
        // marked in its own array as missing, unknown or lost. Therefore, the maximum avaiable
        // change is the minimum value between the missing, unknown and received/lost samples.

        let lowest_before_missing =
            self.missing_changes
            .iter().next()
            .map_or(self.highest_processed_sequence_number, |x| *x-1);

        
        let lowest_unknown = 
        self.unknown_changes
        .iter().next()
        .map_or(self.highest_processed_sequence_number, |x| *x-1);

        self.highest_processed_sequence_number.min(lowest_unknown).min(lowest_before_missing)
    }

    pub fn irrelevant_change_set(&mut self, _a_seq_num: SequenceNumber) {
        todo!()
    }

    /// This operation modifies the status stored in ChangeFromWriter for any changes in the WriterProxy whose 
    /// status is MISSING or UNKNOWN and have sequence numbers lower than ‘first_available_seq_num.’ The status of 
    /// those changes is modified to LOST indicating that the changes are no longer available in the WriterHistoryCache
    /// of the RTPS Writer represented by the RTPS WriterProxy
    pub fn lost_changes_update(&mut self, first_available_seq_num: SequenceNumber) {
        let remaining_unknown = self.unknown_changes.split_off(&first_available_seq_num);
        self.lost_changes.append(&mut self.unknown_changes);
        self.unknown_changes = remaining_unknown;
        
        let remaining_missing = self.missing_changes.split_off(&first_available_seq_num);
        self.lost_changes.append(&mut self.missing_changes);
        self.missing_changes = remaining_missing;

        if first_available_seq_num > self.highest_processed_sequence_number {
            for seq_num in self.highest_processed_sequence_number.0+1 .. first_available_seq_num.0 {
                self.lost_changes.insert(SequenceNumber(seq_num));
            }
            self.highest_processed_sequence_number = first_available_seq_num - 1;
        }
    }

    /// This operation returns the subset of changes for the WriterProxy that have status ‘MISSING.’
    /// The changes with status ‘MISSING’ represent the set of changes available in the HistoryCache of the RTPS Writer
    /// represented by the RTPS WriterProxy that have not been received by the RTPS Reader.
    pub fn missing_changes(&self) -> &BTreeSet<SequenceNumber> {
        &self.missing_changes
    }

    /// This operation modifies the status stored in ChangeFromWriter for any changes in the WriterProxy whose status is UNKNOWN 
    /// and have sequence numbers smaller or equal to ‘last_available_seq_num.’ The status of those changes is modified 
    /// from UNKNOWN to MISSING indicating that the changes are available at the WriterHistoryCache of the RTPS Writer represented 
    /// by the RTPS WriterProxy but have not been received by the RTPS Reader
    pub fn missing_changes_update(&mut self, last_available_seq_num: SequenceNumber) {
        let remaining_unknown = self.unknown_changes.split_off(&last_available_seq_num);
        self.missing_changes.append(&mut self.unknown_changes);
        self.unknown_changes = remaining_unknown;

        if last_available_seq_num > self.highest_processed_sequence_number {
            for seq_num in self.highest_processed_sequence_number.0+1 ..= last_available_seq_num.0 {
                self.missing_changes.insert(SequenceNumber(seq_num));
            }
            self.highest_processed_sequence_number = last_available_seq_num;
        }
    }

    /// This operation modifies the status of the ChangeFromWriter that refers to the CacheChange with the
    /// SequenceNumber_t ‘a_seq_num.’ The status of the change is set to ‘RECEIVED,’ indicating it has been received.
    pub fn received_change_set(&mut self, a_seq_num: SequenceNumber) {
        if a_seq_num > self.highest_processed_sequence_number {
            for seq_num in  self.highest_processed_sequence_number.0+1 .. a_seq_num.0 {
                self.unknown_changes.insert(SequenceNumber(seq_num));
            }
            self.highest_processed_sequence_number = a_seq_num;
        } else {
            self.unknown_changes.remove(&a_seq_num);
            self.missing_changes.remove(&a_seq_num);
        }
    }

    pub fn remote_writer_guid(&self) -> &GUID {
        &self.remote_writer_guid
    }

    pub fn must_send_ack(&self) -> bool {
        self.must_send_ack
    }

    pub fn set_must_send_ack(&mut self, value: bool) {
        self.must_send_ack = value;
    }

    pub fn time_heartbeat_received_reset(&mut self) {
        self.time_heartbeat_received  = Instant::now();
    }

    pub fn duration_since_heartbeat_received(&self) -> Duration {
        self.time_heartbeat_received.elapsed().try_into().unwrap()
    }

    pub fn ackanck_count(&self) -> &Count {
        &self.ackanck_count
    }

    pub fn increment_acknack_count(&mut self) {
        self.ackanck_count += 1;
    }
}

pub struct StatefulReader {
    // From Entity base class
    guid: GUID,
    // entity: Entity,

    // From Endpoint base class:
    topic_kind: TopicKind,
    reliability_level: ReliabilityKind,
    unicast_locator_list: Vec<Locator>,
    multicast_locator_list: Vec<Locator>,

    // From Reader base class:
    expects_inline_qos: bool,
    heartbeat_response_delay: Duration,

    reader_cache: HistoryCache,

    // Fields
    matched_writers: HashMap<GUID, WriterProxy>,
}

impl StatefulReader {
    pub fn new(
        guid: GUID,
        topic_kind: TopicKind,
        reliability_level: ReliabilityKind,
        unicast_locator_list: Vec<Locator>,
        multicast_locator_list: Vec<Locator>,
        expects_inline_qos: bool,
        heartbeat_response_delay: Duration,        
        ) -> Self {
        Self {
            guid,
            topic_kind,
            reliability_level,
            unicast_locator_list,
            multicast_locator_list,
            expects_inline_qos,
            heartbeat_response_delay,       
            reader_cache: HistoryCache::new(),
            matched_writers: HashMap::new(),
        }
    }

    pub fn matched_writer_add(&mut self, a_writer_proxy: WriterProxy) {
        self.matched_writers.insert(a_writer_proxy.remote_writer_guid, a_writer_proxy);
    }

    pub fn matched_writer_remove(&mut self, a_writer_proxy: &WriterProxy) {
        self.matched_writers.remove(&a_writer_proxy.remote_writer_guid);
    }
    
    pub fn matched_writer_lookup(&self, a_writer_guid: &GUID) -> Option<&WriterProxy> {
        self.matched_writers.get(a_writer_guid)
    }

    pub fn run(&mut self, a_writer_guid: &GUID, received_message: Option<&RtpsMessage>) -> Option<RtpsMessage> {
        let writer_proxy =  self.matched_writers.get_mut(a_writer_guid).unwrap();

        let submessages = match self.reliability_level {
            ReliabilityKind::BestEffort => StatefulReaderBehavior::run_best_effort(writer_proxy, &self.guid, &mut self.reader_cache, received_message),
            ReliabilityKind::Reliable => StatefulReaderBehavior::run_reliable(writer_proxy,  &self.guid, &mut self.reader_cache, self.heartbeat_response_delay, received_message),
        };

        match submessages {
            None => None,
            Some(submessages) => Some(RtpsMessage::new(*self.guid.prefix(), submessages)),
        }

    }

    pub fn reader_cache(&self) -> &HistoryCache {
        &self.reader_cache
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::GuidPrefix;
    use crate::types::constants::ENTITYID_SPDP_BUILTIN_PARTICIPANT_ANNOUNCER;

    #[test]
    fn received_changes_set_sequential_ordered() {
        let remote_writer_guid = GUID::new(GuidPrefix([1;12]), ENTITYID_SPDP_BUILTIN_PARTICIPANT_ANNOUNCER );
        let mut writer_proxy = WriterProxy::new(remote_writer_guid, vec![], vec![]);

        // Originally the writer proxy doesn't know of any available changes
        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(0));

        // If the next sequence number is received the unknown changes are empty
        writer_proxy.received_change_set(SequenceNumber(1));
        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(1));
        assert!(writer_proxy.unknown_changes.is_empty());

        // If the next sequence number is received the unknown changes are empty
        writer_proxy.received_change_set(SequenceNumber(2));
        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(2));
        assert!(writer_proxy.unknown_changes.is_empty());
    }

    #[test]
    fn received_changes_set_unordered() {
        let remote_writer_guid = GUID::new(GuidPrefix([1;12]), ENTITYID_SPDP_BUILTIN_PARTICIPANT_ANNOUNCER );
        let mut writer_proxy = WriterProxy::new(remote_writer_guid, vec![], vec![]);

        // Originally the writer proxy doesn't know of any available changes
        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(0));

        // If the next sequence number jumps two number then change 1 and 2 are unknown
        writer_proxy.received_change_set(SequenceNumber(3));
        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(0));
        assert_eq!(writer_proxy.unknown_changes, [SequenceNumber(1), SequenceNumber(2)].iter().cloned().collect());

        // If sequence number 2 is received the only unknown change is 1
        writer_proxy.received_change_set(SequenceNumber(1));
        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(1));
        assert_eq!(writer_proxy.unknown_changes, [SequenceNumber(2)].iter().cloned().collect());

        // If sequence number 1 is received after there are no more unknown changes
        writer_proxy.received_change_set(SequenceNumber(2));
        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(3));
        assert!(writer_proxy.unknown_changes.is_empty());
    }

    #[test]
    fn lost_changes_only() {
        let remote_writer_guid = GUID::new(GuidPrefix([1;12]), ENTITYID_SPDP_BUILTIN_PARTICIPANT_ANNOUNCER );
        let mut writer_proxy = WriterProxy::new(remote_writer_guid, vec![], vec![]);

        writer_proxy.lost_changes_update(SequenceNumber(3));

        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(2));
        assert_eq!(writer_proxy.lost_changes, [SequenceNumber(1), SequenceNumber(2)].iter().cloned().collect());

        writer_proxy.lost_changes_update(SequenceNumber(5));
        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(4));
        assert_eq!(writer_proxy.lost_changes, [SequenceNumber(1), SequenceNumber(2), SequenceNumber(3), SequenceNumber(4)].iter().cloned().collect());
    }

    #[test]
    fn received_and_lost_changes() {
        let remote_writer_guid = GUID::new(GuidPrefix([1;12]), ENTITYID_SPDP_BUILTIN_PARTICIPANT_ANNOUNCER );
        let mut writer_proxy = WriterProxy::new(remote_writer_guid, vec![], vec![]);

        writer_proxy.received_change_set(SequenceNumber(3));
        writer_proxy.lost_changes_update(SequenceNumber(3));

        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(3));
        assert_eq!(writer_proxy.lost_changes, [SequenceNumber(1), SequenceNumber(2)].iter().cloned().collect());

        writer_proxy.lost_changes_update(SequenceNumber(5));
        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(4));
        assert_eq!(writer_proxy.lost_changes, [SequenceNumber(1), SequenceNumber(2), SequenceNumber(4)].iter().cloned().collect());
    }

    #[test]
    fn missing_changes_only() {
        let remote_writer_guid = GUID::new(GuidPrefix([1;12]), ENTITYID_SPDP_BUILTIN_PARTICIPANT_ANNOUNCER );
        let mut writer_proxy = WriterProxy::new(remote_writer_guid, vec![], vec![]);

        writer_proxy.missing_changes_update(SequenceNumber(3));

        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(0));
        assert_eq!(writer_proxy.missing_changes, [SequenceNumber(1), SequenceNumber(2), SequenceNumber(3)].iter().cloned().collect());

        writer_proxy.missing_changes_update(SequenceNumber(5));
        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(0));
        assert_eq!(writer_proxy.missing_changes, [SequenceNumber(1), SequenceNumber(2), SequenceNumber(3), SequenceNumber(4), SequenceNumber(5)].iter().cloned().collect());
    }

    #[test]
    fn received_lost_and_missing_changes() {
        let remote_writer_guid = GUID::new(GuidPrefix([1;12]), ENTITYID_SPDP_BUILTIN_PARTICIPANT_ANNOUNCER );
        let mut writer_proxy = WriterProxy::new(remote_writer_guid, vec![], vec![]);

        // Received sample number 4. Verify that all sample numbers before are unknown
        writer_proxy.received_change_set(SequenceNumber(4));
        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(0));
        assert_eq!(writer_proxy.unknown_changes, [SequenceNumber(1), SequenceNumber(2), SequenceNumber(3)].iter().cloned().collect());
        assert_eq!(writer_proxy.missing_changes, [].iter().cloned().collect());
        assert_eq!(writer_proxy.lost_changes, [].iter().cloned().collect());

        // Mark sample 1 as lost and verify that now samples 2 and 3 are unknown
        writer_proxy.lost_changes_update(SequenceNumber(2));
        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(1));
        assert_eq!(writer_proxy.unknown_changes, [SequenceNumber(2), SequenceNumber(3)].iter().cloned().collect());
        assert_eq!(writer_proxy.missing_changes, [].iter().cloned().collect());
        assert_eq!(writer_proxy.lost_changes, [SequenceNumber(1)].iter().cloned().collect());

        // Mark sample 2 as missing
        writer_proxy.missing_changes_update(SequenceNumber(3));
        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(1));
        assert_eq!(writer_proxy.unknown_changes, [SequenceNumber(3)].iter().cloned().collect());
        assert_eq!(writer_proxy.missing_changes, [SequenceNumber(2)].iter().cloned().collect());
        assert_eq!(writer_proxy.lost_changes, [SequenceNumber(1)].iter().cloned().collect());

        // Mark sample 2 as received
        writer_proxy.received_change_set(SequenceNumber(2));
        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(2));
        assert_eq!(writer_proxy.unknown_changes, [SequenceNumber(3)].iter().cloned().collect());
        assert_eq!(writer_proxy.missing_changes, [].iter().cloned().collect());
        assert_eq!(writer_proxy.lost_changes, [SequenceNumber(1)].iter().cloned().collect());

        // Mark samples 1 and 2 as lost and check that nothing happens
        writer_proxy.lost_changes_update(SequenceNumber(3));
        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(2));
        assert_eq!(writer_proxy.unknown_changes, [SequenceNumber(3)].iter().cloned().collect());
        assert_eq!(writer_proxy.missing_changes, [].iter().cloned().collect());
        assert_eq!(writer_proxy.lost_changes, [SequenceNumber(1)].iter().cloned().collect());

        // Mark sample 3 as missing
        writer_proxy.missing_changes_update(SequenceNumber(4));
        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(2));
        assert_eq!(writer_proxy.unknown_changes, [].iter().cloned().collect());
        assert_eq!(writer_proxy.missing_changes, [SequenceNumber(3)].iter().cloned().collect());
        assert_eq!(writer_proxy.lost_changes, [SequenceNumber(1)].iter().cloned().collect());

        // Mark sample 3 as lost
        writer_proxy.lost_changes_update(SequenceNumber(4));
        assert_eq!(writer_proxy.available_changes_max(), SequenceNumber(4));
        assert_eq!(writer_proxy.unknown_changes, [].iter().cloned().collect());
        assert_eq!(writer_proxy.missing_changes, [].iter().cloned().collect());
        assert_eq!(writer_proxy.lost_changes, [SequenceNumber(1), SequenceNumber(3)].iter().cloned().collect());
    }
}