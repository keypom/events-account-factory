use crate::*;

#[near]
impl Contract {
    /// Allows users to get the stringified JSON for the alerts
    pub fn get_alerts(&self) -> (String, u64) {
        (self.alerts.clone(), self.alerts_timestamp)
    }

    /// Allows an admin to set the new version of the alerts with a timestamp
    ///
    /// # Arguments
    ///
    /// * `new_alerts` - The stringified JSON for the new alerts
    /// * `timestamp` - The timestamp of the new alerts data
    ///
    /// # Panics
    ///
    /// Panics if the calling account is not authorized or if the timestamp is older than the current one.
    pub fn set_alerts(&mut self, new_alerts: String, timestamp: u64) {
        self.assert_data_setter();
        require!(
            timestamp > self.alerts_timestamp,
            "New data is older than the current data"
        );
        self.alerts = new_alerts;
        self.alerts_timestamp = timestamp;

        self.total_transactions += 1;
    }
}
