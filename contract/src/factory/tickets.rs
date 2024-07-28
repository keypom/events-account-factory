use crate::*;

/// Keypom Args struct to be sent to external contracts
#[derive(Serialize, Deserialize, Debug, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct KeypomArgs {
    pub account_id_field: Option<String>,
    pub drop_id_field: Option<String>,
    pub key_id_field: Option<String>,
    pub funder_id_field: Option<String>,
}

#[near_bindgen]
impl Contract {
    /// Retrieves the ticket data for a given drop ID.
    ///
    /// # Arguments
    ///
    /// * `drop_id` - The ID of the drop.
    ///
    /// # Returns
    ///
    /// Returns the ticket data associated with the drop ID.
    ///
    /// # Panics
    ///
    /// Panics if the drop ID does not exist.
    pub fn get_ticket_data(&self, drop_id: String) -> TicketType {
        self.ticket_data_by_id
            .get(&drop_id)
            .expect("No drop ID found")
    }

    /// Updates the ticket data for a given drop ID.
    /// Can only be called by an admin.
    ///
    /// # Arguments
    ///
    /// * `drop_id` - The ID of the drop to be updated.
    /// * `ticket_data` - The new ticket data.
    ///
    /// # Panics
    ///
    /// Panics if the caller is not an admin.
    pub fn update_ticket_data(&mut self, drop_id: String, ticket_data: TicketType) {
        self.assert_admin();
        self.ticket_data_by_id.insert(&drop_id, &ticket_data);
    }
}
