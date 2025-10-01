use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TransactionQuery {
    //Filtering parameters, all optional
    pub sender: Option<String>,
    pub receiver: Option<String>,
    pub amount: Option<u64>,

    //Pagination params, all optional
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

impl TransactionQuery {
    /// Get the limit with default value and maximum cap
    pub fn get_limit(&self) -> u64 {
        //todo
    }
    // Get the offset with default value
    pub fn get_offset(&self) -> u64 {
        //todo
    }
}
