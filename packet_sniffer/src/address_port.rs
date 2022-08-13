#[derive(PartialEq, Eq, Hash, Debug)]
pub struct AddressPort {
    pub address1: String,
    pub port1: u16,
}

impl AddressPort {

    pub fn new (address1: String, port1: u16) -> Self {
        AddressPort {
            address1,
            port1,
        }
    }
}

