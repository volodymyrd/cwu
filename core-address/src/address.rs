use crate::AddressResult;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};

/// The interface for a generic address.
pub trait Address:
    'static + Clone + Debug + Display + FromStr + Hash + PartialEq + Eq + Send + Sized + Sync
{
    type Format;
    type PublicKey;

    /// Returns the address corresponding to the given public key.
    fn from_public_key(public_key: &Self::PublicKey, format: &Self::Format) -> AddressResult<Self>;

    fn is_valid(address: &str) -> bool {
        Self::from_str(address).is_ok()
    }
}
