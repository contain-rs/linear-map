//! An optional implementation of serialization/deserialization. Reference
//! implementations used:
//!
//! - [Serialize][1].
//! - [Deserialize][2].
//!
//! [1]: https://github.com/serde-rs/serde/blob/97856462467db2e90cf368e407c7ebcc726a01a9/serde/src/ser/impls.rs#L601-L611
//! [2]: https://github.com/serde-rs/serde/blob/97856462467db2e90cf368e407c7ebcc726a01a9/serde/src/de/impls.rs#L694-L746

extern crate serde;

use super::LinearMap;

use self::serde::{Serialize, Serializer, Deserialize, Deserializer};
use self::serde::de::{Visitor, MapAccess, SeqAccess, Error};
use self::serde::ser::{SerializeMap, SerializeSeq};

use std::marker::PhantomData;
use std::fmt;

impl<K, V> Serialize for LinearMap<K, V>
    where K: Serialize + Ord,
          V: Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        let mut state = try!(serializer.serialize_map(Some(self.len())));
        for (k, v) in self {
            try!(state.serialize_entry(k, v));
        }
        state.end()
    }
}

#[allow(missing_docs)]
pub struct LinearMapVisitor<K, V> {
    marker: PhantomData<LinearMap<K, V>>,
}

impl<K, V> LinearMapVisitor<K, V> {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        LinearMapVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, K, V> Visitor<'de> for LinearMapVisitor<K, V>
    where K: Deserialize<'de> + Eq,
          V: Deserialize<'de>,
{
    type Value = LinearMap<K, V>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a LinearMap")
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E>
        where E: Error,
    {
        Ok(LinearMap::new())
    }

    #[inline]
    fn visit_map<Visitor>(self, mut visitor: Visitor) -> Result<Self::Value, Visitor::Error>
        where Visitor: MapAccess<'de>,
    {
        let mut values = LinearMap::with_capacity(visitor.size_hint().unwrap_or(0));

        while let Some((key, value)) = try!(visitor.next_entry()) {
            values.insert(key, value);
        }

        Ok(values)
    }
}

impl<'de, K, V> Deserialize<'de> for LinearMap<K, V>
    where K: Deserialize<'de> + Eq,
          V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<LinearMap<K, V>, D::Error>
        where D: Deserializer<'de>,
    {
        deserializer.deserialize_map(LinearMapVisitor::new())
    }
}
