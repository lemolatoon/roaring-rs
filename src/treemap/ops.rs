use std::collections::btree_map::Entry;
use std::mem;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Sub, SubAssign};

use crate::RoaringTreemap;

impl RoaringTreemap {
    /// Unions in-place with the specified other bitmap.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use roaring::RoaringTreemap;
    ///
    /// let mut rb1: RoaringTreemap = (1..4).collect();
    /// let rb2: RoaringTreemap = (3..5).collect();
    /// let rb3: RoaringTreemap = (1..5).collect();
    ///
    /// rb1.union_with(&rb2);
    ///
    /// assert_eq!(rb1, rb3);
    /// ```
    ///
    /// Can also be done via the `BitOr` operator.
    ///
    /// ```rust
    /// use roaring::RoaringTreemap;
    ///
    /// let mut rb1: RoaringTreemap = (1..4).collect();
    /// let rb2: RoaringTreemap = (3..5).collect();
    /// let rb3: RoaringTreemap = (1..5).collect();
    ///
    /// let rb1 = rb1 | rb2;
    ///
    /// assert_eq!(rb1, rb3);
    /// ```
    #[deprecated(
        since = "0.6.7",
        note = "Please use the `BitOrAssign::bitor_assign` ops method instead"
    )]
    pub fn union_with(&mut self, other: &RoaringTreemap) {
        BitOrAssign::bitor_assign(self, other)
    }

    /// Intersects in-place with the specified other bitmap.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use roaring::RoaringTreemap;
    ///
    /// let mut rb1: RoaringTreemap = (1..4).collect();
    /// let rb2: RoaringTreemap = (3..5).collect();
    /// let rb3: RoaringTreemap = (3..4).collect();
    ///
    /// rb1.intersect_with(&rb2);
    ///
    /// assert_eq!(rb1, rb3);
    /// ```
    ///
    /// Can also be done via the `BitAnd` operator.
    ///
    /// ```rust
    /// use roaring::RoaringTreemap;
    ///
    /// let mut rb1: RoaringTreemap = (1..4).collect();
    /// let rb2: RoaringTreemap = (3..5).collect();
    /// let rb3: RoaringTreemap = (3..4).collect();
    ///
    /// let rb1 = rb1 & rb2;
    ///
    /// assert_eq!(rb1, rb3);
    /// ```
    #[deprecated(
        since = "0.6.7",
        note = "Please use the `BitAndAssign::bitand_assign` ops method instead"
    )]
    pub fn intersect_with(&mut self, other: &RoaringTreemap) {
        BitAndAssign::bitand_assign(self, other)
    }

    /// Removes all values in the specified other bitmap from self, in-place.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use roaring::RoaringTreemap;
    ///
    /// let mut rb1: RoaringTreemap = (1..4).collect();
    /// let rb2: RoaringTreemap = (3..5).collect();
    /// let rb3: RoaringTreemap = (1..3).collect();
    ///
    /// rb1.difference_with(&rb2);
    ///
    /// assert_eq!(rb1, rb3);
    /// ```
    ///
    /// Can also be done via the `Sub` operator.
    ///
    /// ```rust
    /// use roaring::RoaringTreemap;
    ///
    /// let mut rb1: RoaringTreemap = (1..4).collect();
    /// let rb2: RoaringTreemap = (3..5).collect();
    /// let rb3: RoaringTreemap = (1..3).collect();
    ///
    /// let rb1 = rb1 - rb2;
    ///
    /// assert_eq!(rb1, rb3);
    /// ```
    pub fn difference_with(&mut self, other: &RoaringTreemap) {
        let mut keys_to_remove: Vec<u32> = Vec::new();
        for (key, self_rb) in &mut self.map {
            if let Some(other_rb) = other.map.get(key) {
                self_rb.difference_with(other_rb);
                if self_rb.is_empty() {
                    keys_to_remove.push(*key);
                }
            }
        }

        for key in keys_to_remove {
            self.map.remove(&key);
        }
    }

    /// Replaces this bitmap with one that is equivalent to `self XOR other`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use roaring::RoaringTreemap;
    ///
    /// let mut rb1: RoaringTreemap = (1..4).collect();
    /// let rb2: RoaringTreemap = (3..6).collect();
    /// let rb3: RoaringTreemap = (1..3).chain(4..6).collect();
    ///
    /// rb1.symmetric_difference_with(&rb2);
    ///
    /// assert_eq!(rb1, rb3);
    /// ```
    ///
    /// Can also be done via the `BitXor` operator.
    ///
    /// ```rust
    /// use roaring::RoaringTreemap;
    ///
    /// let mut rb1: RoaringTreemap = (1..4).collect();
    /// let rb2: RoaringTreemap = (3..6).collect();
    /// let rb3: RoaringTreemap = (1..3).chain(4..6).collect();
    ///
    /// let rb1 = rb1 ^ rb2;
    ///
    /// assert_eq!(rb1, rb3);
    /// ```
    pub fn symmetric_difference_with(&mut self, other: &RoaringTreemap) {
        let mut keys_to_remove: Vec<u32> = Vec::new();
        for (key, other_rb) in &other.map {
            match self.map.entry(*key) {
                Entry::Vacant(ent) => {
                    ent.insert(other_rb.clone());
                }
                Entry::Occupied(mut ent) => {
                    ent.get_mut().symmetric_difference_with(other_rb);
                    if ent.get().is_empty() {
                        keys_to_remove.push(*key);
                    }
                }
            };
        }

        for key in keys_to_remove {
            self.map.remove(&key);
        }
    }
}

