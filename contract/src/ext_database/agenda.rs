use crate::*;

#[near]
impl Contract {
    /// Allows users to get the stringified JSON for the agenda and its timestamp
    pub fn get_agenda(&self) -> (String, u64) {
        (self.agenda.clone(), self.agenda_timestamp)
    }

    /// Allows an admin to set the new version of the agenda with a timestamp
    ///
    /// # Arguments
    ///
    /// * `new_agenda` - The stringified JSON for the new agenda
    /// * `timestamp` - The timestamp of the new agenda data
    ///
    /// # Panics
    ///
    /// Panics if the calling account is not authorized or if the timestamp is older than the current one.
    pub fn set_agenda(&mut self, new_agenda: String, timestamp: u64) {
        self.assert_data_setter();
        require!(
            timestamp > self.agenda_timestamp,
            "New data is older than the current data"
        );
        self.agenda = new_agenda;
        self.agenda_timestamp = timestamp;

        self.total_transactions += 1;
    }
}
