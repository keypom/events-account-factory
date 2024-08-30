use crate::*;

#[near_bindgen]
impl Contract {
    /// Allows users to get the stringified JSON for the alerts
    pub fn get_alerts(&self) -> String {
        self.alerts.clone()
    }

    /// Allows an admin to set the new version of the alerts
    ///
    /// # Arguments
    ///
    /// * `new_alerts` - The stringified JSON for the new alerts
    ///
    /// # Panics
    ///
    /// Panics if the calling account is not authorized.
    pub fn set_alerts(&mut self, new_alerts: String) {
        self.assert_data_setter();
        self.alerts = new_alerts;
    }
}
