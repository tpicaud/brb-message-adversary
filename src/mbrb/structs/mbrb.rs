pub struct Mbrb {
    pub n: u8,   // Total number of processes
    pub t: u8,    // Tolerance
    pub d: usize,
    pub id: u8,
    pub delivered_messages_file: String,
    pub pending_messages_file: String,
    pub saved_signatures_file: String,
    pub key_pair: (String,String)
}