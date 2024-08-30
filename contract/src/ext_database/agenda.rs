use crate::*;

#[near_bindgen]
impl Contract {
    /// Allows users to get the stringified JSON for the agenda
    pub fn get_agenda(&self) -> String {
        self.agenda.clone()
    }

    /// Allows an admin to set the new version of the agenda
    ///
    /// # Arguments
    ///
    /// * `new_agenda` - The stringified JSON for the new agenda
    ///
    /// # Panics
    ///
    /// Panics if the calling account is not authorized.
    pub fn set_agenda(&mut self, new_agenda: String) {
        self.assert_admin();
        self.agenda = new_agenda;
    }
}
