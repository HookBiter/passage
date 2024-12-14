use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Method {
    Connect,
    Disconnect,
    Pass,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::to_string;

    #[test]
    fn method_serialize_test() {
        let m = Method::Connect;
        let ser_m = to_string(&m);
        assert_eq!(ser_m.unwrap(), "\"Connect\"");
    }
}
