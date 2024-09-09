use crate::*;

#[near_bindgen]
impl Contract {
    /// Query for the information of a specific attendee key.
    ///
    /// # Arguments
    ///
    /// * `public_key` - The public key of the attendee key to retrieve information for.
    ///
    /// # Returns
    ///
    /// An `Option` containing the `AttendeeTicketInformation` if the ticket is found, otherwise `None`.
    pub fn get_key_information(&self, key: PublicKey) -> Option<AttendeeTicketInformation> {
        self.attendee_ticket_by_pk.get(&key)
    }
}