impl BitOr<RoaringTreemap> for RoaringTreemap {
    type Output = RoaringTreemap;

    /// This is equivalent to an `union` between both maps.
    fn bitor(mut self, rhs: RoaringTreemap) -> RoaringTreemap {
        BitOrAssign::bitor_assign(&mut self, rhs);
        self
    }
}

impl BitOr<&RoaringTreemap> for RoaringTreemap {
    type Output = RoaringTreemap;

    /// This is equivalent to an `union` between both maps.
    fn bitor(mut self, rhs: &RoaringTreemap) -> RoaringTreemap {
        BitOrAssign::bitor_assign(&mut self, rhs);
        self
    }
}

impl BitOr<RoaringTreemap> for &RoaringTreemap {
    type Output = RoaringTreemap;

    /// This is equivalent to an `union` between both maps.
    fn bitor(self, rhs: RoaringTreemap) -> RoaringTreemap {
        BitOr::bitor(rhs, self)
    }
}

impl BitOr<&RoaringTreemap> for &RoaringTreemap {
    type Output = RoaringTreemap;

    /// This is equivalent to an `union` between both maps.
    fn bitor(self, rhs: &RoaringTreemap) -> RoaringTreemap {
        if self.len() <= rhs.len() {
            BitOr::bitor(rhs.clone(), self)
        } else {
            BitOr::bitor(self.clone(), rhs)
        }
    }
}

impl BitOrAssign<RoaringTreemap> for RoaringTreemap {
    /// This is equivalent to an `union` between both maps.
    fn bitor_assign(&mut self, mut rhs: RoaringTreemap) {
        // We make sure that we apply the union operation on the biggest map.
        if self.len() < rhs.len() {
            mem::swap(self, &mut rhs);
        }

        for (key, other_rb) in rhs.map {
            match self.map.entry(key) {
                Entry::Vacant(ent) => {
                    ent.insert(other_rb);
                }
                Entry::Occupied(mut ent) => {
                    BitOrAssign::bitor_assign(ent.get_mut(), other_rb);
                }
            }
        }
    }
}

impl BitOrAssign<&RoaringTreemap> for RoaringTreemap {
    /// This is equivalent to an `union` between both maps.
    fn bitor_assign(&mut self, rhs: &RoaringTreemap) {
        for (key, other_rb) in &rhs.map {
            match self.map.entry(*key) {
                Entry::Vacant(ent) => {
                    ent.insert(other_rb.clone());
                }
                Entry::Occupied(mut ent) => {
                    BitOrAssign::bitor_assign(ent.get_mut(), other_rb);
                }
            }
        }
    }
}

impl BitAnd<RoaringTreemap> for RoaringTreemap {
    type Output = RoaringTreemap;

    /// This is equivalent to an `intersection` between both maps.
    fn bitand(mut self, rhs: RoaringTreemap) -> RoaringTreemap {
        BitAndAssign::bitand_assign(&mut self, rhs);
        self
    }
}

impl BitAnd<&RoaringTreemap> for RoaringTreemap {
    type Output = RoaringTreemap;

    /// This is equivalent to an `intersection` between both maps.
    fn bitand(mut self, rhs: &RoaringTreemap) -> RoaringTreemap {
        BitAndAssign::bitand_assign(&mut self, rhs);
        self
    }
}

impl BitAnd<RoaringTreemap> for &RoaringTreemap {
    type Output = RoaringTreemap;

    /// This is equivalent to an `intersection` between both maps.
    fn bitand(self, rhs: RoaringTreemap) -> RoaringTreemap {
        BitAnd::bitand(rhs, self)
    }
}

