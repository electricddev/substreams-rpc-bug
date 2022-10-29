// ------------------------------------------------
//      store_volumes
// ------------------------------------------------

pub mod store_volumes {
    pub mod collection {
        pub fn construct(
            contract_address: &String,
            marketplace: &String,
            payment_token: &String,
        ) -> String {
            format!(
                "collection:{}:{}:{}",
                contract_address, marketplace, payment_token
            )
        }

        /// (contract address, marketplace, payment token)
        pub fn destruct(key: &String) -> (String, String, String) {
            (
                key.split(":").nth(1).unwrap().to_owned(),
                key.split(":").nth(2).unwrap().to_owned(),
                key.split(":").nth(3).unwrap().to_owned(),
            )
        }
    }
}
