use std::{mem, fmt};
use std::borrow::{Borrow,ToOwned};
use super::{LinearMap, Iter,IterMut,Keys,Values};

fn first_duplicate<'a,K,V,I>(iter: I) -> Option<usize>
where K: Eq+'a, V: 'a, I: Iterator<Item=&'a(K,V)>+Clone {
    iter.clone().enumerate().filter_map( |(i,&(ref needle,_))|
        iter.clone().take(i).position( |&(ref key,_)| key == needle )
    ).next()
}

pub struct LinearBorrowedMap<K: Eq, V> ( [(K,V)] );

impl<K: Eq, V> LinearBorrowedMap<K, V> {
    pub fn new(slice: &[(K,V)]) -> Result<&Self, &K> {
        unsafe{ match first_duplicate(slice.iter()) {
            None => Ok(Self::new_unchecked(slice)),
            Some(i) => Err(&slice[i].0),
        }}
    }
    /// Create a map with mutable values without checking for duplicate keys,
    ///
    /// This boils down to a transmute, while new takes O(n^2).
    pub unsafe fn new_unchecked(slice: &[(K,V)]) -> &Self {
        mem::transmute(slice)
    }
    pub fn new_mut(slice: &mut[(K,V)]) -> Result<&mut Self, &mut K> {
        unsafe{ match first_duplicate(slice.iter()) {
            None => Ok(Self::new_mut_unchecked(slice)),
            Some(i) => Err(&mut slice[i].0),
        }}
    }
    /// Create a map with mutable values without checking for duplicate keys,
    ///
    /// This boils down to a transmute, while new takes O(n^2).
    pub unsafe fn new_mut_unchecked(slice: &mut[(K,V)]) -> &mut Self {
        mem::transmute(slice)
    }


    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator yielding references to the map's keys and their corresponding values in
    /// arbitrary order.
    ///
    /// The iterator's item type is `(&K, &V)`.
    pub fn iter(&self) -> Iter<K, V> {
        Iter { iter: self.0.iter() }
    }

    /// Returns an iterator yielding references to the map's keys and mutable references to their
    /// corresponding values in arbitrary order.
    ///
    /// The iterator's item type is `(&K, &mut V)`.
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut { iter: self.0.iter_mut() }
    }

    /// Returns an iterator yielding references to the map's keys in arbitrary order.
    ///
    /// The iterator's item type is `&K`.
    pub fn keys(&self) -> Keys<K, V> {
        Keys { iter: self.iter() }
    }

    /// Returns an iterator yielding references to the map's values in arbitrary order.
    ///
    /// The iterator's item type is `&V`.
    pub fn values(&self) -> Values<K, V> {
        Values { iter: self.iter() }
    }

    /// Checks if the map contains a key that is equal to the given key.
    ///
    /// The given key may be any borrowed form of the map's key type, but `Eq` on the borrowed form
    /// *must* match that of the key type.
    pub fn contains_key<Q: ?Sized + Eq>(&self, key: &Q) -> bool
    where K: Borrow<Q> {
        self.get(key).is_some()
    }

    /// Returns a reference to the value in the map whose key is equal to the given key.
    ///
    /// Returns `None` if the map contains no such key.
    ///
    /// The given key may be any borrowed form of the map's key type, but `Eq` on the borrowed form
    /// *must* match that of the key type.
    pub fn get<Q: ?Sized + Eq>(&self, key: &Q) -> Option<&V>
    where K: Borrow<Q> {
        self.iter().find(|&e| e.0.borrow() == key ).map(|(_,v)| v )
    }

    /// Returns a mutable reference to the value in the map whose key is equal to the given key.
    ///
    /// Returns `None` if the map contains no such key.
    ///
    /// The given key may be any borrowed form of the map's key type, but `Eq` on the borrowed form
    /// *must* match that of the key type.
    pub fn get_mut<Q: ?Sized + Eq>(&mut self, key: &Q) -> Option<&mut V>
    where K: Borrow<Q> {
        self.iter_mut().find(|&(k,_)| k.borrow() == key ).map(|(_,v)| v )
    }
}

impl<K: Eq, V> AsRef<[(K,V)]> for LinearBorrowedMap<K,V> {
    fn as_ref(&self) -> &[(K,V)] {
        &self.0
    }
}

impl<K: Eq+Clone, V: Clone> ToOwned for LinearBorrowedMap<K,V> {
    type Owned = LinearMap<K,V>;
    fn to_owned(&self) -> Self::Owned {
        LinearMap{ storage: self.as_ref().to_vec() }
    }
}

impl<'a, K: Eq, V> IntoIterator for &'a LinearBorrowedMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;
    fn into_iter(self) -> Iter<'a, K, V> {
        self.iter()
    }
}

impl<'a, K: Eq, V> IntoIterator for &'a mut LinearBorrowedMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> IterMut<'a, K, V> {
        self.iter_mut()
    }
}

impl<K: Eq+fmt::Debug, V: fmt::Debug> fmt::Debug for LinearBorrowedMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self).finish()
    }
}

impl<K: Eq, V: PartialEq> PartialEq for LinearBorrowedMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len()
        && self.iter().all(|(k, v)| other.get(k) == Some(v) )
    }
}

impl<K: Eq, V: Eq> Eq for LinearBorrowedMap<K, V>
    {}