impl BitAnd<&RoaringTreemap> for &RoaringTreemap {
    type Output = RoaringTreemap;

    /// This is equivalent to an `intersection` between both maps.
    fn bitand(self, rhs: &RoaringTreemap) -> RoaringTreemap {
        if rhs.len() < self.len() {
            BitAnd::bitand(self.clone(), rhs)
        } else {
            BitAnd::bitand(rhs.clone(), self)
        }
    }
}

impl BitAndAssign<RoaringTreemap> for RoaringTreemap {
    /// This is equivalent to an `intersection` between both maps.
    fn bitand_assign(&mut self, mut rhs: RoaringTreemap) {
        // We make sure that we apply the intersection operation on the smallest map.
        if rhs.len() < self.len() {
            mem::swap(self, &mut rhs);
        }

        BitAndAssign::bitand_assign(self, &rhs)
    }
}

impl BitAndAssign<&RoaringTreemap> for RoaringTreemap {
    /// This is equivalent to an `intersection` between both maps.
    fn bitand_assign(&mut self, rhs: &RoaringTreemap) {
        let mut keys_to_remove: Vec<u32> = Vec::new();
        for (key, self_rb) in &mut self.map {
            match rhs.map.get(key) {
                Some(other_rb) => {
                    BitAndAssign::bitand_assign(self_rb, other_rb);
                    if self_rb.is_empty() {
                        keys_to_remove.push(*key);
                    }
                }
                None => keys_to_remove.push(*key),
            }
        }

        for key in keys_to_remove {
            self.map.remove(&key);
        }
    }
}

impl Sub<RoaringTreemap> for RoaringTreemap {
    type Output = RoaringTreemap;

    fn sub(mut self, rhs: RoaringTreemap) -> RoaringTreemap {
        self.difference_with(&rhs);
        self
    }
}

impl<'a> Sub<&'a RoaringTreemap> for RoaringTreemap {
    type Output = RoaringTreemap;

    fn sub(mut self, rhs: &'a RoaringTreemap) -> RoaringTreemap {
        self.difference_with(rhs);
        self
    }
}

impl<'a> Sub<RoaringTreemap> for &'a RoaringTreemap {
    type Output = RoaringTreemap;

    fn sub(self, rhs: RoaringTreemap) -> RoaringTreemap {
        self.clone() - rhs
    }
}

impl<'a, 'b> Sub<&'a RoaringTreemap> for &'b RoaringTreemap {
    type Output = RoaringTreemap;

    fn sub(self, rhs: &'a RoaringTreemap) -> RoaringTreemap {
        self.clone() - rhs
    }
}

impl SubAssign<RoaringTreemap> for RoaringTreemap {
    fn sub_assign(&mut self, rhs: RoaringTreemap) {
        self.difference_with(&rhs)
    }
}

impl<'a> SubAssign<&'a RoaringTreemap> for RoaringTreemap {
    fn sub_assign(&mut self, rhs: &'a RoaringTreemap) {
        self.difference_with(rhs)
    }
}

impl BitXor<RoaringTreemap> for RoaringTreemap {
    type Output = RoaringTreemap;

    fn bitxor(mut self, rhs: RoaringTreemap) -> RoaringTreemap {
        self.symmetric_difference_with(&rhs);
        self
    }
}

impl<'a> BitXor<&'a RoaringTreemap> for RoaringTreemap {
    type Output = RoaringTreemap;

    fn bitxor(mut self, rhs: &'a RoaringTreemap) -> RoaringTreemap {
        self.symmetric_difference_with(rhs);
        self
    }
}

impl<'a> BitXor<RoaringTreemap> for &'a RoaringTreemap {
    type Output = RoaringTreemap;

    fn bitxor(self, rhs: RoaringTreemap) -> RoaringTreemap {
        rhs ^ self
    }
}

impl<'a, 'b> BitXor<&'a RoaringTreemap> for &'b RoaringTreemap {
    type Output = RoaringTreemap;

    fn bitxor(self, rhs: &'a RoaringTreemap) -> RoaringTreemap {
        self.clone() ^ rhs
    }
}

impl BitXorAssign<RoaringTreemap> for RoaringTreemap {
    fn bitxor_assign(&mut self, rhs: RoaringTreemap) {
        self.symmetric_difference_with(&rhs)
    }
}

impl<'a> BitXorAssign<&'a RoaringTreemap> for RoaringTreemap {
    fn bitxor_assign(&mut self, rhs: &'a RoaringTreemap) {
        self.symmetric_difference_with(rhs)
    }
}
