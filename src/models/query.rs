use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TransactionQuery {
    //Filtering parameters, all optional
    pub sender: Option<String>,
    pub receiver: Option<String>,
    pub amount: Option<i64>,

    //Pagination params, all optional
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

impl TransactionQuery {
    /// Get the limit with default value and maximum cap
    pub fn get_limit(&self) -> u64 {
        match self.limit {
            Some(limit_value) if limit_value > 50 => 50,
            Some(limit_value) => limit_value,
            None => 10,
        }
    }
    // Get the offset with default value
    pub fn get_offset(&self) -> u64 {
        match self.offset {
            Some(offset_value) => offset_value,
            None => 0,
        }
    }
}
