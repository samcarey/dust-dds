use std::sync::{Arc, Condvar};

use crate::infrastructure::{error::DdsResult, status::StatusKind};

pub struct StatusConditionImpl {
    enabled_statuses: Vec<StatusKind>,
    communication_status: Vec<StatusKind>,
    cvar_list: Vec<Arc<Condvar>>,
}

impl Default for StatusConditionImpl {
    fn default() -> Self {
        Self {
            enabled_statuses: vec![
                StatusKind::InconsistentTopicStatus,
                StatusKind::OfferedDeadlineMissedStatus,
                StatusKind::RequestedDeadlineMissedStatus,
                StatusKind::OfferedIncompatibleQosStatus,
                StatusKind::RequestedIncompatibleQosStatus,
                StatusKind::SampleLostStatus,
                StatusKind::SampleRejectedStatus,
                StatusKind::DataOnReadersStatus,
                StatusKind::DataAvailableStatus,
                StatusKind::LivelinessLostStatus,
                StatusKind::LivelinessChangedStatus,
                StatusKind::PublicationMatchedStatus,
                StatusKind::SubscriptionMatchedStatus,
            ],
            communication_status: Vec::new(),
            cvar_list: Vec::new(),
        }
    }
}

impl StatusConditionImpl {
    pub fn get_enabled_statuses(&self) -> Vec<StatusKind> {
        self.enabled_statuses.clone()
    }

    pub fn set_enabled_statuses(&mut self, mask: &[StatusKind]) -> DdsResult<()> {
        self.enabled_statuses = mask.to_vec();
        Ok(())
    }

    pub fn get_trigger_value(&self) -> bool {
        for status in &self.communication_status {
            if self.enabled_statuses.contains(status) {
                return true;
            }
        }
        false
    }

    pub(crate) fn add_communication_state(&mut self, state: StatusKind) {
        self.communication_status.push(state);

        if self.get_trigger_value() {
            self.communication_status.retain(|x| x != &state);
            for cvar in self.cvar_list.iter() {
                cvar.notify_all();
            }
        }
    }

    pub(crate) fn push_cvar(&mut self, cvar: Arc<Condvar>) {
        self.cvar_list.push(cvar)
    }
}
