    pub mod map {
        #[allow(unused_imports)]
        use builtin::*;
        #[allow(unused_imports)]
        use builtin_macros::*;
        #[allow(unused_imports)]
        use crate::pervasive::*;
        #[allow(unused_imports)]
        use crate::pervasive::set::*;
        use core::marker;
        #[doc = " `Map<K, V>` is an abstract map type for specifications."]
        #[doc =
          " To use a \"map\" in compiled code, use an `exec` type like HashMap (TODO)"]
        #[doc = " that has a `Map<K, V>` as its specification type."]
        #[doc = ""]
        #[doc =
          " An object `map: Map<K, V>` has a _domain_, a set of keys given by [`map.dom()`](Map::dom),"]
        #[doc =
          " and a mapping for keys in the domain to values, given by [`map[key]`](Map::index)."]
        #[doc =
          " Alternatively, a map can be thought of as a set of `(K, V)` pairs where each key"]
        #[doc = " appears in at most entry."]
        #[doc = ""]
        #[doc = " In general, a map might be infinite."]
        #[doc =
          " To work specifically with finite maps, see the [`self.finite()`](Set::finite) predicate."]
        #[doc = ""]
        #[doc = " Maps can be constructed in a few different ways:"]
        #[doc = "  * [`Map::empty()`] constructs an empty map."]
        #[doc =
          "  * [`Map::new`] and [`Map::total`] construct a map given functions that specify its domain and the mapping"]
        #[doc = "     from keys to values (a _map comprehension_)."]
        #[doc =
          "  * The [`map!`] macro, to construct small maps of a fixed size."]
        #[doc =
          "  * By manipulating an existing map with [`Map::insert`] or [`Map::remove`]."]
        #[doc = ""]
        #[doc =
          " To prove that two maps are equal, it is usually easiest to use the [`assert_maps_equal!`] macro."]
        #[verifier(external_body)]
        #[proof]
        pub struct Map<#[verifier(maybe_negative)] K,
                       #[verifier(strictly_positive)] V> {
            dummy: marker::PhantomData<(K, V)>,
        }
        impl <K, V> Map<K, V> {
            #[doc = " An empty map."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn empty() -> Map<K, V> {
                ::core::panicking::panic("not implemented")
            }
            #[doc =
              " Gives a `Map<K, V>` whose domain contains every key, and maps each key"]
            #[doc = " to the value given by `fv`."]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn total(fv: impl Fn(K) -> V) -> Map<K, V> {
                Set::full().mk_map(fv)
            }
            #[doc =
              " Gives a `Map<K, V>` whose domain is given by the boolean predicate on keys `fk`,"]
            #[doc = " and maps each key to the value given by `fv`."]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn new(fk: impl Fn(K) -> bool, fv: impl Fn(K) -> V)
             -> Map<K, V> {
                Set::new(fk).mk_map(fv)
            }
            #[doc = " The domain of the map as a set."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn dom(self) -> Set<K> {
                ::core::panicking::panic("not implemented")
            }
            #[doc = " Gets the value that the given key `key` maps to."]
            #[doc =
              " For keys not in the domain, the result is meaningless and arbitrary."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn index(self, key: K) -> V {
                ::builtin::recommends([self.dom().contains(key)]);
                ::core::panicking::panic("not implemented")
            }
            #[doc = " `[]` operator, synonymous with `index`"]
            #[verifier(inline)]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn spec_index(self, key: K) -> V {
                ::builtin::recommends([self.dom().contains(key)]);
                self.index(key)
            }
            #[doc = " Inserts the given (key, value) pair into the map."]
            #[doc = ""]
            #[doc =
              " If the key is already present from the map, then its existing value is overwritten"]
            #[doc = " by the new value."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn insert(self, key: K, value: V) -> Map<K, V> {
                ::core::panicking::panic("not implemented")
            }
            #[doc =
              " Removes the given key and its associated value from the map."]
            #[doc = ""]
            #[doc =
              " If the key is already absent from the map, then the map is left unchanged."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn remove(self, key: K) -> Map<K, V> {
                ::core::panicking::panic("not implemented")
            }
            #[doc =
              " Returns true if the two maps are pointwise equal, i.e.,"]
            #[doc =
              " they have the same domains and the corresponding values are equal"]
            #[doc =
              " for each key. This is equivalent to the maps being actually equal"]
            #[doc = " by [`axiom_map_ext_equal`]."]
            #[doc = ""]
            #[doc =
              " To prove that two maps are equal via extensionality, it is generally easier"]
            #[doc =
              " to use the [`assert_maps_equal!`] macro, rather than using `ext_equal` directly."]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn ext_equal(self, m2: Map<K, V>) -> bool {
                (self.dom().ext_equal(m2.dom())) &&
                    ((::builtin::forall(|k: K|
                                            #[auto_trigger] (::builtin::imply(self.dom().contains(k),
                                                                              ::builtin::equal(self.spec_index(k),
                                                                                               m2.spec_index(k)))))))
            }
            #[doc =
              " Returns true if the key `k` is in the domain of `self`, and it maps to the value `v`."]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn contains_pair(self, k: K, v: V) -> bool {
                self.dom().contains(k) &&
                    ::builtin::equal(self.spec_index(k), v)
            }
            #[doc =
              " Returns true if `m1` is _contained in_ `m2`, i.e., the domain of `m1` is a subset"]
            #[doc =
              " of the domain of `m2`, and they agree on all values in `m1`."]
            #[doc = ""]
            #[doc = " ## Example"]
            #[doc = ""]
            #[doc = " ```rust"]
            #[doc = " assert("]
            #[doc =
              "    map![1 => 10, 2 => 11].le(map![1 => 10, 2 => 11, 3 => 12])"]
            #[doc = " );"]
            #[doc = " ```"]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn le(self, m2: Self) -> bool {
                ::builtin::forall(|k: K|
                                      ::builtin::imply(#[trigger] self.dom().contains(k),
                                                       #[trigger] m2.dom().contains(k)
                                                           &&
                                                           ::builtin::equal(self.spec_index(k),
                                                                            m2.spec_index(k))))
            }
            #[doc = " Gives the union of two maps, defined as:"]
            #[doc = "  * The domain is the union of the two input maps."]
            #[doc =
              "  * For a given key in _both_ input maps, it maps to the same value that it maps to in the _right_ map (`m2`)."]
            #[doc =
              "  * For any other key in either input map (but not both), it maps to the same value"]
            #[doc = "    as it does in that map."]
            #[doc = ""]
            #[doc = " ## Example"]
            #[doc = ""]
            #[doc = " ```rust"]
            #[doc = " assert_maps_equal!("]
            #[doc =
              "    map![1 => 10, 2 => 11].union_prefer_right(map![1 => 20, 3 => 13]),"]
            #[doc = "    map![1 => 20, 2 => 11, 3 => 13],"]
            #[doc = " );"]
            #[doc = " ```"]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn union_prefer_right(self, m2: Self) -> Self {
                Self::new(::builtin::closure_to_fn_spec(|k: K|
                                                            self.dom().contains(k)
                                                                ||
                                                                m2.dom().contains(k)),
                          ::builtin::closure_to_fn_spec(|k: K|
                                                            if m2.dom().contains(k)
                                                               {
                                                                m2.spec_index(k)
                                                            } else {
                                                                self.spec_index(k)
                                                            }))
            }
            #[doc =
              " Removes the given keys and their associated values from the map."]
            #[doc = ""]
            #[doc =
              " Ignores any key in `keys` which is not in the domain of `self`."]
            #[doc = ""]
            #[doc = " ## Example"]
            #[doc = ""]
            #[doc = " ```rust"]
            #[doc = " assert_maps_equal!("]
            #[doc =
              "    map![1 => 10, 2 => 11, 3 => 12].remove_keys(set!{2, 3, 4}),"]
            #[doc = "    map![1 => 10],"]
            #[doc = " );"]
            #[doc = " ```"]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn remove_keys(self, keys: Set<K>) -> Self {
                Self::new(::builtin::closure_to_fn_spec(|k: K|
                                                            self.dom().contains(k)
                                                                &&
                                                                !keys.contains(k)),
                          ::builtin::closure_to_fn_spec(|k: K|
                                                            self.spec_index(k)))
            }
            #[doc =
              " Returns `true` if the two given maps agree on all keys that their domains"]
            #[doc = " share in common."]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn agrees(self, m2: Self) -> bool {
                ::builtin::forall(|k|
                                      #[auto_trigger] (::builtin::imply(self.dom().contains(k)
                                                                            &&
                                                                            m2.dom().contains(k),
                                                                        ::builtin::equal(self.spec_index(k),
                                                                                         m2.spec_index(k)))))
            }
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            #[verifier(returns(proof))]
            #[proof]
            pub fn tracked_empty() -> Self {
                ::builtin::ensures(|out_v: Self|
                                       [::builtin::equal(out_v,
                                                         Map::empty())]);
                ::core::panicking::panic("not implemented");
            }
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            #[proof]
            pub fn tracked_insert(#[proof] &mut self, key: K,
                                  #[proof] value: V) {
                ::builtin::ensures([::builtin::equal(*self,
                                                     Map::insert(*old(self),
                                                                 key,
                                                                 value))]);
                ::core::panicking::panic("not implemented");
            }
            #[doc = " todo fill in documentation"]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            #[verifier(returns(proof))]
            #[proof]
            pub fn tracked_remove(#[proof] &mut self, key: K) -> V {
                ::builtin::requires([old(self).dom().contains(key)]);
                ::builtin::ensures(|v: V|
                                       [::builtin::equal(*self,
                                                         Map::remove(*old(self),
                                                                     key)),
                                        ::builtin::equal(v,
                                                         old(self).spec_index(key))]);
                ::core::panicking::panic("not implemented");
            }
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            #[verifier(returns(proof))]
            #[proof]
            pub fn tracked_map_keys<J>(#[proof] old_map: Map<K, V>,
                                       key_map: Map<J, K>) -> Map<J, V> {
                ::builtin::requires([::builtin::forall(|j|
                                                           ::builtin::imply(key_map.dom().contains(j),
                                                                            old_map.dom().contains(key_map.index(j)))),
                                     ::builtin::forall(|j1, j2|
                                                           ::builtin::imply(!equal(j1,
                                                                                   j2)
                                                                                &&
                                                                                key_map.dom().contains(j1)
                                                                                &&
                                                                                key_map.dom().contains(j2),
                                                                            !equal(key_map.index(j1),
                                                                                   key_map.index(j2))))]);
                ::builtin::ensures(|new_map: Map<J, V>|
                                       [::builtin::forall(|j|
                                                              ::builtin::spec_eq((#[trigger] new_map.dom().contains(j)),
                                                                                 (key_map.dom().contains(j)))),
                                        ::builtin::forall(|j|
                                                              ::builtin::imply(key_map.dom().contains(j),
                                                                               new_map.dom().contains(j)
                                                                                   &&
                                                                                   ::builtin::equal(#[trigger] new_map.index(j),
                                                                                                    old_map.index(key_map.index(j)))))]);
                ::core::panicking::panic("not implemented");
            }
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_map_empty<K, V>() {
            ::builtin::ensures([::builtin::equal(#[trigger] Map::<K,
                                                                  V>::empty().dom(),
                                                 Set::empty())]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_map_insert_domain<K, V>(m: Map<K, V>, key: K, value: V) {
            ::builtin::ensures([::builtin::equal(#[trigger] m.insert(key,
                                                                     value).dom(),
                                                 m.dom().insert(key))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_map_insert_same<K, V>(m: Map<K, V>, key: K, value: V) {
            ::builtin::ensures([::builtin::equal(#[trigger] m.insert(key,
                                                                     value).spec_index(key),
                                                 value)]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_map_insert_different<K,
                                          V>(m: Map<K, V>, key1: K, key2: K,
                                             value: V) {
            ::builtin::requires([m.dom().contains(key1),
                                 !::builtin::equal(key1, key2)]);
            ::builtin::ensures([::builtin::equal(m.insert(key2,
                                                          value).spec_index(key1),
                                                 m.spec_index(key1))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_map_remove_domain<K, V>(m: Map<K, V>, key: K) {
            ::builtin::ensures([::builtin::equal(#[trigger] m.remove(key).dom(),
                                                 m.dom().remove(key))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_map_remove_different<K,
                                          V>(m: Map<K, V>, key1: K, key2: K) {
            ::builtin::requires([m.dom().contains(key1),
                                 !::builtin::equal(key1, key2)]);
            ::builtin::ensures([::builtin::equal(m.remove(key2).spec_index(key1),
                                                 m.spec_index(key1))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_map_ext_equal<K, V>(m1: Map<K, V>, m2: Map<K, V>) {
            ::builtin::ensures([::builtin::spec_eq(m1.ext_equal(m2),
                                                   (::builtin::equal(m1,
                                                                     m2)))]);
        }
        #[doc(hidden)]
        #[macro_export]
        macro_rules! map_internal {
            [$($key : expr => $value : expr), * $(,) ?] =>
            {
                $crate :: pervasive :: map :: Map :: empty()
                $(.insert($key, $value)) *
            }
        }
        #[doc =
          " Create a map using syntax like `map![key1 => val1, key2 => val, ...]`."]
        #[doc = ""]
        #[doc =
          " This is equivalent to `Map::empty().insert(key1, val1).insert(key2, val2)...`."]
        #[doc = ""]
        #[doc =
          " Note that this does _not_ require all keys to be distinct. In the case that two"]
        #[doc =
          " or more keys are equal, the resulting map uses the value of the rightmost entry."]
        #[macro_export]
        macro_rules! map {
            [$($tail : tt) *] =>
            {
                :: builtin_macros :: verus_proof_macro_exprs!
                ($crate :: pervasive :: map :: map_internal! ($($tail) *))
            } ;
        }
        #[doc(hidden)]
        #[verifier(inline)]
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn check_argument_is_map<K, V>(m: Map<K, V>) -> Map<K, V> { m }
        #[doc(hidden)]
        pub use map_internal;
        pub use map;
        #[doc =
          " Prove two maps `map1` and `map2` are equal by proving that their values are equal at each key."]
        #[doc = ""]
        #[doc =
          " More precisely, `assert_maps_equal!` requires that for each key `k`:"]
        #[doc =
          "  * `map1` contains `k` in its domain if and only if `map2` does (`map1.dom().contains(k) <==> map2.dom().contains(k)`)"]
        #[doc =
          "  * If they contain `k` in their domains, then their values are equal (`map1.dom().contains(k) && map2.dom().contains(k) ==> map1[k] === map2[k]`)"]
        #[doc = ""]
        #[doc =
          " The property that equality follows from these facts is often called _extensionality_."]
        #[doc = ""]
        #[doc = " `assert_maps_equal!` can handle many trivial-looking"]
        #[doc = " identities without any additional help:"]
        #[doc = ""]
        #[doc = " ```rust"]
        #[doc = " proof fn insert_remove(m: Map<int, int>, k: int, v: int)"]
        #[doc = "     requires !m.dom().contains(k)"]
        #[doc = "     ensures m.insert(k, v).remove(k) === m"]
        #[doc = " {"]
        #[doc = "     let m2 = m.insert(k, v).remove(k);"]
        #[doc = "     assert_maps_equal!(m, m2);"]
        #[doc = "     assert(m === m2);"]
        #[doc = " }"]
        #[doc = " ```"]
        #[doc = " "]
        #[doc =
          " For more complex cases, a proof may be required for each key:"]
        #[doc = ""]
        #[doc = " ```rust"]
        #[doc = " proof fn bitvector_maps() {"]
        #[doc = "     let m1 = Map::<u64, u64>::new("]
        #[doc = "         |key: u64| key & 31 == key,"]
        #[doc = "         |key: u64| key | 5);"]
        #[doc = " "]
        #[doc = "     let m2 = Map::<u64, u64>::new("]
        #[doc = "         |key: u64| key < 32,"]
        #[doc = "         |key: u64| 5 | key);"]
        #[doc = " "]
        #[doc = "     assert_maps_equal!(m1, m2, key => {"]
        #[doc =
          "         // Show that the domains of m1 and m2 are the same by showing their predicates"]
        #[doc = "         // are equivalent."]
        #[doc =
          "         assert_bit_vector((key & 31 == key) <==> (key < 32));"]
        #[doc = " "]
        #[doc =
          "         // Show that the values are the same by showing that these expressions"]
        #[doc = "         // are equivalent."]
        #[doc = "         assert_bit_vector(key | 5 == 5 | key);"]
        #[doc = "     });"]
        #[doc = " }"]
        #[doc = " ```"]
        #[macro_export]
        macro_rules! assert_maps_equal {
            [$($tail : tt) *] =>
            {
                :: builtin_macros :: verus_proof_macro_exprs!
                ($crate :: pervasive :: map :: assert_maps_equal_internal!
                 ($($tail) *))
            } ;
        }
        #[macro_export]
        #[doc(hidden)]
        macro_rules! assert_maps_equal_internal {
            ($m1 : expr, $m2 : expr $(,) ?) =>
            { assert_maps_equal_internal! ($m1, $m2, key => { }) } ;
            ($m1 : expr, $m2 : expr, $k : ident $(: $t : ty) ? => $bblock :
             block) =>
            {
                #[spec] let m1 = $crate :: pervasive :: map ::
                check_argument_is_map($m1) ; #[spec] let m2 = $crate ::
                pervasive :: map :: check_argument_is_map($m2) ; :: builtin ::
                assert_by(:: builtin :: equal(m1, m2),
                          {
                              :: builtin ::
                              assert_forall_by(| $k $(: $t) ? |
                                               {
                                                   :: builtin ::
                                                   ensures([:: builtin ::
                                                            imply(#[trigger]
                                                                  m1.dom().contains($k),
                                                                  m2.dom().contains($k))
                                                            && :: builtin ::
                                                            imply(m2.dom().contains($k),
                                                                  m1.dom().contains($k))
                                                            && :: builtin ::
                                                            imply(m1.dom().contains($k)
                                                                  &&
                                                                  m2.dom().contains($k),
                                                                  :: builtin
                                                                  ::
                                                                  equal(m1.index($k),
                                                                        m2.index($k)))])
                                                   ; { $bblock }
                                               }) ; $crate :: pervasive ::
                              assert(m1.ext_equal(m2)) ;
                          }) ;
            }
        }
        #[doc(hidden)]
        pub use assert_maps_equal_internal;
        pub use assert_maps_equal;
        impl <K, V> Map<K, V> {
            #[proof]
            pub fn tracked_map_keys_in_place(#[proof] &mut self,
                                             key_map: Map<K, K>) {
                requires([forall(|j|
                                     imply(key_map.dom().contains(j),
                                           old(self).dom().contains(key_map.index(j)))),
                          forall(|j1, j2|
                                     imply(!equal(j1, j2) &&
                                               key_map.dom().contains(j1) &&
                                               key_map.dom().contains(j2),
                                           !equal(key_map.index(j1),
                                                  key_map.index(j2))))]);
                ensures([forall(|j|
                                    #[trigger] self.dom().contains(j) ==
                                        key_map.dom().contains(j)),
                         forall(|j|
                                    imply(key_map.dom().contains(j),
                                          self.dom().contains(j) &&
                                              equal(#[trigger] self.index(j),
                                                    old(self).index(key_map.index(j)))))]);
                #[proof]
                let mut tmp = Self::tracked_empty();
                crate::pervasive::modes::tracked_swap(&mut tmp, self);
                #[proof]
                let mut tmp = Self::tracked_map_keys(tmp, key_map);
                crate::pervasive::modes::tracked_swap(&mut tmp, self);
            }
        }
    }
    pub mod option {
        #[allow(unused_imports)]
        use builtin::*;
        use builtin_macros::*;
        #[allow(unused_imports)]
        use crate::pervasive::*;
        pub enum Option<A> { None, Some(A), }
        #[automatically_derived]
        impl <A> Option<A> {
            #[spec]
            #[verifier(is_variant("None"))]
            #[allow(non_snake_case)]
            pub fn is_None(&self) -> bool {
                ::core::panicking::panic("not implemented")
            }
            #[spec]
            #[verifier(is_variant("Some"))]
            #[allow(non_snake_case)]
            pub fn is_Some(&self) -> bool {
                ::core::panicking::panic("not implemented")
            }
            #[spec]
            #[allow(non_snake_case)]
            #[verifier(get_variant("Some", 0))]
            pub fn get_Some_0(self) -> A {
                ::core::panicking::panic("not implemented")
            }
        }
        #[verifier(external)]
        impl <A: Clone> Clone for Option<A> {
            fn clone(&self) -> Self {
                match self {
                    Option::None => Option::None,
                    Option::Some(a) => Option::Some(a.clone()),
                }
            }
        }
        impl <A: Copy> Copy for Option<A> { }
        impl <A> Option<A> {
            #[spec]
            #[verifier(publish)]
            pub fn or(self, optb: Option<A>) -> Option<A> {
                match self { Option::None => optb, Option::Some(s) => self, }
            }
            #[exec]
            pub fn unwrap(&self) -> &A {
                requires(self.is_Some());
                ensures(|a: &A| equal(*a, self.get_Some_0()));
                match self {
                    Option::Some(a) => a,
                    Option::None => unreached(),
                }
            }
        }
    }
    pub mod result {
        #[allow(unused_imports)]
        use builtin::*;
        use builtin_macros::*;
        pub enum Result<T, E> { Ok(T), Err(E), }
        #[automatically_derived]
        impl <T, E> Result<T, E> {
            #[spec]
            #[verifier(is_variant("Ok"))]
            #[allow(non_snake_case)]
            pub fn is_Ok(&self) -> bool {
                ::core::panicking::panic("not implemented")
            }
            #[spec]
            #[allow(non_snake_case)]
            #[verifier(get_variant("Ok", 0))]
            pub fn get_Ok_0(self) -> T {
                ::core::panicking::panic("not implemented")
            }
            #[spec]
            #[verifier(is_variant("Err"))]
            #[allow(non_snake_case)]
            pub fn is_Err(&self) -> bool {
                ::core::panicking::panic("not implemented")
            }
            #[spec]
            #[allow(non_snake_case)]
            #[verifier(get_variant("Err", 0))]
            pub fn get_Err_0(self) -> E {
                ::core::panicking::panic("not implemented")
            }
        }
    }
    pub mod seq {
        use core::{marker};
        #[allow(unused_imports)]
        use builtin::*;
        #[allow(unused_imports)]
        use builtin_macros::*;
        #[allow(unused_imports)]
        use crate::pervasive::*;
        #[doc = " `Seq<A>` is a sequence type for specifications."]
        #[doc =
          " To use a \"sequence\" in compiled code, use an `exec` type like [`vec::Vec`]"]
        #[doc = " that has `Seq<A>` as its specification type."]
        #[doc = ""]
        #[doc =
          " An object `seq: Seq<A>` has a length, given by [`seq.len()`](Seq::len),"]
        #[doc =
          " and a value at each `i` for `0 <= i < seq.len()`, given by [`seq[i]`](Seq::index)."]
        #[doc = ""]
        #[doc = " Sequences can be constructed in a few different ways:"]
        #[doc =
          "  * [`Seq::empty`] construct an empty sequence (`len() == 0`)"]
        #[doc =
          "  * [`Seq::new`] construct a sequence of a given length, initialized according"]
        #[doc = "     to a given function mapping indices `i` to values `A`."]
        #[doc =
          "  * The [`seq!`] macro, to construct small sequences of a fixed size (analagous to the"]
        #[doc = "     [`std::vec!`] macro)."]
        #[doc =
          "  * By manipulating an existing sequence with [`Seq::push`], [`Seq::update`],"]
        #[doc = "    or [`Seq::add`]."]
        #[doc = ""]
        #[doc =
          " To prove that two sequences are equal, it is usually easiest to use the [`assert_seqs_equal!`] macro."]
        #[verifier(external_body)]
        pub struct Seq<#[verifier(strictly_positive)] A> {
            dummy: marker::PhantomData<A>,
        }
        impl <A> Seq<A> {
            #[doc = " An empty sequence (i.e., a sequence of length 0)."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn empty() -> Seq<A> {
                ::core::panicking::panic("not implemented")
            }
            #[doc =
              " Construct a sequence `s` of length `len` where entry `s[i]` is given by `f(i)`."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn new(len: nat, f: impl Fn(int) -> A) -> Seq<A> {
                ::core::panicking::panic("not implemented")
            }
            #[doc = " The length of a sequence."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn len(self) -> nat {
                ::core::panicking::panic("not implemented")
            }
            #[doc = " Gets the value at the given index `i`."]
            #[doc = ""]
            #[doc =
              " If `i` is not in the range `[0, self.len())`, then the resulting value"]
            #[doc = " is meaningless and arbitrary."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn index(self, i: int) -> A {
                ::builtin::recommends([::builtin::spec_chained_cmp(::builtin::spec_chained_lt(::builtin::spec_chained_le(::builtin::spec_chained_value(::builtin::spec_literal_nat("0")),
                                                                                                                         i),
                                                                                              self.len()))]);
                ::core::panicking::panic("not implemented")
            }
            #[doc = " `[]` operator, synonymous with `index`"]
            #[verifier(inline)]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn spec_index(self, i: int) -> A {
                ::builtin::recommends([::builtin::spec_chained_cmp(::builtin::spec_chained_lt(::builtin::spec_chained_le(::builtin::spec_chained_value(::builtin::spec_literal_nat("0")),
                                                                                                                         i),
                                                                                              self.len()))]);
                self.index(i)
            }
            #[doc = " Appends the value `a` to the end of the sequence."]
            #[doc = " This always increases the length of the sequence by 1."]
            #[doc = ""]
            #[doc = " ## Example"]
            #[doc = ""]
            #[doc = " ```rust"]
            #[doc = " proof fn push_test() {"]
            #[doc = "     assert_seqs_equal!("]
            #[doc = "           seq![10, 11, 12].push(13),"]
            #[doc = "           seq![10, 11, 12, 13],"]
            #[doc = "     );"]
            #[doc = " }"]
            #[doc = " ```"]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn push(self, a: A) -> Seq<A> {
                ::core::panicking::panic("not implemented")
            }
            #[doc =
              " Updates the sequence at the given index, replacing the element with the given"]
            #[doc = " value, and leaves all other entries unchanged."]
            #[doc = ""]
            #[doc = " ## Example"]
            #[doc = ""]
            #[doc = " ```rust"]
            #[doc = " proof fn update_test() {"]
            #[doc = "     let s = seq![10, 11, 12, 13, 14];"]
            #[doc = "     let t = s.update(2, -5);"]
            #[doc = "     assert_seqs_equal!(t, seq![10, 11, -5, 13, 14]);"]
            #[doc = " }"]
            #[doc = " ```"]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn update(self, i: int, a: A) -> Seq<A> {
                ::builtin::recommends([::builtin::spec_chained_cmp(::builtin::spec_chained_lt(::builtin::spec_chained_le(::builtin::spec_chained_value(::builtin::spec_literal_nat("0")),
                                                                                                                         i),
                                                                                              self.len()))]);
                ::core::panicking::panic("not implemented")
            }
            #[doc =
              " Returns `true` if the two sequences are pointwise equal, i.e.,"]
            #[doc =
              " they have the same length and the corresponding values are equal"]
            #[doc =
              " at each index. This is equivalent to the sequences being actually equal"]
            #[doc = " by [`axiom_seq_ext_equal`]."]
            #[doc = ""]
            #[doc =
              " To prove that two sequences are equal via extensionality, it is generally easier"]
            #[doc =
              " to use the [`assert_seqs_equal!`] macro, rather than using `ext_equal` directly."]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn ext_equal(self, s2: Seq<A>) -> bool {
                (::builtin::spec_eq(self.len(), s2.len())) &&
                    ((::builtin::forall(|i: int|
                                            ::builtin::imply(::builtin::spec_chained_cmp(::builtin::spec_chained_lt(::builtin::spec_chained_le(::builtin::spec_chained_value(::builtin::spec_literal_nat("0")),
                                                                                                                                               i),
                                                                                                                    self.len())),
                                                             ::builtin::equal(self.spec_index(i),
                                                                              s2.spec_index(i))))))
            }
            #[doc = " Returns a sequence for the given subrange."]
            #[doc = ""]
            #[doc = " ## Example"]
            #[doc = ""]
            #[doc = " ```rust"]
            #[doc = " proof fn subrange_test() {"]
            #[doc = "     let s = seq![10, 11, 12, 13, 14];"]
            #[doc = "     //                  ^-------^"]
            #[doc = "     //          0   1   2   3   4   5"]
            #[doc = "     let sub = s.subrange(2, 4);"]
            #[doc = "     assert_seqs_equal!(sub, seq![12, 13]);"]
            #[doc = " }"]
            #[doc = " ```"]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn subrange(self, start_inclusive: int, end_exclusive: int)
             -> Seq<A> {
                ::builtin::recommends([::builtin::spec_chained_cmp(::builtin::spec_chained_le(::builtin::spec_chained_le(::builtin::spec_chained_le(::builtin::spec_chained_value(::builtin::spec_literal_nat("0")),
                                                                                                                                                    start_inclusive),
                                                                                                                         end_exclusive),
                                                                                              self.len()))]);
                ::core::panicking::panic("not implemented")
            }
            #[doc = " Concatenates the sequences."]
            #[doc = ""]
            #[doc = " ## Example"]
            #[doc = ""]
            #[doc = " ```rust"]
            #[doc = " proof fn add_test() {"]
            #[doc = "     assert_seqs_equal!("]
            #[doc = "         seq![10, 11].push(seq![12, 13, 14]),"]
            #[doc = "         seq![10, 11, 12, 13, 14],"]
            #[doc = "     );"]
            #[doc = " }"]
            #[doc = " ```"]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn add(self, rhs: Seq<A>) -> Seq<A> {
                ::core::panicking::panic("not implemented")
            }
            #[doc = " `+` operator, synonymous with `add`"]
            #[verifier(inline)]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn spec_add(self, rhs: Seq<A>) -> Seq<A> { self.add(rhs) }
            #[doc = " Returns the last element of the sequence."]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn last(self) -> A {
                ::builtin::recommends([(::builtin::spec_literal_nat("0")).spec_lt(self.len())]);
                self.spec_index((::builtin::spec_cast_integer::<_,
                                                                int>(self.len())).spec_sub(::builtin::spec_literal_nat("1")))
            }
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_seq_empty<A>() {
            ::builtin::ensures([::builtin::spec_eq(#[trigger] Seq::<A>::empty().len(),
                                                   ::builtin::spec_literal_nat("0"))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_seq_new_len<A>(len: nat, f: impl Fn(int) -> A) {
            ::builtin::ensures([::builtin::spec_eq(#[trigger] Seq::new(len,
                                                                       f).len(),
                                                   len)]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_seq_new_index<A>(len: nat, f: impl Fn(int) -> A,
                                      i: int) {
            ::builtin::requires([::builtin::spec_chained_cmp(::builtin::spec_chained_lt(::builtin::spec_chained_le(::builtin::spec_chained_value(::builtin::spec_literal_nat("0")),
                                                                                                                   i),
                                                                                        len))]);
            ::builtin::ensures([::builtin::equal(Seq::new(len,
                                                          f).spec_index(i),
                                                 f(i))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_seq_push_len<A>(s: Seq<A>, a: A) {
            ::builtin::ensures([::builtin::spec_eq(#[trigger] s.push(a).len(),
                                                   (s.len()).spec_add(::builtin::spec_literal_nat("1")))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_seq_push_index_same<A>(s: Seq<A>, a: A, i: int) {
            ::builtin::requires([::builtin::spec_eq(i, s.len())]);
            ::builtin::ensures([::builtin::equal(#[trigger] s.push(a).spec_index(i),
                                                 a)]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_seq_push_index_different<A>(s: Seq<A>, a: A, i: int) {
            ::builtin::requires([::builtin::spec_chained_cmp(::builtin::spec_chained_lt(::builtin::spec_chained_le(::builtin::spec_chained_value(::builtin::spec_literal_nat("0")),
                                                                                                                   i),
                                                                                        s.len()))]);
            ::builtin::ensures([::builtin::equal(s.push(a).spec_index(i),
                                                 s.spec_index(i))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_seq_update_len<A>(s: Seq<A>, i: int, a: A) {
            ::builtin::requires([::builtin::spec_chained_cmp(::builtin::spec_chained_lt(::builtin::spec_chained_le(::builtin::spec_chained_value(::builtin::spec_literal_nat("0")),
                                                                                                                   i),
                                                                                        s.len()))]);
            ::builtin::ensures([::builtin::spec_eq(#[trigger] s.update(i,
                                                                       a).len(),
                                                   s.len())]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_seq_update_same<A>(s: Seq<A>, i: int, a: A) {
            ::builtin::requires([::builtin::spec_chained_cmp(::builtin::spec_chained_lt(::builtin::spec_chained_le(::builtin::spec_chained_value(::builtin::spec_literal_nat("0")),
                                                                                                                   i),
                                                                                        s.len()))]);
            ::builtin::ensures([::builtin::equal(#[trigger] s.update(i,
                                                                     a).spec_index(i),
                                                 a)]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_seq_update_different<A>(s: Seq<A>, i1: int, i2: int,
                                             a: A) {
            ::builtin::requires([::builtin::spec_chained_cmp(::builtin::spec_chained_lt(::builtin::spec_chained_le(::builtin::spec_chained_value(::builtin::spec_literal_nat("0")),
                                                                                                                   i1),
                                                                                        s.len())),
                                 ::builtin::spec_chained_cmp(::builtin::spec_chained_lt(::builtin::spec_chained_le(::builtin::spec_chained_value(::builtin::spec_literal_nat("0")),
                                                                                                                   i2),
                                                                                        s.len())),
                                 !::builtin::spec_eq(i1, i2)]);
            ::builtin::ensures([::builtin::equal(s.update(i2,
                                                          a).spec_index(i1),
                                                 s.spec_index(i1))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_seq_ext_equal<A>(s1: Seq<A>, s2: Seq<A>) {
            ::builtin::ensures([::builtin::spec_eq(s1.ext_equal(s2),
                                                   (::builtin::equal(s1,
                                                                     s2)))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_seq_subrange_len<A>(s: Seq<A>, j: int, k: int) {
            ::builtin::requires([::builtin::spec_chained_cmp(::builtin::spec_chained_le(::builtin::spec_chained_le(::builtin::spec_chained_le(::builtin::spec_chained_value(::builtin::spec_literal_nat("0")),
                                                                                                                                              j),
                                                                                                                   k),
                                                                                        s.len()))]);
            ::builtin::ensures([::builtin::spec_eq(#[trigger] s.subrange(j,
                                                                         k).len(),
                                                   (k).spec_sub(j))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_seq_subrange_index<A>(s: Seq<A>, j: int, k: int,
                                           i: int) {
            ::builtin::requires([::builtin::spec_chained_cmp(::builtin::spec_chained_le(::builtin::spec_chained_le(::builtin::spec_chained_le(::builtin::spec_chained_value(::builtin::spec_literal_nat("0")),
                                                                                                                                              j),
                                                                                                                   k),
                                                                                        s.len())),
                                 ::builtin::spec_chained_cmp(::builtin::spec_chained_lt(::builtin::spec_chained_le(::builtin::spec_chained_value(::builtin::spec_literal_nat("0")),
                                                                                                                   i),
                                                                                        (k).spec_sub(j)))]);
            ::builtin::ensures([::builtin::equal(s.subrange(j,
                                                            k).spec_index(i),
                                                 s.spec_index((i).spec_add(j)))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_seq_add_len<A>(s1: Seq<A>, s2: Seq<A>) {
            ::builtin::ensures([::builtin::spec_eq(#[trigger] s1.add(s2).len(),
                                                   (s1.len()).spec_add(s2.len()))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_seq_add_index1<A>(s1: Seq<A>, s2: Seq<A>, i: int) {
            ::builtin::requires([::builtin::spec_chained_cmp(::builtin::spec_chained_lt(::builtin::spec_chained_le(::builtin::spec_chained_value(::builtin::spec_literal_nat("0")),
                                                                                                                   i),
                                                                                        s1.len()))]);
            ::builtin::ensures([::builtin::equal(s1.add(s2).spec_index(i),
                                                 s1.spec_index(i))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_seq_add_index2<A>(s1: Seq<A>, s2: Seq<A>, i: int) {
            ::builtin::requires([(::builtin::spec_literal_nat("0")).spec_le(s1.len()),
                                 (i).spec_lt((::builtin::spec_cast_integer::<_,
                                                                             int>(s1.len())).spec_add(s2.len()))]);
            ::builtin::ensures([::builtin::equal(s1.add(s2).spec_index(i),
                                                 s2.spec_index((i).spec_sub(s1.len())))]);
        }
        #[doc(hidden)]
        #[macro_export]
        macro_rules! seq_internal {
            [$($elem : expr), * $(,) ?] =>
            { $crate :: pervasive :: seq :: Seq :: empty() $(.push($elem)) * }
        }
        #[doc = " Creates a [`Seq`] containing the given elements."]
        #[doc = ""]
        #[doc = " ## Example"]
        #[doc = ""]
        #[doc = " ```rust"]
        #[doc = " let s = seq![11, 12, 13];"]
        #[doc = ""]
        #[doc = " assert(s.len() == 3);"]
        #[doc = " assert(s[0] == 11);"]
        #[doc = " assert(s[1] == 12);"]
        #[doc = " assert(s[2] == 13);"]
        #[doc = " ```"]
        #[macro_export]
        macro_rules! seq {
            [$($tail : tt) *] =>
            {
                :: builtin_macros :: verus_proof_macro_exprs!
                ($crate :: pervasive :: seq :: seq_internal! ($($tail) *))
            } ;
        }
        #[doc(hidden)]
        pub use seq_internal;
        pub use seq;
    }
    pub mod seq_lib {
        #[allow(unused_imports)]
        use builtin::*;
        #[allow(unused_imports)]
        use builtin_macros::*;
        #[allow(unused_imports)]
        use crate::pervasive::*;
        #[allow(unused_imports)]
        use crate::pervasive::seq::*;
        impl <A> Seq<A> {
            #[doc =
              " Applies the function `f` to each element of the sequence, and returns"]
            #[doc = " the resulting sequence."]
            #[doc =
              " The `int` parameter of `f` is the index of the element being mapped."]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn map<B, F: Fn(int, A) -> B>(self, f: F) -> Seq<B> {
                Seq::new(self.len(),
                         ::builtin::closure_to_fn_spec(|i: int|
                                                           f(i,
                                                             self.spec_index(i))))
            }
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn contains(self, needle: A) -> bool {
                ::builtin::exists(|i: int|
                                      ::builtin::spec_chained_cmp(::builtin::spec_chained_lt(::builtin::spec_chained_le(::builtin::spec_chained_value(::builtin::spec_literal_nat("0")),
                                                                                                                        i),
                                                                                             self.len()))
                                          &&
                                          ::builtin::equal(self.spec_index(i),
                                                           needle))
            }
            #[doc =
              " Drops the last element of a sequence and returns a sequence whose length is"]
            #[doc = " thereby 1 smaller."]
            #[doc = ""]
            #[doc =
              " If the input sequence is empty, the result is meaningless and arbitrary."]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn drop_last(self) -> Seq<A> {
                ::builtin::recommends([(self.len()).spec_ge(::builtin::spec_literal_nat("1"))]);
                self.subrange(::builtin::spec_literal_integer("0"),
                              (::builtin::spec_cast_integer::<_,
                                                              int>(self.len())).spec_sub(::builtin::spec_literal_nat("1")))
            }
        }
        #[doc(hidden)]
        #[verifier(inline)]
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn check_argument_is_seq<A>(s: Seq<A>) -> Seq<A> { s }
        #[doc =
          " Prove two sequences `s1` and `s2` are equal by proving that their elements are equal at each index."]
        #[doc = ""]
        #[doc = " More precisely, `assert_seqs_equal!` requires:"]
        #[doc =
          "  * `s1` and `s2` have the same length (`s1.len() == s2.len()`), and"]
        #[doc =
          "  * for all `i` in the range `0 <= i < s1.len()`, we have `s1[i] === s2[i]`."]
        #[doc = ""]
        #[doc =
          " The property that equality follows from these facts is often called _extensionality_."]
        #[doc = ""]
        #[doc = " `assert_seqs_equal!` can handle many trivial-looking"]
        #[doc = " identities without any additional help:"]
        #[doc = ""]
        #[doc = " ```rust"]
        #[doc = " proof fn subrange_concat(s: Seq<u64>, i: int) {"]
        #[doc = "     requires(["]
        #[doc = "         0 <= i && i <= s.len(),"]
        #[doc = "     ]);"]
        #[doc = " "]
        #[doc = "     let t1 = s.subrange(0, i);"]
        #[doc = "     let t2 = s.subrange(i, s.len());"]
        #[doc = "     let t = t1.add(t2);"]
        #[doc = " "]
        #[doc = "     assert_seqs_equal!(s, t);"]
        #[doc = " "]
        #[doc = "     assert(s === t);"]
        #[doc = " }"]
        #[doc = " ```"]
        #[doc = ""]
        #[doc =
          " In more complex cases, a proof may be required for the equality of each element pair."]
        #[doc = " For example,"]
        #[doc = " "]
        #[doc = " ```rust"]
        #[doc = " proof fn bitvector_seqs() {"]
        #[doc = "     let s = Seq::<u64>::new(5, |i| i as u64);"]
        #[doc = "     let t = Seq::<u64>::new(5, |i| i as u64 | 0);"]
        #[doc = " "]
        #[doc = "     assert_seqs_equal!(s, t, i => {"]
        #[doc = "         // Need to show that s[i] == t[i]"]
        #[doc =
          "         // Prove that the elements are equal by appealing to a bitvector solver:"]
        #[doc = "         let j = i as u64;"]
        #[doc = "         assert_bit_vector(j | 0 == j);"]
        #[doc = "         assert(s[i] == t[i]);"]
        #[doc = "     });"]
        #[doc = " }"]
        #[doc = " ```"]
        #[macro_export]
        macro_rules! assert_seqs_equal {
            [$($tail : tt) *] =>
            {
                :: builtin_macros :: verus_proof_macro_exprs!
                ($crate :: pervasive :: seq_lib :: assert_seqs_equal_internal!
                 ($($tail) *))
            } ;
        }
        #[macro_export]
        #[doc(hidden)]
        macro_rules! assert_seqs_equal_internal {
            ($s1 : expr, $s2 : expr $(,) ?) =>
            { assert_seqs_equal_internal! ($s1, $s2, idx => { }) } ;
            ($s1 : expr, $s2 : expr, $idx : ident => $bblock : block) =>
            {
                let s1 = $crate :: pervasive :: seq_lib ::
                check_argument_is_seq($s1) ; let s2 = $crate :: pervasive ::
                seq_lib :: check_argument_is_seq($s2) ; :: builtin ::
                assert_by(:: builtin :: equal(s1, s2),
                          {
                              $crate :: pervasive ::
                              assert(s1.len() == s2.len()) ; :: builtin ::
                              assert_forall_by(| $idx : :: builtin :: int |
                                               {
                                                   :: builtin ::
                                                   requires(:: builtin_macros
                                                            ::
                                                            verus_proof_expr!
                                                            (0 <= $idx && $idx
                                                             < s1.len())) ; ::
                                                   builtin ::
                                                   ensures(:: builtin ::
                                                           equal(s1.index($idx),
                                                                 s2.index($idx)))
                                                   ; { $bblock }
                                               }) ; $crate :: pervasive ::
                              assert(s1.ext_equal(s2)) ;
                          }) ;
            }
        }
        #[doc(hidden)]
        pub use assert_seqs_equal_internal;
        pub use assert_seqs_equal;
    }
    pub mod set {
        use core::marker;
        #[allow(unused_imports)]
        use builtin::*;
        #[allow(unused_imports)]
        use builtin_macros::*;
        #[allow(unused_imports)]
        use crate::pervasive::*;
        #[allow(unused_imports)]
        use crate::pervasive::map::*;
        #[doc = " `Set<A>` is a set type for specifications."]
        #[doc = ""]
        #[doc =
          " An object `set: Set<A>` is a subset of the set of all values `a: A`."]
        #[doc =
          " Equivalently, it can be thought of as a boolean predicate on `A`."]
        #[doc = ""]
        #[doc = " In general, a set might be infinite."]
        #[doc =
          " To work specifically with finite sets, see the [`self.finite()`](Set::finite) predicate."]
        #[doc = " "]
        #[doc = " Sets can be constructed in a few different ways:"]
        #[doc = "  * [`Set::empty`] gives an empty set"]
        #[doc = "  * [`Set::full`] gives the set of all elements in `A`"]
        #[doc = "  * [`Set::new`] constructs a set from a boolean predicate"]
        #[doc =
          "  * The [`set!`] macro, to construct small sets of a fixed size"]
        #[doc =
          "  * By manipulating an existing sequence with [`Set::union`], [`Set::intersect`],"]
        #[doc =
          "    [`Set::difference`], [`Set::complement`], [`Set::filter`], [`Set::insert`],"]
        #[doc = "    or [`Set::remove`]."]
        #[doc = ""]
        #[doc =
          " To prove that two sequences are equal, it is usually easiest to use the [`assert_seqs_equal!`] macro."]
        #[verifier(external_body)]
        pub struct Set<#[verifier(maybe_negative)] A> {
            dummy: marker::PhantomData<A>,
        }
        impl <A> Set<A> {
            #[doc = " The \"empty\" set. "]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn empty() -> Set<A> {
                ::core::panicking::panic("not implemented")
            }
            #[doc =
              " Set whose membership is determined by the given boolean predicate."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn new<F: Fn(A) -> bool>(f: F) -> Set<A> {
                ::core::panicking::panic("not implemented")
            }
            #[doc =
              " The \"full\" set, i.e., set containing every element of type `A`."]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn full() -> Set<A> { Set::empty().complement() }
            #[doc =
              " Predicate indicating if the set contains the given element."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn contains(self, a: A) -> bool {
                ::core::panicking::panic("not implemented")
            }
            #[doc =
              " Returns `true` if for every value `a: A`, it is either in both input sets or neither."]
            #[doc = " This is equivalent to the sets being actually equal"]
            #[doc = " by [`axiom_set_ext_equal`]."]
            #[doc = ""]
            #[doc =
              " To prove that two sets are equal via extensionality, it is generally easier"]
            #[doc =
              " to use the [`assert_sets_equal!`] macro, rather than using `ext_equal` directly."]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn ext_equal(self, s2: Set<A>) -> bool {
                ::builtin::forall(|a: A|
                                      ::builtin::spec_eq(self.contains(a),
                                                         s2.contains(a)))
            }
            #[doc =
              " Returns `true` if the first argument is a subset of the second."]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn subset_of(self, s2: Set<A>) -> bool {
                ::builtin::forall(|a: A|
                                      ::builtin::imply(self.contains(a),
                                                       s2.contains(a)))
            }
            #[doc = " Returns a new set with the given element inserted."]
            #[doc =
              " If that element is already in the set, then an identical set is returned."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn insert(self, a: A) -> Set<A> {
                ::core::panicking::panic("not implemented")
            }
            #[doc = " Returns a new set with the given element removed."]
            #[doc =
              " If that element is already absent from the set, then an identical set is returned."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn remove(self, a: A) -> Set<A> {
                ::core::panicking::panic("not implemented")
            }
            #[doc = " Union of two sets."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn union(self, s2: Set<A>) -> Set<A> {
                ::core::panicking::panic("not implemented")
            }
            #[doc = " `+` operator, synonymous with `union`"]
            #[verifier(inline)]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn spec_add(self, s2: Set<A>) -> Set<A> { self.union(s2) }
            #[doc = " Intersection of two sets."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn intersect(self, s2: Set<A>) -> Set<A> {
                ::core::panicking::panic("not implemented")
            }
            #[doc =
              " Set difference, i.e., the set of all elements in the first one but not in the second."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn difference(self, s2: Set<A>) -> Set<A> {
                ::core::panicking::panic("not implemented")
            }
            #[doc =
              " Set complement (within the space of all possible elements in `A`)."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn complement(self) -> Set<A> {
                ::core::panicking::panic("not implemented")
            }
            #[doc =
              " Set of all elements in the given set which satisfy the predicate `f`."]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn filter<F: Fn(A) -> bool>(self, f: F) -> Set<A> {
                self.intersect(Self::new(f))
            }
            #[doc = " Returns `true` if the set is finite."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn finite(self) -> bool {
                ::core::panicking::panic("not implemented")
            }
            #[doc =
              " Cardinality of the set. (Only meaningful if a set is finite.)"]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn len(self) -> nat {
                ::core::panicking::panic("not implemented")
            }
            #[doc = " Chooses an arbitrary element of the set."]
            #[doc = ""]
            #[doc = " This is often useful for proofs by induction."]
            #[doc = ""]
            #[doc =
              " (Note that, although the result is arbitrary, it is still a _deterministic_ function"]
            #[doc = " like any other `spec` function.)"]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn choose(self) -> A {
                ::builtin::choose(|a: A| self.contains(a))
            }
            #[doc =
              " Creates a [`Map`](map::Map) whose domain is the given set."]
            #[doc =
              " The values of the map are given by `f`, a function of the keys."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn mk_map<V, F: Fn(A) -> V>(self, f: F) -> Map<A, V> {
                ::core::panicking::panic("not implemented")
            }
            #[doc =
              " Returns `true` if the sets are disjoint, i.e., if their interesection is"]
            #[doc = " the empty set."]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn disjoint(self, s2: Self) -> bool {
                ::builtin::forall(|a: A|
                                      ::builtin::imply(self.contains(a),
                                                       !s2.contains(a)))
            }
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_empty<A>(a: A) {
            ::builtin::ensures([!Set::empty().contains(a)]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_new<A, F: Fn(A) -> bool>(f: F, a: A) {
            ::builtin::ensures([::builtin::spec_eq(Set::new(f).contains(a),
                                                   f(a))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_insert_same<A>(s: Set<A>, a: A) {
            ::builtin::ensures([#[trigger] s.insert(a).contains(a)]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_insert_different<A>(s: Set<A>, a1: A, a2: A) {
            ::builtin::requires([!::builtin::equal(a1, a2)]);
            ::builtin::ensures([::builtin::spec_eq(s.insert(a2).contains(a1),
                                                   s.contains(a1))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_remove_same<A>(s: Set<A>, a: A) {
            ::builtin::ensures([!(#[trigger] s.remove(a).contains(a))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_remove_different<A>(s: Set<A>, a1: A, a2: A) {
            ::builtin::requires([!::builtin::equal(a1, a2)]);
            ::builtin::ensures([::builtin::spec_eq(s.remove(a2).contains(a1),
                                                   s.contains(a1))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_union<A>(s1: Set<A>, s2: Set<A>, a: A) {
            ::builtin::ensures([::builtin::spec_eq(s1.union(s2).contains(a),
                                                   (s1.contains(a) ||
                                                        s2.contains(a)))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_intersect<A>(s1: Set<A>, s2: Set<A>, a: A) {
            ::builtin::ensures([::builtin::spec_eq(s1.intersect(s2).contains(a),
                                                   (s1.contains(a) &&
                                                        s2.contains(a)))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_difference<A>(s1: Set<A>, s2: Set<A>, a: A) {
            ::builtin::ensures([::builtin::spec_eq(s1.difference(s2).contains(a),
                                                   (s1.contains(a) &&
                                                        !s2.contains(a)))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_complement<A>(s: Set<A>, a: A) {
            ::builtin::ensures([::builtin::spec_eq(s.complement().contains(a),
                                                   !s.contains(a))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_ext_equal<A>(s1: Set<A>, s2: Set<A>) {
            ::builtin::ensures([::builtin::spec_eq(s1.ext_equal(s2),
                                                   (::builtin::equal(s1,
                                                                     s2)))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_mk_map_domain<K, V, F: Fn(K) -> V>(s: Set<K>, f: F) {
            ::builtin::ensures([::builtin::equal(#[trigger] s.mk_map(f).dom(),
                                                 s)]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_mk_map_index<K, V, F: Fn(K)
                                  -> V>(s: Set<K>, f: F, key: K) {
            ::builtin::requires([s.contains(key)]);
            ::builtin::ensures([::builtin::equal(s.mk_map(f).spec_index(key),
                                                 f(key))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_empty_finite<A>() {
            ::builtin::ensures([#[trigger] Set::<A>::empty().finite()]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_insert_finite<A>(s: Set<A>, a: A) {
            ::builtin::requires([s.finite()]);
            ::builtin::ensures([#[trigger] s.insert(a).finite()]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_remove_finite<A>(s: Set<A>, a: A) {
            ::builtin::requires([s.finite()]);
            ::builtin::ensures([#[trigger] s.remove(a).finite()]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_union_finite<A>(s1: Set<A>, s2: Set<A>) {
            ::builtin::requires([s1.finite(), s2.finite()]);
            ::builtin::ensures([#[trigger] s1.union(s2).finite()]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_intersect_finite<A>(s1: Set<A>, s2: Set<A>) {
            ::builtin::requires([s1.finite() || s2.finite()]);
            ::builtin::ensures([#[trigger] s1.intersect(s2).finite()]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_difference_finite<A>(s1: Set<A>, s2: Set<A>) {
            ::builtin::requires([s1.finite()]);
            ::builtin::ensures([#[trigger] s1.difference(s2).finite()]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_choose_finite<A>(s: Set<A>) {
            ::builtin::requires([!s.finite()]);
            ::builtin::ensures([#[trigger] s.contains(s.choose())]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_empty_len<A>() {
            ::builtin::ensures([::builtin::spec_eq(#[trigger] Set::<A>::empty().len(),
                                                   ::builtin::spec_literal_nat("0"))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_insert_len<A>(s: Set<A>, a: A) {
            ::builtin::requires([s.finite()]);
            ::builtin::ensures([::builtin::spec_eq(#[trigger] s.insert(a).len(),
                                                   (s.len()).spec_add((if s.contains(a)
                                                                          {
                                                                           ::builtin::spec_literal_int("0")
                                                                       } else {
                                                                           ::builtin::spec_literal_integer("1")
                                                                       })))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_remove_len<A>(s: Set<A>, a: A) {
            ::builtin::requires([s.finite()]);
            ::builtin::ensures([::builtin::spec_eq(s.len(),
                                                   (#[trigger] s.remove(a).len()).spec_add((if s.contains(a)
                                                                                               {
                                                                                                ::builtin::spec_literal_int("1")
                                                                                            } else {
                                                                                                ::builtin::spec_literal_integer("0")
                                                                                            })))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_set_choose_len<A>(s: Set<A>) {
            ::builtin::requires([s.finite(),
                                 !::builtin::spec_eq(#[trigger] s.len(),
                                                     ::builtin::spec_literal_nat("0"))]);
            ::builtin::ensures([#[trigger] s.contains(s.choose())]);
        }
        #[doc(hidden)]
        #[macro_export]
        macro_rules! set_internal {
            [$($elem : expr), * $(,) ?] =>
            {
                $crate :: pervasive :: set :: Set :: empty() $(.insert($elem))
                *
            } ;
        }
        #[macro_export]
        macro_rules! set {
            [$($tail : tt) *] =>
            {
                :: builtin_macros :: verus_proof_macro_exprs!
                ($crate :: pervasive :: set :: set_internal! ($($tail) *))
            } ;
        }
        pub use set_internal;
        pub use set;
    }
    pub mod set_lib {
        #[allow(unused_imports)]
        use builtin::*;
        #[allow(unused_imports)]
        use builtin_macros::*;
        #[allow(unused_imports)]
        use crate::pervasive::*;
        #[allow(unused_imports)]
        use crate::pervasive::set::*;
        impl <A> Set<A> {
            #[verifier(verus_macro)]
            #[proof]
            pub fn is_empty(self) -> bool {
                ::builtin::requires([self.finite()]);
                ::builtin::ensures(|b: bool|
                                       [::builtin::spec_eq((b),
                                                           (self.finite() &&
                                                                ::builtin::spec_eq(self.len(),
                                                                                   ::builtin::spec_literal_nat("0")))),
                                        ::builtin::spec_eq((b),
                                                           (self.ext_equal(Set::empty())))]);
                if self.finite() &&
                       ::builtin::spec_eq(self.len(),
                                          ::builtin::spec_literal_nat("0")) {
                    lemma_len0_is_empty::<A>(self);
                }
                self.ext_equal(Set::empty())
            }
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn map<B>(self, f: impl Fn(A) -> B) -> Set<B> {
                Set::new(::builtin::closure_to_fn_spec(|a: B|
                                                           ::builtin::exists(|x:
                                                                                  A|
                                                                                 self.contains(x)
                                                                                     &&
                                                                                     ::builtin::equal(a,
                                                                                                      f(x)))))
            }
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn fold<E, F: Fn(E, A) -> E>(self, init: E, f: F) -> E {
                ::builtin::decreases((self.len(),));
                if self.finite() {
                    if ::builtin::spec_eq(self.len(),
                                          ::builtin::spec_literal_nat("0")) {
                        init
                    } else {
                        let a = self.choose();
                        self.remove(a).fold(f(init, a), f)
                    }
                } else { arbitrary() }
            }
        }
        #[verifier(verus_macro)]
        #[proof]
        pub fn lemma_len0_is_empty<A>(s: Set<A>) {
            ::builtin::requires([s.finite(),
                                 ::builtin::spec_eq(s.len(),
                                                    ::builtin::spec_literal_nat("0"))]);
            ::builtin::ensures([::builtin::equal(s, Set::empty())]);
            if ::builtin::exists(|a: A| s.contains(a)) {
                ::builtin::assert_(::builtin::spec_eq((s.remove(s.choose()).len()).spec_add(::builtin::spec_literal_nat("1")),
                                                      ::builtin::spec_literal_nat("0")));
            }
            ::builtin::assert_(s.ext_equal(Set::empty()));
        }
        #[verifier(verus_macro)]
        #[proof]
        pub fn lemma_len_union<A>(s1: Set<A>, s2: Set<A>) {
            ::builtin::requires([s1.finite(), s2.finite()]);
            ::builtin::ensures([(s1.union(s2).len()).spec_le((s1.len()).spec_add(s2.len()))]);
            ::builtin::decreases((s1.len(),));
            if s1.is_empty() {
                ::builtin::assert_(s1.union(s2).ext_equal(s2));
            } else {
                let a = s1.choose();
                if s2.contains(a) {
                    ::builtin::assert_(s1.union(s2).ext_equal(s1.remove(a).union(s2)));
                } else {
                    ::builtin::assert_(s1.union(s2).remove(a).ext_equal(s1.remove(a).union(s2)));
                }
                lemma_len_union::<A>(s1.remove(a), s2);
            }
        }
        #[verifier(verus_macro)]
        #[proof]
        pub fn lemma_len_intersect<A>(s1: Set<A>, s2: Set<A>) {
            ::builtin::requires([s1.finite()]);
            ::builtin::ensures([(s1.intersect(s2).len()).spec_le(s1.len())]);
            ::builtin::decreases((s1.len(),));
            if s1.is_empty() {
                ::builtin::assert_(s1.intersect(s2).ext_equal(s1));
            } else {
                let a = s1.choose();
                ::builtin::assert_(s1.intersect(s2).remove(a).ext_equal(s1.remove(a).intersect(s2)));
                lemma_len_intersect::<A>(s1.remove(a), s2);
            }
        }
        #[verifier(verus_macro)]
        #[proof]
        pub fn lemma_len_subset<A>(s1: Set<A>, s2: Set<A>) {
            ::builtin::requires([s2.finite(), s1.subset_of(s2)]);
            ::builtin::ensures([(s1.len()).spec_le(s2.len()), s1.finite()]);
            lemma_len_intersect::<A>(s2, s1);
            ::builtin::assert_(s2.intersect(s1).ext_equal(s1));
        }
        #[verifier(verus_macro)]
        #[proof]
        pub fn lemma_len_difference<A>(s1: Set<A>, s2: Set<A>) {
            ::builtin::requires([s1.finite()]);
            ::builtin::ensures([(s1.difference(s2).len()).spec_le(s1.len())]);
            ::builtin::decreases((s1.len(),));
            if s1.is_empty() {
                ::builtin::assert_(s1.difference(s2).ext_equal(s1));
            } else {
                let a = s1.choose();
                ::builtin::assert_(s1.difference(s2).remove(a).ext_equal(s1.remove(a).difference(s2)));
                lemma_len_difference::<A>(s1.remove(a), s2);
            }
        }
        #[verifier(verus_macro)]
        #[proof]
        pub fn lemma_len_filter<A>(s: Set<A>, f: impl Fn(A) -> bool) {
            ::builtin::requires([s.finite()]);
            ::builtin::ensures([s.filter(f).finite(),
                                (s.filter(f).len()).spec_le(s.len())]);
            ::builtin::decreases((s.len(),));
            lemma_len_intersect::<A>(s, Set::new(f));
        }
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn set_int_range(lo: int, hi: int) -> Set<int> {
            Set::new(::builtin::closure_to_fn_spec(|i: int|
                                                       (lo).spec_le(i) &&
                                                           (i).spec_lt(hi)))
        }
        #[verifier(verus_macro)]
        #[proof]
        pub fn lemma_int_range(lo: int, hi: int) {
            ::builtin::requires([(lo).spec_le(hi)]);
            ::builtin::ensures([set_int_range(lo, hi).finite(),
                                ::builtin::spec_eq(set_int_range(lo,
                                                                 hi).len(),
                                                   (hi).spec_sub(lo))]);
            ::builtin::decreases(((hi).spec_sub(lo),));
            if ::builtin::spec_eq(lo, hi) {
                ::builtin::assert_(set_int_range(lo,
                                                 hi).ext_equal(Set::empty()));
            } else {
                lemma_int_range(lo,
                                (hi).spec_sub(::builtin::spec_literal_nat("1")));
                ::builtin::assert_(set_int_range(lo,
                                                 (hi).spec_sub(::builtin::spec_literal_nat("1"))).insert((hi).spec_sub(::builtin::spec_literal_nat("1"))).ext_equal(set_int_range(lo,
                                                                                                                                                                                  hi)));
            }
        }
        #[doc(hidden)]
        #[verifier(inline)]
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn check_argument_is_set<A>(s: Set<A>) -> Set<A> { s }
        #[doc = " Prove two sets equal by extensionality. Usage is:"]
        #[doc = ""]
        #[doc = " ```rust"]
        #[doc = " assert_sets_equal!(set1, set2);"]
        #[doc = " ```"]
        #[doc = " "]
        #[doc = " or,"]
        #[doc = " "]
        #[doc = " ```rust"]
        #[doc = " assert_sets_equal!(set1, set2, elem => {"]
        #[doc =
          "     // prove that set1.contains(elem) iff set2.contains(elem)"]
        #[doc = " });"]
        #[doc = " ```"]
        #[macro_export]
        macro_rules! assert_sets_equal {
            [$($tail : tt) *] =>
            {
                :: builtin_macros :: verus_proof_macro_exprs!
                ($crate :: pervasive :: set_lib :: assert_sets_equal_internal!
                 ($($tail) *))
            } ;
        }
        #[macro_export]
        #[doc(hidden)]
        macro_rules! assert_sets_equal_internal {
            ($s1 : expr, $s2 : expr $(,) ?) =>
            { assert_sets_equal_internal! ($s1, $s2, elem => { }) } ;
            ($s1 : expr, $s2 : expr, $elem : ident $(: $t : ty) ? => $bblock :
             block) =>
            {
                let s1 = $crate :: pervasive :: set_lib ::
                check_argument_is_set($s1) ; let s2 = $crate :: pervasive ::
                set_lib :: check_argument_is_set($s2) ; :: builtin ::
                assert_by(:: builtin :: equal(s1, s2),
                          {
                              :: builtin ::
                              assert_forall_by(| $elem $(: $t) ? |
                                               {
                                                   :: builtin ::
                                                   ensures(:: builtin ::
                                                           imply(s1.contains($elem),
                                                                 s2.contains($elem))
                                                           && :: builtin ::
                                                           imply(s2.contains($elem),
                                                                 s1.contains($elem)))
                                                   ; { $bblock }
                                               }) ; $crate :: pervasive ::
                              assert(s1.ext_equal(s2)) ;
                          }) ;
            }
        }
        pub use assert_sets_equal_internal;
        pub use assert_sets_equal;
    }
    pub mod cell {
        use core::cell::UnsafeCell;
        use core::{mem, mem::MaybeUninit};
        use core::marker;
        #[allow(unused_imports)]
        use builtin::*;
        #[allow(unused_imports)]
        use builtin_macros::*;
        #[allow(unused_imports)]
        use crate::pervasive::*;
        #[allow(unused_imports)]
        use crate::pervasive::modes::*;
        #[allow(unused_imports)]
        use crate::pervasive::invariant::*;
        #[allow(unused_imports)]
        use crate::pervasive::set::*;
        #[doc =
          " `PCell<V>` (which stands for \"permissioned call\") is the primitive Verus `Cell` type."]
        #[doc = ""]
        #[doc = " Technically, it is a wrapper around"]
        #[doc =
          " `core::cell::UnsafeCell<core::mem::MaybeUninit<V>>`, and thus has the same runtime"]
        #[doc =
          " properties: there are no runtime checks (as there would be for Rust\'s traditional"]
        #[doc =
          " [`core::cell::RefCell`](https://doc.rust-lang.org/core/cell/struct.RefCell.html))."]
        #[doc = " Its data may be uninitialized."]
        #[doc = ""]
        #[doc = " Furthermore (and unlike both"]
        #[doc =
          " [`core::cell::Cell`](https://doc.rust-lang.org/core/cell/struct.Cell.html) and"]
        #[doc =
          " [`core::cell::RefCell`](https://doc.rust-lang.org/core/cell/struct.RefCell.html)),"]
        #[doc = " a `PCell<V>` may be `Sync` (depending on `V`)."]
        #[doc =
          " Thanks to verification, Verus ensures that access to the cell is data-race-free."]
        #[doc = ""]
        #[doc =
          " `PCell` uses a _ghost permission token_ similar to [`ptr::PPtr`] -- see the [`ptr::PPtr`]"]
        #[doc = " documentation for the basics."]
        #[doc =
          " For `PCell`, the associated type of the permission token is [`cell::PermissionOpt`]."]
        #[doc = ""]
        #[doc = " ### Differences from `PPtr`."]
        #[doc = ""]
        #[doc =
          " The key difference is that, whereas [`ptr::PPtr`] represents a fixed address in memory,"]
        #[doc =
          " a `PCell` has _no_ fixed address because a `PCell` might be moved."]
        #[doc =
          " As such, the [`pcell.id()`](PCell::id) does not correspond to a memory address; rather,"]
        #[doc =
          " it is a unique identifier that is fixed for a given cell, even when it is moved."]
        #[doc = ""]
        #[doc =
          " The arbitrary ID given by [`pcell.id()`](PCell::id) is of type [`CellId`]."]
        #[doc =
          " Despite the fact that it is, in some ways, \"like a pointer\", note that"]
        #[doc = " `CellId` does not support any meangingful arithmetic,"]
        #[doc = " has no concept of a \"null ID\","]
        #[doc = " and has no runtime representation."]
        #[doc = ""]
        #[doc =
          " Also note that the `PCell` might be dropped before the `PermissionOpt` token is dropped,"]
        #[doc =
          " although in that case it will no longer be possible to use the `PermissionOpt` in `exec` code"]
        #[doc = " to extract data from the cell."]
        #[doc = ""]
        #[doc = " ### Example (TODO)"]
        #[verifier(external_body)]
        pub struct PCell<#[verifier(strictly_positive)] V> {
            ucell: UnsafeCell<MaybeUninit<V>>,
        }
        #[verifier(external)]
        unsafe impl <T> Sync for PCell<T> { }
        #[verifier(external)]
        unsafe impl <T> Send for PCell<T> { }
        #[verifier(external_body)]
        #[proof]
        pub struct PermissionOpt<#[verifier(strictly_positive)] V> {
            phantom: marker::PhantomData<V>,
            no_copy: NoCopy,
        }
        #[spec]
        pub struct PermissionOptData<V> {
            pub pcell: CellId,
            pub value: option::Option<V>,
        }
        #[doc(hidden)]
        #[macro_export]
        macro_rules! pcell_opt_internal {
            [$pcell : expr => $val : expr] =>
            {
                $crate :: pervasive :: cell :: PermissionOptData
                { pcell : $pcell, value : $val, }
            } ;
        }
        #[macro_export]
        macro_rules! pcell_opt {
            [$($tail : tt) *] =>
            {
                :: builtin_macros :: verus_proof_macro_exprs!
                ($crate :: pervasive :: cell :: pcell_opt_internal!
                 ($($tail) *))
            }
        }
        pub use pcell_opt_internal;
        pub use pcell_opt;
        #[verifier(external_body)]
        pub struct CellId {
            id: int,
        }
        impl <V> PermissionOpt<V> {
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn view(self) -> PermissionOptData<V> {
                ::core::panicking::panic("not implemented")
            }
        }
        impl <V> PCell<V> {
            #[doc = " A unique ID for the cell."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn id(&self) -> CellId {
                ::core::panicking::panic("not implemented")
            }
            #[doc = " Return an empty (\"uninitialized\") cell."]
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn empty() -> (PCell<V>, Tracked<PermissionOpt<V>>) {
                ::builtin::ensures(|pt: (PCell<V>, Tracked<PermissionOpt<V>>)|
                                       [::builtin::equal(((pt.1.view()).view()),
                                                         crate::pervasive::cell::PermissionOptData{pcell:
                                                                                                       pt.0.id(),
                                                                                                   value:
                                                                                                       option::Option::None,})]);
                let p = PCell{ucell: UnsafeCell::new(MaybeUninit::uninit()),};
                (p, Tracked::assume_new())
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn put(&self, perm: &mut Tracked<PermissionOpt<V>>, v: V) {
                ::builtin::requires([::builtin::equal(((old(perm).view()).view()),
                                                      crate::pervasive::cell::PermissionOptData{pcell:
                                                                                                    self.id(),
                                                                                                value:
                                                                                                    option::Option::None,})]);
                ::builtin::ensures([::builtin::equal(((perm.view()).view()),
                                                     crate::pervasive::cell::PermissionOptData{pcell:
                                                                                                   self.id(),
                                                                                               value:
                                                                                                   option::Option::Some(v),})]);
                opens_invariants_none();
                unsafe { *(self.ucell.get()) = MaybeUninit::new(v); }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn take(&self, perm: &mut Tracked<PermissionOpt<V>>) -> V {
                ::builtin::requires([::builtin::equal(self.id(),
                                                      ((old(perm).view()).view()).pcell),
                                     ((old(perm).view()).view()).value.is_Some()]);
                ::builtin::ensures(|v: V|
                                       [::builtin::equal(((perm.view()).view()).pcell,
                                                         ((old(perm).view()).view()).pcell),
                                        ::builtin::equal(((perm.view()).view()).value,
                                                         option::Option::None),
                                        ::builtin::equal(v,
                                                         ((old(perm).view()).view()).value.get_Some_0())]);
                opens_invariants_none();
                unsafe {
                    let mut m = MaybeUninit::uninit();
                    mem::swap(&mut m, &mut *self.ucell.get());
                    m.assume_init()
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn replace(&self, perm: &mut Tracked<PermissionOpt<V>>,
                           in_v: V) -> V {
                ::builtin::requires([::builtin::equal(self.id(),
                                                      ((old(perm).view()).view()).pcell),
                                     ((old(perm).view()).view()).value.is_Some()]);
                ::builtin::ensures(|out_v: V|
                                       [::builtin::equal(((perm.view()).view()).pcell,
                                                         ((old(perm).view()).view()).pcell),
                                        ::builtin::equal(((perm.view()).view()).value,
                                                         option::Option::Some(in_v)),
                                        ::builtin::equal(out_v,
                                                         ((old(perm).view()).view()).value.get_Some_0())]);
                opens_invariants_none();
                unsafe {
                    let mut m = MaybeUninit::new(in_v);
                    mem::swap(&mut m, &mut *self.ucell.get());
                    m.assume_init()
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn borrow<'a>(&'a self, perm: &'a Tracked<PermissionOpt<V>>)
             -> &'a V {
                ::builtin::requires([::builtin::equal(self.id(),
                                                      ((perm.view()).view()).pcell),
                                     ((perm.view()).view()).value.is_Some()]);
                ::builtin::ensures(|v: &'a V|
                                       [::builtin::equal(*v,
                                                         ((perm.view()).view()).value.get_Some_0())]);
                opens_invariants_none();
                unsafe { (*self.ucell.get()).assume_init_ref() }
            }
            #[inline(always)]
            #[verifier(verus_macro)]
            pub fn into_inner(self, perm: Tracked<PermissionOpt<V>>) -> V {
                ::builtin::requires([::builtin::equal(self.id(),
                                                      ((perm.view()).view()).pcell),
                                     ((perm.view()).view()).value.is_Some()]);
                ::builtin::ensures(|v: V|
                                       [::builtin::equal(v,
                                                         ((perm.view()).view()).value.get_Some_0())]);
                opens_invariants_none();
                let mut perm = perm;
                self.take(&mut perm)
            }
            #[inline(always)]
            #[verifier(verus_macro)]
            pub fn new(v: V) -> (PCell<V>, Tracked<PermissionOpt<V>>) {
                ::builtin::ensures(|pt: (PCell<V>, Tracked<PermissionOpt<V>>)|
                                       [(::builtin::equal(((pt.1.view()).view()),
                                                          PermissionOptData{pcell:
                                                                                pt.0.id(),
                                                                            value:
                                                                                option::Option::Some(v),}))]);
                let (p, mut t) = Self::empty();
                p.put(&mut t, v);
                (p, t)
            }
        }
        pub struct InvCell<#[verifier(maybe_negative)] T> {
            possible_values: Ghost<Set<T>>,
            pcell: PCell<T>,
            perm_inv: Tracked<LocalInvariant<PermissionOpt<T>>>,
        }
        impl <T> InvCell<T> {
            #[verifier(verus_macro)]
            #[spec]
            pub fn wf(&self) -> bool {
                (::builtin::forall(|perm|
                                       ::builtin::spec_eq(((self.perm_inv.view()).inv(perm)),
                                                          ({
                                                               (((perm.view()).value.is_Some())
                                                                    &&
                                                                    ((self.possible_values.view()).contains((perm.view()).value.get_Some_0())))
                                                                   &&
                                                                   (::builtin::equal(self.pcell.id(),
                                                                                     (perm.view()).pcell))
                                                           }))))
            }
            #[verifier(verus_macro)]
            #[spec]
            pub fn inv(&self, val: T) -> bool {
                (self.possible_values.view()).contains(val)
            }
            #[verifier(verus_macro)]
            pub fn new<F: Fn(T) -> bool>(val: T, #[spec] f: Ghost<F>)
             -> Self {
                ::builtin::requires([(f.view())(val)]);
                ::builtin::ensures(|cell: Self|
                                       [cell.wf() &&
                                            ::builtin::forall(|v|
                                                                  ::builtin::spec_eq(((f.view())(v)),
                                                                                     (cell.inv(v))))]);
                let (pcell, perm) = PCell::new(val);
                let possible_values =
                    #[verifier(ghost_wrapper)] crate::pervasive::modes::ghost_exec(#[verifier(ghost_block_wrapped)] (Set::new((f.view()))));
                let perm_inv =
                    #[verifier(ghost_wrapper)] crate::pervasive::modes::tracked_exec(#[verifier(tracked_block_wrapped)] (LocalInvariant::new(perm.get(),
                                                                                                                                             ::builtin::closure_to_fn_spec(|perm:
                                                                                                                                                                                cell::PermissionOpt<T>|
                                                                                                                                                                               {
                                                                                                                                                                                   (((perm.view()).value.is_Some())
                                                                                                                                                                                        &&
                                                                                                                                                                                        ((possible_values.view()).contains((perm.view()).value.get_Some_0())))
                                                                                                                                                                                       &&
                                                                                                                                                                                       (::builtin::equal(pcell.id(),
                                                                                                                                                                                                         (perm.view()).pcell))
                                                                                                                                                                               }),
                                                                                                                                             ::builtin::spec_literal_integer("0"))));
                InvCell{possible_values, pcell, perm_inv,}
            }
            pub fn replace(&self, val: T) -> T {
                requires(self.wf() && self.inv(val));
                ensures(|old_val| self.inv(old_val));
                let r;
                #[verifier(invariant_block)]
                {
                    #[allow(unused_mut)]
                    let (guard, mut perm) =
                        crate::pervasive::invariant::open_local_invariant_begin(self.perm_inv.borrow());
                    {
                        let mut t = tracked_exec(perm);
                        r = self.pcell.replace(&mut t, val);
                        perm = t.get();
                    }
                    crate::pervasive::invariant::open_invariant_end(guard,
                                                                    perm);
                };
                r
            }
        }
        impl <T: Copy> InvCell<T> {
            pub fn get(&self) -> T {
                requires(self.wf());
                ensures(|val| self.inv(val));
                let r;
                #[verifier(invariant_block)]
                {
                    #[allow(unused_mut)]
                    let (guard, mut perm) =
                        crate::pervasive::invariant::open_local_invariant_begin(self.perm_inv.borrow());
                    { r = *self.pcell.borrow(tracked_exec_borrow(&perm)); }
                    crate::pervasive::invariant::open_invariant_end(guard,
                                                                    perm);
                };
                r
            }
        }
    }
    pub mod cell_old_style {
        use core::cell::UnsafeCell;
        use core::{mem, mem::MaybeUninit};
        use core::marker;
        #[allow(unused_imports)]
        use builtin::*;
        #[allow(unused_imports)]
        use builtin_macros::*;
        #[allow(unused_imports)]
        use crate::pervasive::*;
        #[allow(unused_imports)]
        use crate::pervasive::modes::*;
        #[allow(unused_imports)]
        use crate::pervasive::invariant::*;
        #[allow(unused_imports)]
        use crate::pervasive::set::*;
        #[doc =
          " `PCell<V>` (which stands for \"permissioned call\") is the primitive Verus `Cell` type."]
        #[doc = ""]
        #[doc = " Technically, it is a wrapper around"]
        #[doc =
          " `core::cell::UnsafeCell<core::mem::MaybeUninit<V>>`, and thus has the same runtime"]
        #[doc =
          " properties: there are no runtime checks (as there would be for Rust\'s traditional"]
        #[doc =
          " [`core::cell::RefCell`](https://doc.rust-lang.org/core/cell/struct.RefCell.html))."]
        #[doc = " Its data may be uninitialized."]
        #[doc = ""]
        #[doc = " Furthermore (and unlike both"]
        #[doc =
          " [`core::cell::Cell`](https://doc.rust-lang.org/core/cell/struct.Cell.html) and"]
        #[doc =
          " [`core::cell::RefCell`](https://doc.rust-lang.org/core/cell/struct.RefCell.html)),"]
        #[doc = " a `PCell<V>` may be `Sync` (depending on `V`)."]
        #[doc =
          " Thanks to verification, Verus ensures that access to the cell is data-race-free."]
        #[doc = ""]
        #[doc =
          " `PCell` uses a _ghost permission token_ similar to [`ptr::PPtr`] -- see the [`ptr::PPtr`]"]
        #[doc = " documentation for the basics."]
        #[doc =
          " For `PCell`, the associated type of the permission token is [`cell::PermissionOpt`]."]
        #[doc = ""]
        #[doc = " ### Differences from `PPtr`."]
        #[doc = ""]
        #[doc =
          " The key difference is that, whereas [`ptr::PPtr`] represents a fixed address in memory,"]
        #[doc =
          " a `PCell` has _no_ fixed address because a `PCell` might be moved."]
        #[doc =
          " As such, the [`pcell.id()`](PCell::id) does not correspond to a memory address; rather,"]
        #[doc =
          " it is a unique identifier that is fixed for a given cell, even when it is moved."]
        #[doc = ""]
        #[doc =
          " The arbitrary ID given by [`pcell.id()`](PCell::id) is of type [`CellId`]."]
        #[doc =
          " Despite the fact that it is, in some ways, \"like a pointer\", note that"]
        #[doc = " `CellId` does not support any meangingful arithmetic,"]
        #[doc = " has no concept of a \"null ID\","]
        #[doc = " and has no runtime representation."]
        #[doc = ""]
        #[doc =
          " Also note that the `PCell` might be dropped before the `PermissionOpt` token is dropped,"]
        #[doc =
          " although in that case it will no longer be possible to use the `PermissionOpt` in `exec` code"]
        #[doc = " to extract data from the cell."]
        #[doc = ""]
        #[doc = " ### Example (TODO)"]
        #[verifier(external_body)]
        pub struct PCell<#[verifier(strictly_positive)] V> {
            ucell: UnsafeCell<MaybeUninit<V>>,
        }
        #[verifier(external)]
        unsafe impl <T> Sync for PCell<T> { }
        #[verifier(external)]
        unsafe impl <T> Send for PCell<T> { }
        #[verifier(external_body)]
        #[proof]
        pub struct PermissionOpt<#[verifier(strictly_positive)] V> {
            phantom: marker::PhantomData<V>,
            no_copy: NoCopy,
        }
        #[spec]
        pub struct PermissionOptData<V> {
            pub pcell: CellId,
            pub value: option::Option<V>,
        }
        #[doc(hidden)]
        #[macro_export]
        macro_rules! old_style_pcell_opt_internal {
            [$pcell : expr => $val : expr] =>
            {
                $crate :: pervasive :: cell_old_style :: PermissionOptData
                { pcell : $pcell, value : $val, }
            } ;
        }
        #[macro_export]
        macro_rules! old_style_pcell_opt {
            [$($tail : tt) *] =>
            {
                :: builtin_macros :: verus_proof_macro_exprs!
                ($crate :: pervasive :: cell_old_style :: pcell_opt_internal!
                 ($($tail) *))
            }
        }
        pub use old_style_pcell_opt_internal as pcell_opt_internal;
        pub use old_style_pcell_opt as pcell_opt;
        #[verifier(external_body)]
        pub struct CellId {
            id: int,
        }
        impl <V> PermissionOpt<V> {
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn view(self) -> PermissionOptData<V> {
                ::core::panicking::panic("not implemented")
            }
        }
        impl <V> PCell<V> {
            #[doc = " A unique ID for the cell."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn id(&self) -> CellId {
                ::core::panicking::panic("not implemented")
            }
            #[doc = " Return an empty (\"uninitialized\") cell."]
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn empty() -> (PCell<V>, Trk<PermissionOpt<V>>) {
                ::builtin::ensures(|pt: (PCell<V>, Trk<PermissionOpt<V>>)|
                                       [::builtin::equal((pt.1.0.view()),
                                                         crate::pervasive::cell_old_style::PermissionOptData{pcell:
                                                                                                                 pt.0.id(),
                                                                                                             value:
                                                                                                                 option::Option::None,})]);
                let p = PCell{ucell: UnsafeCell::new(MaybeUninit::uninit()),};
                (p, Trk(proof_from_false()))
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn put(&self, #[proof] perm: &mut PermissionOpt<V>, v: V) {
                ::builtin::requires([::builtin::equal((old(perm).view()),
                                                      crate::pervasive::cell_old_style::PermissionOptData{pcell:
                                                                                                              self.id(),
                                                                                                          value:
                                                                                                              option::Option::None,})]);
                ::builtin::ensures([::builtin::equal((perm.view()),
                                                     crate::pervasive::cell_old_style::PermissionOptData{pcell:
                                                                                                             self.id(),
                                                                                                         value:
                                                                                                             option::Option::Some(v),})]);
                opens_invariants_none();
                unsafe { *(self.ucell.get()) = MaybeUninit::new(v); }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn take(&self, #[proof] perm: &mut PermissionOpt<V>) -> V {
                ::builtin::requires([::builtin::equal(self.id(),
                                                      (old(perm).view()).pcell),
                                     (old(perm).view()).value.is_Some()]);
                ::builtin::ensures(|v: V|
                                       [::builtin::equal((perm.view()).pcell,
                                                         (old(perm).view()).pcell),
                                        ::builtin::equal((perm.view()).value,
                                                         option::Option::None),
                                        ::builtin::equal(v,
                                                         (old(perm).view()).value.get_Some_0())]);
                opens_invariants_none();
                unsafe {
                    let mut m = MaybeUninit::uninit();
                    mem::swap(&mut m, &mut *self.ucell.get());
                    m.assume_init()
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn replace(&self, #[proof] perm: &mut PermissionOpt<V>,
                           in_v: V) -> V {
                ::builtin::requires([::builtin::equal(self.id(),
                                                      (old(perm).view()).pcell),
                                     (old(perm).view()).value.is_Some()]);
                ::builtin::ensures(|out_v: V|
                                       [::builtin::equal((perm.view()).pcell,
                                                         (old(perm).view()).pcell),
                                        ::builtin::equal((perm.view()).value,
                                                         option::Option::Some(in_v)),
                                        ::builtin::equal(out_v,
                                                         (old(perm).view()).value.get_Some_0())]);
                opens_invariants_none();
                unsafe {
                    let mut m = MaybeUninit::new(in_v);
                    mem::swap(&mut m, &mut *self.ucell.get());
                    m.assume_init()
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn borrow<'a>(&'a self, #[proof] perm: &'a PermissionOpt<V>)
             -> &'a V {
                ::builtin::requires([::builtin::equal(self.id(),
                                                      (perm.view()).pcell),
                                     (perm.view()).value.is_Some()]);
                ::builtin::ensures(|v: &'a V|
                                       [::builtin::equal(*v,
                                                         (perm.view()).value.get_Some_0())]);
                opens_invariants_none();
                unsafe { (*self.ucell.get()).assume_init_ref() }
            }
            #[inline(always)]
            pub fn into_inner(self, #[proof] perm: PermissionOpt<V>) -> V {
                requires([equal(self.id(), perm.view().pcell),
                          perm.view().value.is_Some()]);
                ensures(|v: V| [equal(v, perm.view().value.get_Some_0())]);
                opens_invariants_none();
                #[proof]
                let mut perm = perm;
                self.take(&mut perm)
            }
            #[inline(always)]
            #[verifier(verus_macro)]
            pub fn new(v: V) -> (PCell<V>, Trk<PermissionOpt<V>>) {
                ::builtin::ensures(|pt: (PCell<V>, Trk<PermissionOpt<V>>)|
                                       [(::builtin::equal((pt.1.0.view()),
                                                          PermissionOptData{pcell:
                                                                                pt.0.id(),
                                                                            value:
                                                                                option::Option::Some(v),}))]);
                let (p, Trk(mut t)) = Self::empty();
                p.put(&mut t, v);
                (p, Trk(t))
            }
        }
        pub struct InvCell<#[verifier(maybe_negative)] T> {
            #[spec]
            possible_values: Set<T>,
            pcell: PCell<T>,
            #[proof]
            perm_inv: LocalInvariant<PermissionOpt<T>>,
        }
        impl <T> InvCell<T> {
            #[spec]
            pub fn wf(&self) -> bool {
                (forall(|perm|
                            self.perm_inv.inv(perm) ==
                                {
                                    perm.view().value.is_Some() &&
                                        self.possible_values.contains(perm.view().value.get_Some_0())
                                        &&
                                        equal(self.pcell.id(),
                                              perm.view().pcell)
                                }))
            }
            #[spec]
            pub fn inv(&self, val: T) -> bool {
                self.possible_values.contains(val)
            }
            pub fn new<F: Fn(T) -> bool>(val: T, #[spec] f: F) -> Self {
                requires(f(val));
                ensures(|cell: Self|
                            cell.wf() && forall(|v| f(v) == cell.inv(v)));
                let (pcell, Trk(perm)) = PCell::new(val);
                #[spec]
                let possible_values = Set::new(f);
                #[proof]
                let perm_inv =
                    LocalInvariant::new(perm,
                                        |perm: PermissionOpt<T>|
                                            {
                                                perm.view().value.is_Some() &&
                                                    possible_values.contains(perm.view().value.get_Some_0())
                                                    &&
                                                    equal(pcell.id(),
                                                          perm.view().pcell)
                                            },
                                        ::builtin::spec_literal_integer("0"));
                InvCell{possible_values, pcell, perm_inv,}
            }
            pub fn replace(&self, val: T) -> T {
                requires(self.wf() && self.inv(val));
                ensures(|old_val| self.inv(old_val));
                let r;
                #[verifier(invariant_block)]
                {
                    #[allow(unused_mut)]
                    let (guard, mut perm) =
                        crate::pervasive::invariant::open_local_invariant_begin(&self.perm_inv);
                    { r = self.pcell.replace(&mut perm, val); }
                    crate::pervasive::invariant::open_invariant_end(guard,
                                                                    perm);
                };
                r
            }
        }
        impl <T: Copy> InvCell<T> {
            pub fn get(&self) -> T {
                requires(self.wf());
                ensures(|val| self.inv(val));
                let r;
                #[verifier(invariant_block)]
                {
                    #[allow(unused_mut)]
                    let (guard, mut perm) =
                        crate::pervasive::invariant::open_local_invariant_begin(&self.perm_inv);
                    { r = *self.pcell.borrow(&perm); }
                    crate::pervasive::invariant::open_invariant_end(guard,
                                                                    perm);
                };
                r
            }
        }
    }
    pub mod invariant {
        #[allow(unused_imports)]
        use builtin::*;
        #[allow(unused_imports)]
        use builtin_macros::*;
        #[allow(unused_imports)]
        use crate::pervasive::*;
        #[proof]
        #[verifier(external_body)]
        pub struct AtomicInvariant<#[verifier(maybe_negative)] V> {
            dummy: builtin::SyncSendIfSend<V>,
        }
        #[verifier(external_body)]
        pub struct LocalInvariant<#[verifier(maybe_negative)] V> {
            dummy: builtin::SendIfSend<V>,
        }
        macro_rules! declare_invariant_impl {
            ($invariant : ident) =>
            {
                #[proof] impl < V > $invariant < V >
                {
                    fndecl! (pub fn inv(& self, _v : V) -> bool) ; fndecl!
                    (pub fn namespace(& self) -> int) ; #[proof]
                    #[verifier(external_body)] #[verifier(returns(proof))] pub
                    fn
                    new(#[proof] v : V, #[spec] inv : impl Fn(V) -> bool,
                        #[spec] ns : int) -> $invariant < V >
                    {
                        requires([inv(v),]) ;
                        ensures(| i : $invariant < V > |
                                forall(| v : V | i.inv(v) == inv(v)) &&
                                equal(i.namespace(), ns)) ; unimplemented! ()
                        ;
                    } #[proof] #[verifier(external_body)]
                    #[verifier(returns(proof))] pub fn
                    into_inner(#[proof] self) -> V
                    { ensures(| v : V | self.inv(v)) ; unimplemented! () ; }
                }
            }
        }
        #[proof]
        impl <V> AtomicInvariant<V> {
            #[spec]
            #[verifier(external_body)]
            pub fn inv(&self, _v: V) -> bool {
                ::core::panicking::panic("not implemented")
            }
            #[spec]
            #[verifier(external_body)]
            pub fn namespace(&self) -> int {
                ::core::panicking::panic("not implemented")
            }
            #[proof]
            #[verifier(external_body)]
            #[verifier(returns(proof))]
            pub fn new(#[proof] v: V, #[spec] inv: impl Fn(V) -> bool,
                       #[spec] ns: int) -> AtomicInvariant<V> {
                requires([inv(v)]);
                ensures(|i: AtomicInvariant<V>|
                            forall(|v: V| i.inv(v) == inv(v)) &&
                                equal(i.namespace(), ns));
                ::core::panicking::panic("not implemented");
            }
            #[proof]
            #[verifier(external_body)]
            #[verifier(returns(proof))]
            pub fn into_inner(#[proof] self) -> V {
                ensures(|v: V| self.inv(v));
                ::core::panicking::panic("not implemented");
            }
        }
        #[proof]
        impl <V> LocalInvariant<V> {
            #[spec]
            #[verifier(external_body)]
            pub fn inv(&self, _v: V) -> bool {
                ::core::panicking::panic("not implemented")
            }
            #[spec]
            #[verifier(external_body)]
            pub fn namespace(&self) -> int {
                ::core::panicking::panic("not implemented")
            }
            #[proof]
            #[verifier(external_body)]
            #[verifier(returns(proof))]
            pub fn new(#[proof] v: V, #[spec] inv: impl Fn(V) -> bool,
                       #[spec] ns: int) -> LocalInvariant<V> {
                requires([inv(v)]);
                ensures(|i: LocalInvariant<V>|
                            forall(|v: V| i.inv(v) == inv(v)) &&
                                equal(i.namespace(), ns));
                ::core::panicking::panic("not implemented");
            }
            #[proof]
            #[verifier(external_body)]
            #[verifier(returns(proof))]
            pub fn into_inner(#[proof] self) -> V {
                ensures(|v: V| self.inv(v));
                ::core::panicking::panic("not implemented");
            }
        }
        #[doc(hidden)]
        #[proof]
        pub struct InvariantBlockGuard;
        #[doc(hidden)]
        #[verifier(external)]
        pub fn open_atomic_invariant_begin<'a,
                                           V>(_inv: &'a AtomicInvariant<V>)
         -> (&'a InvariantBlockGuard, V) {
            ::core::panicking::panic("not implemented");
        }
        #[doc(hidden)]
        #[verifier(external)]
        pub fn open_local_invariant_begin<'a, V>(_inv: &'a LocalInvariant<V>)
         -> (&'a InvariantBlockGuard, V) {
            ::core::panicking::panic("not implemented");
        }
        #[doc(hidden)]
        #[verifier(external)]
        pub fn open_invariant_end<V>(_guard: &InvariantBlockGuard, _v: V) {
            ::core::panicking::panic("not implemented");
        }
        #[macro_export]
        macro_rules! open_atomic_invariant {
            ($eexpr : expr => $iident : ident => $bblock : block) =>
            {
                #[verifier(invariant_block)]
                {
                    #[allow(unused_mut)] let(guard, mut $iident) = $crate ::
                    pervasive :: invariant ::
                    open_atomic_invariant_begin($eexpr) ; $bblock $crate ::
                    pervasive :: invariant ::
                    open_invariant_end(guard, $iident) ;
                }
            }
        }
        #[macro_export]
        macro_rules! open_local_invariant {
            ($eexpr : expr => $iident : ident => $bblock : block) =>
            {
                #[verifier(invariant_block)]
                {
                    #[allow(unused_mut)] let(guard, mut $iident) = $crate ::
                    pervasive :: invariant ::
                    open_local_invariant_begin($eexpr) ; $bblock $crate ::
                    pervasive :: invariant ::
                    open_invariant_end(guard, $iident) ;
                }
            }
        }
    }
    pub mod atomic {
        use core::sync::atomic::{AtomicBool, AtomicU8, AtomicU16, AtomicU32,
                                 AtomicU64, AtomicI8, AtomicI16, AtomicI32,
                                 AtomicI64, Ordering};
        #[allow(unused_imports)]
        use builtin::*;
        #[allow(unused_imports)]
        use builtin_macros::*;
        #[allow(unused_imports)]
        use crate::pervasive::*;
        #[allow(unused_imports)]
        use crate::pervasive::modes::*;
        #[allow(unused_imports)]
        use crate::pervasive::result::*;
        macro_rules! make_unsigned_integer_atomic {
            ($at_ident : ident, $p_ident : ident, $p_data_ident : ident,
             $rust_ty : ty, $value_ty : ty, $wrap_add : ident, $wrap_sub :
             ident, $int_min : expr, $int_max : expr) =>
            {
                verus!
                {
                    pub open spec fn $wrap_add(a : int, b : int) -> int
                    {
                        if a + b > $int_max
                        { a + b - ($int_max - $int_min + 1) } else { a + b }
                    } pub open spec fn $wrap_sub(a : int, b : int) -> int
                    {
                        if a - b < $int_min
                        { a - b + ($int_max - $int_min + 1) } else { a - b }
                    }
                } atomic_types!
                ($at_ident, $p_ident, $p_data_ident, $rust_ty, $value_ty) ;
                impl $at_ident
                {
                    atomic_common_methods!
                    ($at_ident, $p_ident, $p_data_ident, $rust_ty, $value_ty)
                    ; atomic_integer_methods!
                    ($at_ident, $p_ident, $rust_ty, $value_ty, $wrap_add,
                     $wrap_sub, $int_min, $int_max) ;
                }
            }
        }
        macro_rules! make_signed_integer_atomic {
            ($at_ident : ident, $p_ident : ident, $p_data_ident : ident,
             $rust_ty : ty, $value_ty : ty, $wrap_add : ident, $wrap_sub :
             ident, $int_min : expr, $int_max : expr) =>
            {
                verus!
                {
                    pub open spec fn $wrap_add(a : int, b : int) -> int
                    {
                        if a + b > $int_max
                        { a + b - ($int_max - $int_min + 1) } else if a + b <
                        $int_min { a + b + ($int_max - $int_min + 1) } else
                        { a + b }
                    } pub open spec fn $wrap_sub(a : int, b : int) -> int
                    {
                        if a - b > $int_max
                        { a - b - ($int_max - $int_min + 1) } else if a - b <
                        $int_min { a - b + ($int_max - $int_min + 1) } else
                        { a - b }
                    }
                } atomic_types!
                ($at_ident, $p_ident, $p_data_ident, $rust_ty, $value_ty) ;
                impl $at_ident
                {
                    atomic_common_methods!
                    ($at_ident, $p_ident, $p_data_ident, $rust_ty, $value_ty)
                    ; atomic_integer_methods!
                    ($at_ident, $p_ident, $rust_ty, $value_ty, $wrap_add,
                     $wrap_sub, $int_min, $int_max) ;
                }
            }
        }
        macro_rules! make_bool_atomic {
            ($at_ident : ident, $p_ident : ident, $p_data_ident : ident,
             $rust_ty : ty, $value_ty : ty) =>
            {
                atomic_types!
                ($at_ident, $p_ident, $p_data_ident, $rust_ty, $value_ty) ;
                impl $at_ident
                {
                    atomic_common_methods!
                    ($at_ident, $p_ident, $p_data_ident, $rust_ty, $value_ty)
                    ; atomic_bool_methods!
                    ($at_ident, $p_ident, $rust_ty, $value_ty) ;
                }
            }
        }
        macro_rules! atomic_types {
            ($at_ident : ident, $p_ident : ident, $p_data_ident : ident,
             $rust_ty : ty, $value_ty : ty) =>
            {
                #[verifier(external_body)] pub struct $at_ident
                { ato : $rust_ty, } #[proof] #[verifier(external_body)] pub
                struct $p_ident { no_copy : NoCopy, } #[spec] pub struct
                $p_data_ident
                { #[spec] pub patomic : int, #[spec] pub value : $value_ty, }
                impl $p_ident
                {
                    #[spec] #[verifier(external_body)] pub fn view(self) ->
                    $p_data_ident { unimplemented! () ; } #[spec]
                    #[verifier(publish)] pub fn
                    is_for(& self, patomic : $at_ident) -> bool
                    { self.view().patomic == patomic.id() } #[spec]
                    #[verifier(publish)] pub fn
                    points_to(& self, v : $value_ty) -> bool
                    { self.view().value == v }
                }
            }
        }
        macro_rules! atomic_common_methods {
            ($at_ident : ident, $p_ident : ident, $p_data_ident : ident,
             $rust_ty : ty, $value_ty : ty) =>
            {
                fndecl! (pub fn id(& self) -> int) ; #[inline(always)]
                #[verifier(external_body)] pub fn new(i : $value_ty) ->
                ($at_ident, Proof < $p_ident >)
                {
                    ensures(| res : ($at_ident, Proof < $p_ident >) |
                            equal(res.1.0.view(), $p_data_ident
                                  { patomic : res.0.id(), value : i })) ; let
                    p = $at_ident { ato : < $rust_ty > :: new(i) } ; let
                    Proof(t) = exec_proof_from_false() ; (p, Proof(t))
                } #[inline(always)] #[verifier(external_body)]
                #[verifier(atomic)] pub fn
                load(& self, #[proof] perm : & $p_ident) -> $value_ty
                {
                    requires([equal(self.id(), perm.view().patomic),]) ;
                    ensures(| ret : $value_ty | equal(perm.view().value, ret))
                    ; opens_invariants_none() ; return
                    self.ato.load(Ordering :: SeqCst) ;
                } #[inline(always)] #[verifier(external_body)]
                #[verifier(atomic)] pub fn
                store(& self, #[proof] perm : & mut $p_ident, v : $value_ty)
                {
                    requires([equal(self.id(), old(perm).view().patomic),]) ;
                    ensures(equal(perm.view().value, v) &&
                            equal(self.id(), perm.view().patomic)) ;
                    opens_invariants_none() ;
                    self.ato.store(v, Ordering :: SeqCst) ;
                } #[inline(always)] #[verifier(external_body)]
                #[verifier(atomic)] pub fn
                compare_exchange(& self, #[proof] perm : & mut $p_ident,
                                 current : $value_ty, new : $value_ty) ->
                Result < $value_ty, $value_ty >
                {
                    requires([equal(self.id(), old(perm).view().patomic),]) ;
                    ensures(| ret : Result < $value_ty, $value_ty > |
                            equal(self.id(), perm.view().patomic) && match ret
                            {
                                Result :: Ok(r) => current ==
                                old(perm).view().value &&
                                equal(perm.view().value, new) &&
                                equal(r, old(perm).view().value), Result ::
                                Err(r) => current != old(perm).view().value &&
                                equal(perm.view().value,
                                      old(perm).view().value) &&
                                equal(r, old(perm).view().value),
                            }) ; opens_invariants_none() ; match
                    self.ato.compare_exchange(current, new, Ordering ::
                                              SeqCst, Ordering :: SeqCst)
                    { Ok(x) => Result :: Ok(x), Err(x) => Result :: Err(x), }
                } #[inline(always)] #[verifier(external_body)]
                #[verifier(atomic)] pub fn
                compare_exchange_weak(& self, #[proof] perm : & mut $p_ident,
                                      current : $value_ty, new : $value_ty) ->
                Result < $value_ty, $value_ty >
                {
                    requires([equal(self.id(), old(perm).view().patomic),]) ;
                    ensures(| ret : Result < $value_ty, $value_ty > |
                            equal(self.id(), perm.view().patomic) && match ret
                            {
                                Result :: Ok(r) => current ==
                                old(perm).view().value &&
                                equal(perm.view().value, new) &&
                                equal(r, old(perm).view().value), Result ::
                                Err(r) =>
                                equal(perm.view().value,
                                      old(perm).view().value) &&
                                equal(r, old(perm).view().value),
                            }) ; opens_invariants_none() ; match
                    self.ato.compare_exchange_weak(current, new, Ordering ::
                                                   SeqCst, Ordering :: SeqCst)
                    { Ok(x) => Result :: Ok(x), Err(x) => Result :: Err(x), }
                } #[inline(always)] #[verifier(external_body)]
                #[verifier(atomic)] pub fn
                swap(& self, #[proof] perm : & mut $p_ident, v : $value_ty) ->
                $value_ty
                {
                    requires([equal(self.id(), old(perm).view().patomic),]) ;
                    ensures(| ret : $value_ty | equal(perm.view().value, v) &&
                            equal(old(perm).view().value, ret) &&
                            equal(self.id(), perm.view().patomic)) ;
                    opens_invariants_none() ; return
                    self.ato.swap(v, Ordering :: SeqCst) ;
                } #[inline(always)] #[verifier(external_body)] pub fn
                into_inner(self, #[proof] perm : $p_ident) -> $value_ty
                {
                    requires([equal(self.id(), perm.view().patomic),]) ;
                    ensures(| ret : $value_ty | equal(perm.view().value, ret))
                    ; opens_invariants_none() ; return self.ato.into_inner() ;
                }
            }
        }
        macro_rules! atomic_integer_methods {
            ($at_ident : ident, $p_ident : ident, $rust_ty : ty, $value_ty :
             ty, $wrap_add : ident, $wrap_sub : ident, $int_min : expr,
             $int_max : expr) =>
            {
                #[inline(always)] #[verifier(external_body)]
                #[verifier(atomic)] pub fn
                fetch_add_wrapping(& self, #[proof] perm : & mut $p_ident, n :
                                   $value_ty) -> $value_ty
                {
                    requires(equal(self.id(), old(perm).view().patomic)) ;
                    ensures(| ret : $value_ty |
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             spec_cast_integer :: < $value_ty, int >
                             (perm.view().value) ==
                             $wrap_add(spec_cast_integer(old(perm).view().value),
                                       spec_cast_integer(n)),]) ;
                    opens_invariants_none() ; return
                    self.ato.fetch_add(n, Ordering :: SeqCst) ;
                } #[inline(always)] #[verifier(external_body)]
                #[verifier(atomic)] pub fn
                fetch_sub_wrapping(& self, #[proof] perm : & mut $p_ident, n :
                                   $value_ty) -> $value_ty
                {
                    requires(equal(self.id(), old(perm).view().patomic)) ;
                    ensures(| ret : $value_ty |
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             spec_cast_integer :: < $value_ty, int >
                             (perm.view().value) ==
                             $wrap_sub(spec_cast_integer :: < $value_ty, int >
                                       (old(perm).view().value),
                                       spec_cast_integer(n)),]) ;
                    opens_invariants_none() ; return
                    self.ato.fetch_sub(n, Ordering :: SeqCst) ;
                } #[inline(always)] #[verifier(atomic)] pub fn
                fetch_add(& self, #[proof] perm : & mut $p_ident, n :
                          $value_ty) -> $value_ty
                {
                    requires([equal(self.id(), old(perm).view().patomic),
                              $int_min <= old(perm).view().value.spec_add(n),
                              old(perm).view().value.spec_add(n) <= $int_max])
                    ;
                    ensures(| ret : $value_ty |
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value == old(perm).view().value +
                             n,]) ; opens_invariants_none() ;
                    self.fetch_add_wrapping(& mut * perm, n)
                } #[inline(always)] #[verifier(atomic)] pub fn
                fetch_sub(& self, #[proof] perm : & mut $p_ident, n :
                          $value_ty) -> $value_ty
                {
                    requires([equal(self.id(), old(perm).view().patomic),
                              $int_min <= old(perm).view().value.spec_sub(n),
                              old(perm).view().value.spec_sub(n) <=
                              $int_max,]) ;
                    ensures(| ret : $value_ty |
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value == old(perm).view().value -
                             n,]) ; opens_invariants_none() ;
                    self.fetch_sub_wrapping(& mut * perm, n)
                } #[inline(always)] #[verifier(external_body)]
                #[verifier(atomic)] pub fn
                fetch_and(& self, #[proof] perm : & mut $p_ident, n :
                          $value_ty) -> $value_ty
                {
                    requires(equal(self.id(), old(perm).view().patomic)) ;
                    ensures(| ret : $value_ty |
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                             (old(perm).view().value & n),]) ;
                    opens_invariants_none() ; return
                    self.ato.fetch_and(n, Ordering :: SeqCst) ;
                } #[inline(always)] #[verifier(external_body)]
                #[verifier(atomic)] pub fn
                fetch_or(& self, #[proof] perm : & mut $p_ident, n :
                         $value_ty) -> $value_ty
                {
                    requires(equal(self.id(), old(perm).view().patomic)) ;
                    ensures(| ret : $value_ty |
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                             (old(perm).view().value | n),]) ;
                    opens_invariants_none() ; return
                    self.ato.fetch_or(n, Ordering :: SeqCst) ;
                } #[inline(always)] #[verifier(external_body)]
                #[verifier(atomic)] pub fn
                fetch_xor(& self, #[proof] perm : & mut $p_ident, n :
                          $value_ty) -> $value_ty
                {
                    requires(equal(self.id(), old(perm).view().patomic)) ;
                    ensures(| ret : $value_ty |
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                             (old(perm).view().value ^ n),]) ;
                    opens_invariants_none() ; return
                    self.ato.fetch_or(n, Ordering :: SeqCst) ;
                } #[inline(always)] #[verifier(external_body)]
                #[verifier(atomic)] pub fn
                fetch_nand(& self, #[proof] perm : & mut $p_ident, n :
                           $value_ty) -> $value_ty
                {
                    requires(equal(self.id(), old(perm).view().patomic)) ;
                    ensures(| ret : $value_ty |
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==!
                             (old(perm).view().value & n),]) ;
                    opens_invariants_none() ; return
                    self.ato.fetch_nand(n, Ordering :: SeqCst) ;
                } #[inline(always)] #[verifier(external_body)]
                #[verifier(atomic)] pub fn
                fetch_max(& self, #[proof] perm : & mut $p_ident, n :
                          $value_ty) -> $value_ty
                {
                    requires(equal(self.id(), old(perm).view().patomic)) ;
                    ensures(| ret : $value_ty |
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                             (if old(perm).view().value > n
                              { old(perm).view().value } else { n }),]) ;
                    opens_invariants_none() ; return
                    self.ato.fetch_max(n, Ordering :: SeqCst) ;
                } #[inline(always)] #[verifier(external_body)]
                #[verifier(atomic)] pub fn
                fetch_min(& self, #[proof] perm : & mut $p_ident, n :
                          $value_ty) -> $value_ty
                {
                    requires(equal(self.id(), old(perm).view().patomic)) ;
                    ensures(| ret : $value_ty |
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                             (if old(perm).view().value < n
                              { old(perm).view().value } else { n }),]) ;
                    opens_invariants_none() ; return
                    self.ato.fetch_min(n, Ordering :: SeqCst) ;
                }
            }
        }
        macro_rules! atomic_bool_methods {
            ($at_ident : ident, $p_ident : ident, $rust_ty : ty, $value_ty :
             ty) =>
            {
                #[inline(always)] #[verifier(external_body)]
                #[verifier(atomic)] pub fn
                fetch_and(& self, #[proof] perm : & mut $p_ident, n :
                          $value_ty) -> $value_ty
                {
                    requires([equal(self.id(), old(perm).view().patomic),]) ;
                    ensures(| ret : $value_ty |
                            equal(old(perm).view().value, ret) &&
                            perm.view().patomic == old(perm).view().patomic &&
                            perm.view().value ==
                            (old(perm).view().value && n)) ;
                    opens_invariants_none() ; return
                    self.ato.fetch_and(n, Ordering :: SeqCst) ;
                } #[inline(always)] #[verifier(external_body)]
                #[verifier(atomic)] pub fn
                fetch_or(& self, #[proof] perm : & mut $p_ident, n :
                         $value_ty) -> $value_ty
                {
                    requires([equal(self.id(), old(perm).view().patomic),]) ;
                    ensures(| ret : $value_ty |
                            equal(old(perm).view().value, ret) &&
                            perm.view().patomic == old(perm).view().patomic &&
                            perm.view().value ==
                            (old(perm).view().value || n)) ;
                    opens_invariants_none() ; return
                    self.ato.fetch_or(n, Ordering :: SeqCst) ;
                } #[inline(always)] #[verifier(external_body)]
                #[verifier(atomic)] pub fn
                fetch_xor(& self, #[proof] perm : & mut $p_ident, n :
                          $value_ty) -> $value_ty
                {
                    requires([equal(self.id(), old(perm).view().patomic),]) ;
                    ensures(| ret : $value_ty |
                            equal(old(perm).view().value, ret) &&
                            perm.view().patomic == old(perm).view().patomic &&
                            perm.view().value ==
                            ((old(perm).view().value &&! n) ||
                             (! old(perm).view().value && n))) ;
                    opens_invariants_none() ; return
                    self.ato.fetch_or(n, Ordering :: SeqCst) ;
                } #[inline(always)] #[verifier(external_body)]
                #[verifier(atomic)] pub fn
                fetch_nand(& self, #[proof] perm : & mut $p_ident, n :
                           $value_ty) -> $value_ty
                {
                    requires([equal(self.id(), old(perm).view().patomic),]) ;
                    ensures(| ret : $value_ty |
                            equal(old(perm).view().value, ret) &&
                            perm.view().patomic == old(perm).view().patomic &&
                            perm.view().value ==!
                            (old(perm).view().value && n)) ;
                    opens_invariants_none() ; return
                    self.ato.fetch_nand(n, Ordering :: SeqCst) ;
                }
            }
        }
        #[verifier(external_body)]
        pub struct PAtomicBool {
            ato: AtomicBool,
        }
        #[proof]
        #[verifier(external_body)]
        pub struct PermissionBool {
            no_copy: NoCopy,
        }
        #[spec]
        pub struct PermissionDataBool {
            #[spec]
            pub patomic: int,
            #[spec]
            pub value: bool,
        }
        impl PermissionBool {
            #[spec]
            #[verifier(external_body)]
            pub fn view(self) -> PermissionDataBool {
                ::core::panicking::panic("not implemented");
            }
            #[spec]
            #[verifier(publish)]
            pub fn is_for(&self, patomic: PAtomicBool) -> bool {
                self.view().patomic == patomic.id()
            }
            #[spec]
            #[verifier(publish)]
            pub fn points_to(&self, v: bool) -> bool {
                self.view().value == v
            }
        }
        impl PAtomicBool {
            #[spec]
            #[verifier(external_body)]
            pub fn id(&self) -> int {
                ::core::panicking::panic("not implemented")
            }
            #[inline(always)]
            #[verifier(external_body)]
            pub fn new(i: bool) -> (PAtomicBool, Proof<PermissionBool>) {
                ensures(|res: (PAtomicBool, Proof<PermissionBool>)|
                            equal(res.1.0.view(),
                                  PermissionDataBool{patomic: res.0.id(),
                                                     value: i,}));
                let p = PAtomicBool{ato: <AtomicBool>::new(i),};
                let Proof(t) = exec_proof_from_false();
                (p, Proof(t))
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn load(&self, #[proof] perm: &PermissionBool) -> bool {
                requires([equal(self.id(), perm.view().patomic)]);
                ensures(|ret: bool| equal(perm.view().value, ret));
                opens_invariants_none();
                return self.ato.load(Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn store(&self, #[proof] perm: &mut PermissionBool, v: bool) {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(equal(perm.view().value, v) &&
                            equal(self.id(), perm.view().patomic));
                opens_invariants_none();
                self.ato.store(v, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn compare_exchange(&self, #[proof] perm: &mut PermissionBool,
                                    current: bool, new: bool)
             -> Result<bool, bool> {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: Result<bool, bool>|
                            equal(self.id(), perm.view().patomic) &&
                                match ret {
                                    Result::Ok(r) =>
                                    current == old(perm).view().value &&
                                        equal(perm.view().value, new) &&
                                        equal(r, old(perm).view().value),
                                    Result::Err(r) =>
                                    current != old(perm).view().value &&
                                        equal(perm.view().value,
                                              old(perm).view().value) &&
                                        equal(r, old(perm).view().value),
                                });
                opens_invariants_none();
                match self.ato.compare_exchange(current, new,
                                                Ordering::SeqCst,
                                                Ordering::SeqCst) {
                    Ok(x) => Result::Ok(x),
                    Err(x) => Result::Err(x),
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn compare_exchange_weak(&self,
                                         #[proof] perm: &mut PermissionBool,
                                         current: bool, new: bool)
             -> Result<bool, bool> {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: Result<bool, bool>|
                            equal(self.id(), perm.view().patomic) &&
                                match ret {
                                    Result::Ok(r) =>
                                    current == old(perm).view().value &&
                                        equal(perm.view().value, new) &&
                                        equal(r, old(perm).view().value),
                                    Result::Err(r) =>
                                    equal(perm.view().value,
                                          old(perm).view().value) &&
                                        equal(r, old(perm).view().value),
                                });
                opens_invariants_none();
                match self.ato.compare_exchange_weak(current, new,
                                                     Ordering::SeqCst,
                                                     Ordering::SeqCst) {
                    Ok(x) => Result::Ok(x),
                    Err(x) => Result::Err(x),
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn swap(&self, #[proof] perm: &mut PermissionBool, v: bool)
             -> bool {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: bool|
                            equal(perm.view().value, v) &&
                                equal(old(perm).view().value, ret) &&
                                equal(self.id(), perm.view().patomic));
                opens_invariants_none();
                return self.ato.swap(v, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            pub fn into_inner(self, #[proof] perm: PermissionBool) -> bool {
                requires([equal(self.id(), perm.view().patomic)]);
                ensures(|ret: bool| equal(perm.view().value, ret));
                opens_invariants_none();
                return self.ato.into_inner();
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_and(&self, #[proof] perm: &mut PermissionBool,
                             n: bool) -> bool {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: bool|
                            equal(old(perm).view().value, ret) &&
                                perm.view().patomic ==
                                    old(perm).view().patomic &&
                                perm.view().value ==
                                    (old(perm).view().value && n));
                opens_invariants_none();
                return self.ato.fetch_and(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_or(&self, #[proof] perm: &mut PermissionBool,
                            n: bool) -> bool {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: bool|
                            equal(old(perm).view().value, ret) &&
                                perm.view().patomic ==
                                    old(perm).view().patomic &&
                                perm.view().value ==
                                    (old(perm).view().value || n));
                opens_invariants_none();
                return self.ato.fetch_or(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_xor(&self, #[proof] perm: &mut PermissionBool,
                             n: bool) -> bool {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: bool|
                            equal(old(perm).view().value, ret) &&
                                perm.view().patomic ==
                                    old(perm).view().patomic &&
                                perm.view().value ==
                                    ((old(perm).view().value && !n) ||
                                         (!old(perm).view().value && n)));
                opens_invariants_none();
                return self.ato.fetch_or(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_nand(&self, #[proof] perm: &mut PermissionBool,
                              n: bool) -> bool {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: bool|
                            equal(old(perm).view().value, ret) &&
                                perm.view().patomic ==
                                    old(perm).view().patomic &&
                                perm.view().value ==
                                    !(old(perm).view().value && n));
                opens_invariants_none();
                return self.ato.fetch_nand(n, Ordering::SeqCst);
            }
        }
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn wrapping_add_u8(a: int, b: int) -> int {
            if ((a).spec_add(b)).spec_gt(spec_literal_int("255")) {
                ((a).spec_add(b)).spec_sub((((spec_literal_int("255")).spec_sub(spec_literal_int("0"))).spec_add(::builtin::spec_literal_nat("1"))))
            } else { (a).spec_add(b) }
        }
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn wrapping_sub_u8(a: int, b: int) -> int {
            if ((a).spec_sub(b)).spec_lt(spec_literal_int("0")) {
                ((a).spec_sub(b)).spec_add((((spec_literal_int("255")).spec_sub(spec_literal_int("0"))).spec_add(::builtin::spec_literal_nat("1"))))
            } else { (a).spec_sub(b) }
        }
        #[verifier(external_body)]
        pub struct PAtomicU8 {
            ato: AtomicU8,
        }
        #[proof]
        #[verifier(external_body)]
        pub struct PermissionU8 {
            no_copy: NoCopy,
        }
        #[spec]
        pub struct PermissionDataU8 {
            #[spec]
            pub patomic: int,
            #[spec]
            pub value: u8,
        }
        impl PermissionU8 {
            #[spec]
            #[verifier(external_body)]
            pub fn view(self) -> PermissionDataU8 {
                ::core::panicking::panic("not implemented");
            }
            #[spec]
            #[verifier(publish)]
            pub fn is_for(&self, patomic: PAtomicU8) -> bool {
                self.view().patomic == patomic.id()
            }
            #[spec]
            #[verifier(publish)]
            pub fn points_to(&self, v: u8) -> bool { self.view().value == v }
        }
        impl PAtomicU8 {
            #[spec]
            #[verifier(external_body)]
            pub fn id(&self) -> int {
                ::core::panicking::panic("not implemented")
            }
            #[inline(always)]
            #[verifier(external_body)]
            pub fn new(i: u8) -> (PAtomicU8, Proof<PermissionU8>) {
                ensures(|res: (PAtomicU8, Proof<PermissionU8>)|
                            equal(res.1.0.view(),
                                  PermissionDataU8{patomic: res.0.id(),
                                                   value: i,}));
                let p = PAtomicU8{ato: <AtomicU8>::new(i),};
                let Proof(t) = exec_proof_from_false();
                (p, Proof(t))
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn load(&self, #[proof] perm: &PermissionU8) -> u8 {
                requires([equal(self.id(), perm.view().patomic)]);
                ensures(|ret: u8| equal(perm.view().value, ret));
                opens_invariants_none();
                return self.ato.load(Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn store(&self, #[proof] perm: &mut PermissionU8, v: u8) {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(equal(perm.view().value, v) &&
                            equal(self.id(), perm.view().patomic));
                opens_invariants_none();
                self.ato.store(v, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn compare_exchange(&self, #[proof] perm: &mut PermissionU8,
                                    current: u8, new: u8) -> Result<u8, u8> {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: Result<u8, u8>|
                            equal(self.id(), perm.view().patomic) &&
                                match ret {
                                    Result::Ok(r) =>
                                    current == old(perm).view().value &&
                                        equal(perm.view().value, new) &&
                                        equal(r, old(perm).view().value),
                                    Result::Err(r) =>
                                    current != old(perm).view().value &&
                                        equal(perm.view().value,
                                              old(perm).view().value) &&
                                        equal(r, old(perm).view().value),
                                });
                opens_invariants_none();
                match self.ato.compare_exchange(current, new,
                                                Ordering::SeqCst,
                                                Ordering::SeqCst) {
                    Ok(x) => Result::Ok(x),
                    Err(x) => Result::Err(x),
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn compare_exchange_weak(&self,
                                         #[proof] perm: &mut PermissionU8,
                                         current: u8, new: u8)
             -> Result<u8, u8> {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: Result<u8, u8>|
                            equal(self.id(), perm.view().patomic) &&
                                match ret {
                                    Result::Ok(r) =>
                                    current == old(perm).view().value &&
                                        equal(perm.view().value, new) &&
                                        equal(r, old(perm).view().value),
                                    Result::Err(r) =>
                                    equal(perm.view().value,
                                          old(perm).view().value) &&
                                        equal(r, old(perm).view().value),
                                });
                opens_invariants_none();
                match self.ato.compare_exchange_weak(current, new,
                                                     Ordering::SeqCst,
                                                     Ordering::SeqCst) {
                    Ok(x) => Result::Ok(x),
                    Err(x) => Result::Err(x),
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn swap(&self, #[proof] perm: &mut PermissionU8, v: u8)
             -> u8 {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: u8|
                            equal(perm.view().value, v) &&
                                equal(old(perm).view().value, ret) &&
                                equal(self.id(), perm.view().patomic));
                opens_invariants_none();
                return self.ato.swap(v, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            pub fn into_inner(self, #[proof] perm: PermissionU8) -> u8 {
                requires([equal(self.id(), perm.view().patomic)]);
                ensures(|ret: u8| equal(perm.view().value, ret));
                opens_invariants_none();
                return self.ato.into_inner();
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_add_wrapping(&self, #[proof] perm: &mut PermissionU8,
                                      n: u8) -> u8 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             spec_cast_integer::<u8, int>(perm.view().value)
                                 ==
                                 wrapping_add_u8(spec_cast_integer(old(perm).view().value),
                                                 spec_cast_integer(n))]);
                opens_invariants_none();
                return self.ato.fetch_add(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_sub_wrapping(&self, #[proof] perm: &mut PermissionU8,
                                      n: u8) -> u8 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             spec_cast_integer::<u8, int>(perm.view().value)
                                 ==
                                 wrapping_sub_u8(spec_cast_integer::<u8,
                                                                     int>(old(perm).view().value),
                                                 spec_cast_integer(n))]);
                opens_invariants_none();
                return self.ato.fetch_sub(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(atomic)]
            pub fn fetch_add(&self, #[proof] perm: &mut PermissionU8, n: u8)
             -> u8 {
                requires([equal(self.id(), old(perm).view().patomic),
                          spec_literal_int("0") <=
                              old(perm).view().value.spec_add(n),
                          old(perm).view().value.spec_add(n) <=
                              spec_literal_int("255")]);
                ensures(|ret: u8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 old(perm).view().value + n]);
                opens_invariants_none();
                self.fetch_add_wrapping(&mut *perm, n)
            }
            #[inline(always)]
            #[verifier(atomic)]
            pub fn fetch_sub(&self, #[proof] perm: &mut PermissionU8, n: u8)
             -> u8 {
                requires([equal(self.id(), old(perm).view().patomic),
                          spec_literal_int("0") <=
                              old(perm).view().value.spec_sub(n),
                          old(perm).view().value.spec_sub(n) <=
                              spec_literal_int("255")]);
                ensures(|ret: u8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 old(perm).view().value - n]);
                opens_invariants_none();
                self.fetch_sub_wrapping(&mut *perm, n)
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_and(&self, #[proof] perm: &mut PermissionU8, n: u8)
             -> u8 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value & n)]);
                opens_invariants_none();
                return self.ato.fetch_and(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_or(&self, #[proof] perm: &mut PermissionU8, n: u8)
             -> u8 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value | n)]);
                opens_invariants_none();
                return self.ato.fetch_or(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_xor(&self, #[proof] perm: &mut PermissionU8, n: u8)
             -> u8 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value ^ n)]);
                opens_invariants_none();
                return self.ato.fetch_or(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_nand(&self, #[proof] perm: &mut PermissionU8, n: u8)
             -> u8 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 !(old(perm).view().value & n)]);
                opens_invariants_none();
                return self.ato.fetch_nand(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_max(&self, #[proof] perm: &mut PermissionU8, n: u8)
             -> u8 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (if old(perm).view().value > n {
                                      old(perm).view().value
                                  } else { n })]);
                opens_invariants_none();
                return self.ato.fetch_max(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_min(&self, #[proof] perm: &mut PermissionU8, n: u8)
             -> u8 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (if old(perm).view().value < n {
                                      old(perm).view().value
                                  } else { n })]);
                opens_invariants_none();
                return self.ato.fetch_min(n, Ordering::SeqCst);
            }
        }
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn wrapping_add_u16(a: int, b: int) -> int {
            if ((a).spec_add(b)).spec_gt(spec_literal_int("65535")) {
                ((a).spec_add(b)).spec_sub((((spec_literal_int("65535")).spec_sub(spec_literal_int("0"))).spec_add(::builtin::spec_literal_nat("1"))))
            } else { (a).spec_add(b) }
        }
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn wrapping_sub_u16(a: int, b: int) -> int {
            if ((a).spec_sub(b)).spec_lt(spec_literal_int("0")) {
                ((a).spec_sub(b)).spec_add((((spec_literal_int("65535")).spec_sub(spec_literal_int("0"))).spec_add(::builtin::spec_literal_nat("1"))))
            } else { (a).spec_sub(b) }
        }
        #[verifier(external_body)]
        pub struct PAtomicU16 {
            ato: AtomicU16,
        }
        #[proof]
        #[verifier(external_body)]
        pub struct PermissionU16 {
            no_copy: NoCopy,
        }
        #[spec]
        pub struct PermissionDataU16 {
            #[spec]
            pub patomic: int,
            #[spec]
            pub value: u16,
        }
        impl PermissionU16 {
            #[spec]
            #[verifier(external_body)]
            pub fn view(self) -> PermissionDataU16 {
                ::core::panicking::panic("not implemented");
            }
            #[spec]
            #[verifier(publish)]
            pub fn is_for(&self, patomic: PAtomicU16) -> bool {
                self.view().patomic == patomic.id()
            }
            #[spec]
            #[verifier(publish)]
            pub fn points_to(&self, v: u16) -> bool { self.view().value == v }
        }
        impl PAtomicU16 {
            #[spec]
            #[verifier(external_body)]
            pub fn id(&self) -> int {
                ::core::panicking::panic("not implemented")
            }
            #[inline(always)]
            #[verifier(external_body)]
            pub fn new(i: u16) -> (PAtomicU16, Proof<PermissionU16>) {
                ensures(|res: (PAtomicU16, Proof<PermissionU16>)|
                            equal(res.1.0.view(),
                                  PermissionDataU16{patomic: res.0.id(),
                                                    value: i,}));
                let p = PAtomicU16{ato: <AtomicU16>::new(i),};
                let Proof(t) = exec_proof_from_false();
                (p, Proof(t))
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn load(&self, #[proof] perm: &PermissionU16) -> u16 {
                requires([equal(self.id(), perm.view().patomic)]);
                ensures(|ret: u16| equal(perm.view().value, ret));
                opens_invariants_none();
                return self.ato.load(Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn store(&self, #[proof] perm: &mut PermissionU16, v: u16) {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(equal(perm.view().value, v) &&
                            equal(self.id(), perm.view().patomic));
                opens_invariants_none();
                self.ato.store(v, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn compare_exchange(&self, #[proof] perm: &mut PermissionU16,
                                    current: u16, new: u16)
             -> Result<u16, u16> {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: Result<u16, u16>|
                            equal(self.id(), perm.view().patomic) &&
                                match ret {
                                    Result::Ok(r) =>
                                    current == old(perm).view().value &&
                                        equal(perm.view().value, new) &&
                                        equal(r, old(perm).view().value),
                                    Result::Err(r) =>
                                    current != old(perm).view().value &&
                                        equal(perm.view().value,
                                              old(perm).view().value) &&
                                        equal(r, old(perm).view().value),
                                });
                opens_invariants_none();
                match self.ato.compare_exchange(current, new,
                                                Ordering::SeqCst,
                                                Ordering::SeqCst) {
                    Ok(x) => Result::Ok(x),
                    Err(x) => Result::Err(x),
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn compare_exchange_weak(&self,
                                         #[proof] perm: &mut PermissionU16,
                                         current: u16, new: u16)
             -> Result<u16, u16> {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: Result<u16, u16>|
                            equal(self.id(), perm.view().patomic) &&
                                match ret {
                                    Result::Ok(r) =>
                                    current == old(perm).view().value &&
                                        equal(perm.view().value, new) &&
                                        equal(r, old(perm).view().value),
                                    Result::Err(r) =>
                                    equal(perm.view().value,
                                          old(perm).view().value) &&
                                        equal(r, old(perm).view().value),
                                });
                opens_invariants_none();
                match self.ato.compare_exchange_weak(current, new,
                                                     Ordering::SeqCst,
                                                     Ordering::SeqCst) {
                    Ok(x) => Result::Ok(x),
                    Err(x) => Result::Err(x),
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn swap(&self, #[proof] perm: &mut PermissionU16, v: u16)
             -> u16 {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: u16|
                            equal(perm.view().value, v) &&
                                equal(old(perm).view().value, ret) &&
                                equal(self.id(), perm.view().patomic));
                opens_invariants_none();
                return self.ato.swap(v, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            pub fn into_inner(self, #[proof] perm: PermissionU16) -> u16 {
                requires([equal(self.id(), perm.view().patomic)]);
                ensures(|ret: u16| equal(perm.view().value, ret));
                opens_invariants_none();
                return self.ato.into_inner();
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_add_wrapping(&self,
                                      #[proof] perm: &mut PermissionU16,
                                      n: u16) -> u16 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             spec_cast_integer::<u16, int>(perm.view().value)
                                 ==
                                 wrapping_add_u16(spec_cast_integer(old(perm).view().value),
                                                  spec_cast_integer(n))]);
                opens_invariants_none();
                return self.ato.fetch_add(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_sub_wrapping(&self,
                                      #[proof] perm: &mut PermissionU16,
                                      n: u16) -> u16 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             spec_cast_integer::<u16, int>(perm.view().value)
                                 ==
                                 wrapping_sub_u16(spec_cast_integer::<u16,
                                                                      int>(old(perm).view().value),
                                                  spec_cast_integer(n))]);
                opens_invariants_none();
                return self.ato.fetch_sub(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(atomic)]
            pub fn fetch_add(&self, #[proof] perm: &mut PermissionU16, n: u16)
             -> u16 {
                requires([equal(self.id(), old(perm).view().patomic),
                          spec_literal_int("0") <=
                              old(perm).view().value.spec_add(n),
                          old(perm).view().value.spec_add(n) <=
                              spec_literal_int("65535")]);
                ensures(|ret: u16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 old(perm).view().value + n]);
                opens_invariants_none();
                self.fetch_add_wrapping(&mut *perm, n)
            }
            #[inline(always)]
            #[verifier(atomic)]
            pub fn fetch_sub(&self, #[proof] perm: &mut PermissionU16, n: u16)
             -> u16 {
                requires([equal(self.id(), old(perm).view().patomic),
                          spec_literal_int("0") <=
                              old(perm).view().value.spec_sub(n),
                          old(perm).view().value.spec_sub(n) <=
                              spec_literal_int("65535")]);
                ensures(|ret: u16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 old(perm).view().value - n]);
                opens_invariants_none();
                self.fetch_sub_wrapping(&mut *perm, n)
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_and(&self, #[proof] perm: &mut PermissionU16, n: u16)
             -> u16 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value & n)]);
                opens_invariants_none();
                return self.ato.fetch_and(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_or(&self, #[proof] perm: &mut PermissionU16, n: u16)
             -> u16 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value | n)]);
                opens_invariants_none();
                return self.ato.fetch_or(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_xor(&self, #[proof] perm: &mut PermissionU16, n: u16)
             -> u16 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value ^ n)]);
                opens_invariants_none();
                return self.ato.fetch_or(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_nand(&self, #[proof] perm: &mut PermissionU16,
                              n: u16) -> u16 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 !(old(perm).view().value & n)]);
                opens_invariants_none();
                return self.ato.fetch_nand(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_max(&self, #[proof] perm: &mut PermissionU16, n: u16)
             -> u16 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (if old(perm).view().value > n {
                                      old(perm).view().value
                                  } else { n })]);
                opens_invariants_none();
                return self.ato.fetch_max(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_min(&self, #[proof] perm: &mut PermissionU16, n: u16)
             -> u16 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (if old(perm).view().value < n {
                                      old(perm).view().value
                                  } else { n })]);
                opens_invariants_none();
                return self.ato.fetch_min(n, Ordering::SeqCst);
            }
        }
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn wrapping_add_u32(a: int, b: int) -> int {
            if ((a).spec_add(b)).spec_gt(spec_literal_int("4294967295")) {
                ((a).spec_add(b)).spec_sub((((spec_literal_int("4294967295")).spec_sub(spec_literal_int("0"))).spec_add(::builtin::spec_literal_nat("1"))))
            } else { (a).spec_add(b) }
        }
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn wrapping_sub_u32(a: int, b: int) -> int {
            if ((a).spec_sub(b)).spec_lt(spec_literal_int("0")) {
                ((a).spec_sub(b)).spec_add((((spec_literal_int("4294967295")).spec_sub(spec_literal_int("0"))).spec_add(::builtin::spec_literal_nat("1"))))
            } else { (a).spec_sub(b) }
        }
        #[verifier(external_body)]
        pub struct PAtomicU32 {
            ato: AtomicU32,
        }
        #[proof]
        #[verifier(external_body)]
        pub struct PermissionU32 {
            no_copy: NoCopy,
        }
        #[spec]
        pub struct PermissionDataU32 {
            #[spec]
            pub patomic: int,
            #[spec]
            pub value: u32,
        }
        impl PermissionU32 {
            #[spec]
            #[verifier(external_body)]
            pub fn view(self) -> PermissionDataU32 {
                ::core::panicking::panic("not implemented");
            }
            #[spec]
            #[verifier(publish)]
            pub fn is_for(&self, patomic: PAtomicU32) -> bool {
                self.view().patomic == patomic.id()
            }
            #[spec]
            #[verifier(publish)]
            pub fn points_to(&self, v: u32) -> bool { self.view().value == v }
        }
        impl PAtomicU32 {
            #[spec]
            #[verifier(external_body)]
            pub fn id(&self) -> int {
                ::core::panicking::panic("not implemented")
            }
            #[inline(always)]
            #[verifier(external_body)]
            pub fn new(i: u32) -> (PAtomicU32, Proof<PermissionU32>) {
                ensures(|res: (PAtomicU32, Proof<PermissionU32>)|
                            equal(res.1.0.view(),
                                  PermissionDataU32{patomic: res.0.id(),
                                                    value: i,}));
                let p = PAtomicU32{ato: <AtomicU32>::new(i),};
                let Proof(t) = exec_proof_from_false();
                (p, Proof(t))
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn load(&self, #[proof] perm: &PermissionU32) -> u32 {
                requires([equal(self.id(), perm.view().patomic)]);
                ensures(|ret: u32| equal(perm.view().value, ret));
                opens_invariants_none();
                return self.ato.load(Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn store(&self, #[proof] perm: &mut PermissionU32, v: u32) {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(equal(perm.view().value, v) &&
                            equal(self.id(), perm.view().patomic));
                opens_invariants_none();
                self.ato.store(v, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn compare_exchange(&self, #[proof] perm: &mut PermissionU32,
                                    current: u32, new: u32)
             -> Result<u32, u32> {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: Result<u32, u32>|
                            equal(self.id(), perm.view().patomic) &&
                                match ret {
                                    Result::Ok(r) =>
                                    current == old(perm).view().value &&
                                        equal(perm.view().value, new) &&
                                        equal(r, old(perm).view().value),
                                    Result::Err(r) =>
                                    current != old(perm).view().value &&
                                        equal(perm.view().value,
                                              old(perm).view().value) &&
                                        equal(r, old(perm).view().value),
                                });
                opens_invariants_none();
                match self.ato.compare_exchange(current, new,
                                                Ordering::SeqCst,
                                                Ordering::SeqCst) {
                    Ok(x) => Result::Ok(x),
                    Err(x) => Result::Err(x),
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn compare_exchange_weak(&self,
                                         #[proof] perm: &mut PermissionU32,
                                         current: u32, new: u32)
             -> Result<u32, u32> {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: Result<u32, u32>|
                            equal(self.id(), perm.view().patomic) &&
                                match ret {
                                    Result::Ok(r) =>
                                    current == old(perm).view().value &&
                                        equal(perm.view().value, new) &&
                                        equal(r, old(perm).view().value),
                                    Result::Err(r) =>
                                    equal(perm.view().value,
                                          old(perm).view().value) &&
                                        equal(r, old(perm).view().value),
                                });
                opens_invariants_none();
                match self.ato.compare_exchange_weak(current, new,
                                                     Ordering::SeqCst,
                                                     Ordering::SeqCst) {
                    Ok(x) => Result::Ok(x),
                    Err(x) => Result::Err(x),
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn swap(&self, #[proof] perm: &mut PermissionU32, v: u32)
             -> u32 {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: u32|
                            equal(perm.view().value, v) &&
                                equal(old(perm).view().value, ret) &&
                                equal(self.id(), perm.view().patomic));
                opens_invariants_none();
                return self.ato.swap(v, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            pub fn into_inner(self, #[proof] perm: PermissionU32) -> u32 {
                requires([equal(self.id(), perm.view().patomic)]);
                ensures(|ret: u32| equal(perm.view().value, ret));
                opens_invariants_none();
                return self.ato.into_inner();
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_add_wrapping(&self,
                                      #[proof] perm: &mut PermissionU32,
                                      n: u32) -> u32 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             spec_cast_integer::<u32, int>(perm.view().value)
                                 ==
                                 wrapping_add_u32(spec_cast_integer(old(perm).view().value),
                                                  spec_cast_integer(n))]);
                opens_invariants_none();
                return self.ato.fetch_add(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_sub_wrapping(&self,
                                      #[proof] perm: &mut PermissionU32,
                                      n: u32) -> u32 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             spec_cast_integer::<u32, int>(perm.view().value)
                                 ==
                                 wrapping_sub_u32(spec_cast_integer::<u32,
                                                                      int>(old(perm).view().value),
                                                  spec_cast_integer(n))]);
                opens_invariants_none();
                return self.ato.fetch_sub(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(atomic)]
            pub fn fetch_add(&self, #[proof] perm: &mut PermissionU32, n: u32)
             -> u32 {
                requires([equal(self.id(), old(perm).view().patomic),
                          spec_literal_int("0") <=
                              old(perm).view().value.spec_add(n),
                          old(perm).view().value.spec_add(n) <=
                              spec_literal_int("4294967295")]);
                ensures(|ret: u32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 old(perm).view().value + n]);
                opens_invariants_none();
                self.fetch_add_wrapping(&mut *perm, n)
            }
            #[inline(always)]
            #[verifier(atomic)]
            pub fn fetch_sub(&self, #[proof] perm: &mut PermissionU32, n: u32)
             -> u32 {
                requires([equal(self.id(), old(perm).view().patomic),
                          spec_literal_int("0") <=
                              old(perm).view().value.spec_sub(n),
                          old(perm).view().value.spec_sub(n) <=
                              spec_literal_int("4294967295")]);
                ensures(|ret: u32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 old(perm).view().value - n]);
                opens_invariants_none();
                self.fetch_sub_wrapping(&mut *perm, n)
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_and(&self, #[proof] perm: &mut PermissionU32, n: u32)
             -> u32 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value & n)]);
                opens_invariants_none();
                return self.ato.fetch_and(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_or(&self, #[proof] perm: &mut PermissionU32, n: u32)
             -> u32 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value | n)]);
                opens_invariants_none();
                return self.ato.fetch_or(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_xor(&self, #[proof] perm: &mut PermissionU32, n: u32)
             -> u32 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value ^ n)]);
                opens_invariants_none();
                return self.ato.fetch_or(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_nand(&self, #[proof] perm: &mut PermissionU32,
                              n: u32) -> u32 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 !(old(perm).view().value & n)]);
                opens_invariants_none();
                return self.ato.fetch_nand(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_max(&self, #[proof] perm: &mut PermissionU32, n: u32)
             -> u32 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (if old(perm).view().value > n {
                                      old(perm).view().value
                                  } else { n })]);
                opens_invariants_none();
                return self.ato.fetch_max(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_min(&self, #[proof] perm: &mut PermissionU32, n: u32)
             -> u32 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (if old(perm).view().value < n {
                                      old(perm).view().value
                                  } else { n })]);
                opens_invariants_none();
                return self.ato.fetch_min(n, Ordering::SeqCst);
            }
        }
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn wrapping_add_u64(a: int, b: int) -> int {
            if ((a).spec_add(b)).spec_gt(spec_literal_int("18446744073709551615"))
               {
                ((a).spec_add(b)).spec_sub((((spec_literal_int("18446744073709551615")).spec_sub(spec_literal_int("0"))).spec_add(::builtin::spec_literal_nat("1"))))
            } else { (a).spec_add(b) }
        }
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn wrapping_sub_u64(a: int, b: int) -> int {
            if ((a).spec_sub(b)).spec_lt(spec_literal_int("0")) {
                ((a).spec_sub(b)).spec_add((((spec_literal_int("18446744073709551615")).spec_sub(spec_literal_int("0"))).spec_add(::builtin::spec_literal_nat("1"))))
            } else { (a).spec_sub(b) }
        }
        #[verifier(external_body)]
        pub struct PAtomicU64 {
            ato: AtomicU64,
        }
        #[proof]
        #[verifier(external_body)]
        pub struct PermissionU64 {
            no_copy: NoCopy,
        }
        #[spec]
        pub struct PermissionDataU64 {
            #[spec]
            pub patomic: int,
            #[spec]
            pub value: u64,
        }
        impl PermissionU64 {
            #[spec]
            #[verifier(external_body)]
            pub fn view(self) -> PermissionDataU64 {
                ::core::panicking::panic("not implemented");
            }
            #[spec]
            #[verifier(publish)]
            pub fn is_for(&self, patomic: PAtomicU64) -> bool {
                self.view().patomic == patomic.id()
            }
            #[spec]
            #[verifier(publish)]
            pub fn points_to(&self, v: u64) -> bool { self.view().value == v }
        }
        impl PAtomicU64 {
            #[spec]
            #[verifier(external_body)]
            pub fn id(&self) -> int {
                ::core::panicking::panic("not implemented")
            }
            #[inline(always)]
            #[verifier(external_body)]
            pub fn new(i: u64) -> (PAtomicU64, Proof<PermissionU64>) {
                ensures(|res: (PAtomicU64, Proof<PermissionU64>)|
                            equal(res.1.0.view(),
                                  PermissionDataU64{patomic: res.0.id(),
                                                    value: i,}));
                let p = PAtomicU64{ato: <AtomicU64>::new(i),};
                let Proof(t) = exec_proof_from_false();
                (p, Proof(t))
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn load(&self, #[proof] perm: &PermissionU64) -> u64 {
                requires([equal(self.id(), perm.view().patomic)]);
                ensures(|ret: u64| equal(perm.view().value, ret));
                opens_invariants_none();
                return self.ato.load(Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn store(&self, #[proof] perm: &mut PermissionU64, v: u64) {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(equal(perm.view().value, v) &&
                            equal(self.id(), perm.view().patomic));
                opens_invariants_none();
                self.ato.store(v, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn compare_exchange(&self, #[proof] perm: &mut PermissionU64,
                                    current: u64, new: u64)
             -> Result<u64, u64> {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: Result<u64, u64>|
                            equal(self.id(), perm.view().patomic) &&
                                match ret {
                                    Result::Ok(r) =>
                                    current == old(perm).view().value &&
                                        equal(perm.view().value, new) &&
                                        equal(r, old(perm).view().value),
                                    Result::Err(r) =>
                                    current != old(perm).view().value &&
                                        equal(perm.view().value,
                                              old(perm).view().value) &&
                                        equal(r, old(perm).view().value),
                                });
                opens_invariants_none();
                match self.ato.compare_exchange(current, new,
                                                Ordering::SeqCst,
                                                Ordering::SeqCst) {
                    Ok(x) => Result::Ok(x),
                    Err(x) => Result::Err(x),
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn compare_exchange_weak(&self,
                                         #[proof] perm: &mut PermissionU64,
                                         current: u64, new: u64)
             -> Result<u64, u64> {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: Result<u64, u64>|
                            equal(self.id(), perm.view().patomic) &&
                                match ret {
                                    Result::Ok(r) =>
                                    current == old(perm).view().value &&
                                        equal(perm.view().value, new) &&
                                        equal(r, old(perm).view().value),
                                    Result::Err(r) =>
                                    equal(perm.view().value,
                                          old(perm).view().value) &&
                                        equal(r, old(perm).view().value),
                                });
                opens_invariants_none();
                match self.ato.compare_exchange_weak(current, new,
                                                     Ordering::SeqCst,
                                                     Ordering::SeqCst) {
                    Ok(x) => Result::Ok(x),
                    Err(x) => Result::Err(x),
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn swap(&self, #[proof] perm: &mut PermissionU64, v: u64)
             -> u64 {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: u64|
                            equal(perm.view().value, v) &&
                                equal(old(perm).view().value, ret) &&
                                equal(self.id(), perm.view().patomic));
                opens_invariants_none();
                return self.ato.swap(v, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            pub fn into_inner(self, #[proof] perm: PermissionU64) -> u64 {
                requires([equal(self.id(), perm.view().patomic)]);
                ensures(|ret: u64| equal(perm.view().value, ret));
                opens_invariants_none();
                return self.ato.into_inner();
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_add_wrapping(&self,
                                      #[proof] perm: &mut PermissionU64,
                                      n: u64) -> u64 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             spec_cast_integer::<u64, int>(perm.view().value)
                                 ==
                                 wrapping_add_u64(spec_cast_integer(old(perm).view().value),
                                                  spec_cast_integer(n))]);
                opens_invariants_none();
                return self.ato.fetch_add(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_sub_wrapping(&self,
                                      #[proof] perm: &mut PermissionU64,
                                      n: u64) -> u64 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             spec_cast_integer::<u64, int>(perm.view().value)
                                 ==
                                 wrapping_sub_u64(spec_cast_integer::<u64,
                                                                      int>(old(perm).view().value),
                                                  spec_cast_integer(n))]);
                opens_invariants_none();
                return self.ato.fetch_sub(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(atomic)]
            pub fn fetch_add(&self, #[proof] perm: &mut PermissionU64, n: u64)
             -> u64 {
                requires([equal(self.id(), old(perm).view().patomic),
                          spec_literal_int("0") <=
                              old(perm).view().value.spec_add(n),
                          old(perm).view().value.spec_add(n) <=
                              spec_literal_int("18446744073709551615")]);
                ensures(|ret: u64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 old(perm).view().value + n]);
                opens_invariants_none();
                self.fetch_add_wrapping(&mut *perm, n)
            }
            #[inline(always)]
            #[verifier(atomic)]
            pub fn fetch_sub(&self, #[proof] perm: &mut PermissionU64, n: u64)
             -> u64 {
                requires([equal(self.id(), old(perm).view().patomic),
                          spec_literal_int("0") <=
                              old(perm).view().value.spec_sub(n),
                          old(perm).view().value.spec_sub(n) <=
                              spec_literal_int("18446744073709551615")]);
                ensures(|ret: u64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 old(perm).view().value - n]);
                opens_invariants_none();
                self.fetch_sub_wrapping(&mut *perm, n)
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_and(&self, #[proof] perm: &mut PermissionU64, n: u64)
             -> u64 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value & n)]);
                opens_invariants_none();
                return self.ato.fetch_and(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_or(&self, #[proof] perm: &mut PermissionU64, n: u64)
             -> u64 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value | n)]);
                opens_invariants_none();
                return self.ato.fetch_or(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_xor(&self, #[proof] perm: &mut PermissionU64, n: u64)
             -> u64 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value ^ n)]);
                opens_invariants_none();
                return self.ato.fetch_or(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_nand(&self, #[proof] perm: &mut PermissionU64,
                              n: u64) -> u64 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 !(old(perm).view().value & n)]);
                opens_invariants_none();
                return self.ato.fetch_nand(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_max(&self, #[proof] perm: &mut PermissionU64, n: u64)
             -> u64 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (if old(perm).view().value > n {
                                      old(perm).view().value
                                  } else { n })]);
                opens_invariants_none();
                return self.ato.fetch_max(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_min(&self, #[proof] perm: &mut PermissionU64, n: u64)
             -> u64 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: u64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (if old(perm).view().value < n {
                                      old(perm).view().value
                                  } else { n })]);
                opens_invariants_none();
                return self.ato.fetch_min(n, Ordering::SeqCst);
            }
        }
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn wrapping_add_i8(a: int, b: int) -> int {
            if ((a).spec_add(b)).spec_gt(spec_literal_int("127")) {
                ((a).spec_add(b)).spec_sub((((spec_literal_int("127")).spec_sub((spec_literal_int("128")).spec_neg())).spec_add(::builtin::spec_literal_nat("1"))))
            } else if ((a).spec_add(b)).spec_lt((spec_literal_int("128")).spec_neg())
             {
                ((a).spec_add(b)).spec_add((((spec_literal_int("127")).spec_sub((spec_literal_int("128")).spec_neg())).spec_add(::builtin::spec_literal_nat("1"))))
            } else { (a).spec_add(b) }
        }
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn wrapping_sub_i8(a: int, b: int) -> int {
            if ((a).spec_sub(b)).spec_gt(spec_literal_int("127")) {
                ((a).spec_sub(b)).spec_sub((((spec_literal_int("127")).spec_sub((spec_literal_int("128")).spec_neg())).spec_add(::builtin::spec_literal_nat("1"))))
            } else if ((a).spec_sub(b)).spec_lt((spec_literal_int("128")).spec_neg())
             {
                ((a).spec_sub(b)).spec_add((((spec_literal_int("127")).spec_sub((spec_literal_int("128")).spec_neg())).spec_add(::builtin::spec_literal_nat("1"))))
            } else { (a).spec_sub(b) }
        }
        #[verifier(external_body)]
        pub struct PAtomicI8 {
            ato: AtomicI8,
        }
        #[proof]
        #[verifier(external_body)]
        pub struct PermissionI8 {
            no_copy: NoCopy,
        }
        #[spec]
        pub struct PermissionDataI8 {
            #[spec]
            pub patomic: int,
            #[spec]
            pub value: i8,
        }
        impl PermissionI8 {
            #[spec]
            #[verifier(external_body)]
            pub fn view(self) -> PermissionDataI8 {
                ::core::panicking::panic("not implemented");
            }
            #[spec]
            #[verifier(publish)]
            pub fn is_for(&self, patomic: PAtomicI8) -> bool {
                self.view().patomic == patomic.id()
            }
            #[spec]
            #[verifier(publish)]
            pub fn points_to(&self, v: i8) -> bool { self.view().value == v }
        }
        impl PAtomicI8 {
            #[spec]
            #[verifier(external_body)]
            pub fn id(&self) -> int {
                ::core::panicking::panic("not implemented")
            }
            #[inline(always)]
            #[verifier(external_body)]
            pub fn new(i: i8) -> (PAtomicI8, Proof<PermissionI8>) {
                ensures(|res: (PAtomicI8, Proof<PermissionI8>)|
                            equal(res.1.0.view(),
                                  PermissionDataI8{patomic: res.0.id(),
                                                   value: i,}));
                let p = PAtomicI8{ato: <AtomicI8>::new(i),};
                let Proof(t) = exec_proof_from_false();
                (p, Proof(t))
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn load(&self, #[proof] perm: &PermissionI8) -> i8 {
                requires([equal(self.id(), perm.view().patomic)]);
                ensures(|ret: i8| equal(perm.view().value, ret));
                opens_invariants_none();
                return self.ato.load(Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn store(&self, #[proof] perm: &mut PermissionI8, v: i8) {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(equal(perm.view().value, v) &&
                            equal(self.id(), perm.view().patomic));
                opens_invariants_none();
                self.ato.store(v, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn compare_exchange(&self, #[proof] perm: &mut PermissionI8,
                                    current: i8, new: i8) -> Result<i8, i8> {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: Result<i8, i8>|
                            equal(self.id(), perm.view().patomic) &&
                                match ret {
                                    Result::Ok(r) =>
                                    current == old(perm).view().value &&
                                        equal(perm.view().value, new) &&
                                        equal(r, old(perm).view().value),
                                    Result::Err(r) =>
                                    current != old(perm).view().value &&
                                        equal(perm.view().value,
                                              old(perm).view().value) &&
                                        equal(r, old(perm).view().value),
                                });
                opens_invariants_none();
                match self.ato.compare_exchange(current, new,
                                                Ordering::SeqCst,
                                                Ordering::SeqCst) {
                    Ok(x) => Result::Ok(x),
                    Err(x) => Result::Err(x),
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn compare_exchange_weak(&self,
                                         #[proof] perm: &mut PermissionI8,
                                         current: i8, new: i8)
             -> Result<i8, i8> {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: Result<i8, i8>|
                            equal(self.id(), perm.view().patomic) &&
                                match ret {
                                    Result::Ok(r) =>
                                    current == old(perm).view().value &&
                                        equal(perm.view().value, new) &&
                                        equal(r, old(perm).view().value),
                                    Result::Err(r) =>
                                    equal(perm.view().value,
                                          old(perm).view().value) &&
                                        equal(r, old(perm).view().value),
                                });
                opens_invariants_none();
                match self.ato.compare_exchange_weak(current, new,
                                                     Ordering::SeqCst,
                                                     Ordering::SeqCst) {
                    Ok(x) => Result::Ok(x),
                    Err(x) => Result::Err(x),
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn swap(&self, #[proof] perm: &mut PermissionI8, v: i8)
             -> i8 {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: i8|
                            equal(perm.view().value, v) &&
                                equal(old(perm).view().value, ret) &&
                                equal(self.id(), perm.view().patomic));
                opens_invariants_none();
                return self.ato.swap(v, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            pub fn into_inner(self, #[proof] perm: PermissionI8) -> i8 {
                requires([equal(self.id(), perm.view().patomic)]);
                ensures(|ret: i8| equal(perm.view().value, ret));
                opens_invariants_none();
                return self.ato.into_inner();
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_add_wrapping(&self, #[proof] perm: &mut PermissionI8,
                                      n: i8) -> i8 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             spec_cast_integer::<i8, int>(perm.view().value)
                                 ==
                                 wrapping_add_i8(spec_cast_integer(old(perm).view().value),
                                                 spec_cast_integer(n))]);
                opens_invariants_none();
                return self.ato.fetch_add(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_sub_wrapping(&self, #[proof] perm: &mut PermissionI8,
                                      n: i8) -> i8 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             spec_cast_integer::<i8, int>(perm.view().value)
                                 ==
                                 wrapping_sub_i8(spec_cast_integer::<i8,
                                                                     int>(old(perm).view().value),
                                                 spec_cast_integer(n))]);
                opens_invariants_none();
                return self.ato.fetch_sub(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(atomic)]
            pub fn fetch_add(&self, #[proof] perm: &mut PermissionI8, n: i8)
             -> i8 {
                requires([equal(self.id(), old(perm).view().patomic),
                          -spec_literal_int("128") <=
                              old(perm).view().value.spec_add(n),
                          old(perm).view().value.spec_add(n) <=
                              spec_literal_int("127")]);
                ensures(|ret: i8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 old(perm).view().value + n]);
                opens_invariants_none();
                self.fetch_add_wrapping(&mut *perm, n)
            }
            #[inline(always)]
            #[verifier(atomic)]
            pub fn fetch_sub(&self, #[proof] perm: &mut PermissionI8, n: i8)
             -> i8 {
                requires([equal(self.id(), old(perm).view().patomic),
                          -spec_literal_int("128") <=
                              old(perm).view().value.spec_sub(n),
                          old(perm).view().value.spec_sub(n) <=
                              spec_literal_int("127")]);
                ensures(|ret: i8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 old(perm).view().value - n]);
                opens_invariants_none();
                self.fetch_sub_wrapping(&mut *perm, n)
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_and(&self, #[proof] perm: &mut PermissionI8, n: i8)
             -> i8 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value & n)]);
                opens_invariants_none();
                return self.ato.fetch_and(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_or(&self, #[proof] perm: &mut PermissionI8, n: i8)
             -> i8 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value | n)]);
                opens_invariants_none();
                return self.ato.fetch_or(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_xor(&self, #[proof] perm: &mut PermissionI8, n: i8)
             -> i8 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value ^ n)]);
                opens_invariants_none();
                return self.ato.fetch_or(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_nand(&self, #[proof] perm: &mut PermissionI8, n: i8)
             -> i8 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 !(old(perm).view().value & n)]);
                opens_invariants_none();
                return self.ato.fetch_nand(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_max(&self, #[proof] perm: &mut PermissionI8, n: i8)
             -> i8 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (if old(perm).view().value > n {
                                      old(perm).view().value
                                  } else { n })]);
                opens_invariants_none();
                return self.ato.fetch_max(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_min(&self, #[proof] perm: &mut PermissionI8, n: i8)
             -> i8 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i8|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (if old(perm).view().value < n {
                                      old(perm).view().value
                                  } else { n })]);
                opens_invariants_none();
                return self.ato.fetch_min(n, Ordering::SeqCst);
            }
        }
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn wrapping_add_i16(a: int, b: int) -> int {
            if ((a).spec_add(b)).spec_gt(spec_literal_int("32767")) {
                ((a).spec_add(b)).spec_sub((((spec_literal_int("32767")).spec_sub((spec_literal_int("32768")).spec_neg())).spec_add(::builtin::spec_literal_nat("1"))))
            } else if ((a).spec_add(b)).spec_lt((spec_literal_int("32768")).spec_neg())
             {
                ((a).spec_add(b)).spec_add((((spec_literal_int("32767")).spec_sub((spec_literal_int("32768")).spec_neg())).spec_add(::builtin::spec_literal_nat("1"))))
            } else { (a).spec_add(b) }
        }
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn wrapping_sub_i16(a: int, b: int) -> int {
            if ((a).spec_sub(b)).spec_gt(spec_literal_int("32767")) {
                ((a).spec_sub(b)).spec_sub((((spec_literal_int("32767")).spec_sub((spec_literal_int("32768")).spec_neg())).spec_add(::builtin::spec_literal_nat("1"))))
            } else if ((a).spec_sub(b)).spec_lt((spec_literal_int("32768")).spec_neg())
             {
                ((a).spec_sub(b)).spec_add((((spec_literal_int("32767")).spec_sub((spec_literal_int("32768")).spec_neg())).spec_add(::builtin::spec_literal_nat("1"))))
            } else { (a).spec_sub(b) }
        }
        #[verifier(external_body)]
        pub struct PAtomicI16 {
            ato: AtomicI16,
        }
        #[proof]
        #[verifier(external_body)]
        pub struct PermissionI16 {
            no_copy: NoCopy,
        }
        #[spec]
        pub struct PermissionDataI16 {
            #[spec]
            pub patomic: int,
            #[spec]
            pub value: i16,
        }
        impl PermissionI16 {
            #[spec]
            #[verifier(external_body)]
            pub fn view(self) -> PermissionDataI16 {
                ::core::panicking::panic("not implemented");
            }
            #[spec]
            #[verifier(publish)]
            pub fn is_for(&self, patomic: PAtomicI16) -> bool {
                self.view().patomic == patomic.id()
            }
            #[spec]
            #[verifier(publish)]
            pub fn points_to(&self, v: i16) -> bool { self.view().value == v }
        }
        impl PAtomicI16 {
            #[spec]
            #[verifier(external_body)]
            pub fn id(&self) -> int {
                ::core::panicking::panic("not implemented")
            }
            #[inline(always)]
            #[verifier(external_body)]
            pub fn new(i: i16) -> (PAtomicI16, Proof<PermissionI16>) {
                ensures(|res: (PAtomicI16, Proof<PermissionI16>)|
                            equal(res.1.0.view(),
                                  PermissionDataI16{patomic: res.0.id(),
                                                    value: i,}));
                let p = PAtomicI16{ato: <AtomicI16>::new(i),};
                let Proof(t) = exec_proof_from_false();
                (p, Proof(t))
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn load(&self, #[proof] perm: &PermissionI16) -> i16 {
                requires([equal(self.id(), perm.view().patomic)]);
                ensures(|ret: i16| equal(perm.view().value, ret));
                opens_invariants_none();
                return self.ato.load(Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn store(&self, #[proof] perm: &mut PermissionI16, v: i16) {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(equal(perm.view().value, v) &&
                            equal(self.id(), perm.view().patomic));
                opens_invariants_none();
                self.ato.store(v, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn compare_exchange(&self, #[proof] perm: &mut PermissionI16,
                                    current: i16, new: i16)
             -> Result<i16, i16> {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: Result<i16, i16>|
                            equal(self.id(), perm.view().patomic) &&
                                match ret {
                                    Result::Ok(r) =>
                                    current == old(perm).view().value &&
                                        equal(perm.view().value, new) &&
                                        equal(r, old(perm).view().value),
                                    Result::Err(r) =>
                                    current != old(perm).view().value &&
                                        equal(perm.view().value,
                                              old(perm).view().value) &&
                                        equal(r, old(perm).view().value),
                                });
                opens_invariants_none();
                match self.ato.compare_exchange(current, new,
                                                Ordering::SeqCst,
                                                Ordering::SeqCst) {
                    Ok(x) => Result::Ok(x),
                    Err(x) => Result::Err(x),
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn compare_exchange_weak(&self,
                                         #[proof] perm: &mut PermissionI16,
                                         current: i16, new: i16)
             -> Result<i16, i16> {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: Result<i16, i16>|
                            equal(self.id(), perm.view().patomic) &&
                                match ret {
                                    Result::Ok(r) =>
                                    current == old(perm).view().value &&
                                        equal(perm.view().value, new) &&
                                        equal(r, old(perm).view().value),
                                    Result::Err(r) =>
                                    equal(perm.view().value,
                                          old(perm).view().value) &&
                                        equal(r, old(perm).view().value),
                                });
                opens_invariants_none();
                match self.ato.compare_exchange_weak(current, new,
                                                     Ordering::SeqCst,
                                                     Ordering::SeqCst) {
                    Ok(x) => Result::Ok(x),
                    Err(x) => Result::Err(x),
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn swap(&self, #[proof] perm: &mut PermissionI16, v: i16)
             -> i16 {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: i16|
                            equal(perm.view().value, v) &&
                                equal(old(perm).view().value, ret) &&
                                equal(self.id(), perm.view().patomic));
                opens_invariants_none();
                return self.ato.swap(v, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            pub fn into_inner(self, #[proof] perm: PermissionI16) -> i16 {
                requires([equal(self.id(), perm.view().patomic)]);
                ensures(|ret: i16| equal(perm.view().value, ret));
                opens_invariants_none();
                return self.ato.into_inner();
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_add_wrapping(&self,
                                      #[proof] perm: &mut PermissionI16,
                                      n: i16) -> i16 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             spec_cast_integer::<i16, int>(perm.view().value)
                                 ==
                                 wrapping_add_i16(spec_cast_integer(old(perm).view().value),
                                                  spec_cast_integer(n))]);
                opens_invariants_none();
                return self.ato.fetch_add(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_sub_wrapping(&self,
                                      #[proof] perm: &mut PermissionI16,
                                      n: i16) -> i16 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             spec_cast_integer::<i16, int>(perm.view().value)
                                 ==
                                 wrapping_sub_i16(spec_cast_integer::<i16,
                                                                      int>(old(perm).view().value),
                                                  spec_cast_integer(n))]);
                opens_invariants_none();
                return self.ato.fetch_sub(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(atomic)]
            pub fn fetch_add(&self, #[proof] perm: &mut PermissionI16, n: i16)
             -> i16 {
                requires([equal(self.id(), old(perm).view().patomic),
                          -spec_literal_int("32768") <=
                              old(perm).view().value.spec_add(n),
                          old(perm).view().value.spec_add(n) <=
                              spec_literal_int("32767")]);
                ensures(|ret: i16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 old(perm).view().value + n]);
                opens_invariants_none();
                self.fetch_add_wrapping(&mut *perm, n)
            }
            #[inline(always)]
            #[verifier(atomic)]
            pub fn fetch_sub(&self, #[proof] perm: &mut PermissionI16, n: i16)
             -> i16 {
                requires([equal(self.id(), old(perm).view().patomic),
                          -spec_literal_int("32768") <=
                              old(perm).view().value.spec_sub(n),
                          old(perm).view().value.spec_sub(n) <=
                              spec_literal_int("32767")]);
                ensures(|ret: i16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 old(perm).view().value - n]);
                opens_invariants_none();
                self.fetch_sub_wrapping(&mut *perm, n)
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_and(&self, #[proof] perm: &mut PermissionI16, n: i16)
             -> i16 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value & n)]);
                opens_invariants_none();
                return self.ato.fetch_and(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_or(&self, #[proof] perm: &mut PermissionI16, n: i16)
             -> i16 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value | n)]);
                opens_invariants_none();
                return self.ato.fetch_or(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_xor(&self, #[proof] perm: &mut PermissionI16, n: i16)
             -> i16 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value ^ n)]);
                opens_invariants_none();
                return self.ato.fetch_or(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_nand(&self, #[proof] perm: &mut PermissionI16,
                              n: i16) -> i16 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 !(old(perm).view().value & n)]);
                opens_invariants_none();
                return self.ato.fetch_nand(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_max(&self, #[proof] perm: &mut PermissionI16, n: i16)
             -> i16 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (if old(perm).view().value > n {
                                      old(perm).view().value
                                  } else { n })]);
                opens_invariants_none();
                return self.ato.fetch_max(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_min(&self, #[proof] perm: &mut PermissionI16, n: i16)
             -> i16 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i16|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (if old(perm).view().value < n {
                                      old(perm).view().value
                                  } else { n })]);
                opens_invariants_none();
                return self.ato.fetch_min(n, Ordering::SeqCst);
            }
        }
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn wrapping_add_i32(a: int, b: int) -> int {
            if ((a).spec_add(b)).spec_gt(spec_literal_int("2147483647")) {
                ((a).spec_add(b)).spec_sub((((spec_literal_int("2147483647")).spec_sub((spec_literal_int("2147483648")).spec_neg())).spec_add(::builtin::spec_literal_nat("1"))))
            } else if ((a).spec_add(b)).spec_lt((spec_literal_int("2147483648")).spec_neg())
             {
                ((a).spec_add(b)).spec_add((((spec_literal_int("2147483647")).spec_sub((spec_literal_int("2147483648")).spec_neg())).spec_add(::builtin::spec_literal_nat("1"))))
            } else { (a).spec_add(b) }
        }
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn wrapping_sub_i32(a: int, b: int) -> int {
            if ((a).spec_sub(b)).spec_gt(spec_literal_int("2147483647")) {
                ((a).spec_sub(b)).spec_sub((((spec_literal_int("2147483647")).spec_sub((spec_literal_int("2147483648")).spec_neg())).spec_add(::builtin::spec_literal_nat("1"))))
            } else if ((a).spec_sub(b)).spec_lt((spec_literal_int("2147483648")).spec_neg())
             {
                ((a).spec_sub(b)).spec_add((((spec_literal_int("2147483647")).spec_sub((spec_literal_int("2147483648")).spec_neg())).spec_add(::builtin::spec_literal_nat("1"))))
            } else { (a).spec_sub(b) }
        }
        #[verifier(external_body)]
        pub struct PAtomicI32 {
            ato: AtomicI32,
        }
        #[proof]
        #[verifier(external_body)]
        pub struct PermissionI32 {
            no_copy: NoCopy,
        }
        #[spec]
        pub struct PermissionDataI32 {
            #[spec]
            pub patomic: int,
            #[spec]
            pub value: i32,
        }
        impl PermissionI32 {
            #[spec]
            #[verifier(external_body)]
            pub fn view(self) -> PermissionDataI32 {
                ::core::panicking::panic("not implemented");
            }
            #[spec]
            #[verifier(publish)]
            pub fn is_for(&self, patomic: PAtomicI32) -> bool {
                self.view().patomic == patomic.id()
            }
            #[spec]
            #[verifier(publish)]
            pub fn points_to(&self, v: i32) -> bool { self.view().value == v }
        }
        impl PAtomicI32 {
            #[spec]
            #[verifier(external_body)]
            pub fn id(&self) -> int {
                ::core::panicking::panic("not implemented")
            }
            #[inline(always)]
            #[verifier(external_body)]
            pub fn new(i: i32) -> (PAtomicI32, Proof<PermissionI32>) {
                ensures(|res: (PAtomicI32, Proof<PermissionI32>)|
                            equal(res.1.0.view(),
                                  PermissionDataI32{patomic: res.0.id(),
                                                    value: i,}));
                let p = PAtomicI32{ato: <AtomicI32>::new(i),};
                let Proof(t) = exec_proof_from_false();
                (p, Proof(t))
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn load(&self, #[proof] perm: &PermissionI32) -> i32 {
                requires([equal(self.id(), perm.view().patomic)]);
                ensures(|ret: i32| equal(perm.view().value, ret));
                opens_invariants_none();
                return self.ato.load(Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn store(&self, #[proof] perm: &mut PermissionI32, v: i32) {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(equal(perm.view().value, v) &&
                            equal(self.id(), perm.view().patomic));
                opens_invariants_none();
                self.ato.store(v, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn compare_exchange(&self, #[proof] perm: &mut PermissionI32,
                                    current: i32, new: i32)
             -> Result<i32, i32> {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: Result<i32, i32>|
                            equal(self.id(), perm.view().patomic) &&
                                match ret {
                                    Result::Ok(r) =>
                                    current == old(perm).view().value &&
                                        equal(perm.view().value, new) &&
                                        equal(r, old(perm).view().value),
                                    Result::Err(r) =>
                                    current != old(perm).view().value &&
                                        equal(perm.view().value,
                                              old(perm).view().value) &&
                                        equal(r, old(perm).view().value),
                                });
                opens_invariants_none();
                match self.ato.compare_exchange(current, new,
                                                Ordering::SeqCst,
                                                Ordering::SeqCst) {
                    Ok(x) => Result::Ok(x),
                    Err(x) => Result::Err(x),
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn compare_exchange_weak(&self,
                                         #[proof] perm: &mut PermissionI32,
                                         current: i32, new: i32)
             -> Result<i32, i32> {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: Result<i32, i32>|
                            equal(self.id(), perm.view().patomic) &&
                                match ret {
                                    Result::Ok(r) =>
                                    current == old(perm).view().value &&
                                        equal(perm.view().value, new) &&
                                        equal(r, old(perm).view().value),
                                    Result::Err(r) =>
                                    equal(perm.view().value,
                                          old(perm).view().value) &&
                                        equal(r, old(perm).view().value),
                                });
                opens_invariants_none();
                match self.ato.compare_exchange_weak(current, new,
                                                     Ordering::SeqCst,
                                                     Ordering::SeqCst) {
                    Ok(x) => Result::Ok(x),
                    Err(x) => Result::Err(x),
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn swap(&self, #[proof] perm: &mut PermissionI32, v: i32)
             -> i32 {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: i32|
                            equal(perm.view().value, v) &&
                                equal(old(perm).view().value, ret) &&
                                equal(self.id(), perm.view().patomic));
                opens_invariants_none();
                return self.ato.swap(v, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            pub fn into_inner(self, #[proof] perm: PermissionI32) -> i32 {
                requires([equal(self.id(), perm.view().patomic)]);
                ensures(|ret: i32| equal(perm.view().value, ret));
                opens_invariants_none();
                return self.ato.into_inner();
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_add_wrapping(&self,
                                      #[proof] perm: &mut PermissionI32,
                                      n: i32) -> i32 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             spec_cast_integer::<i32, int>(perm.view().value)
                                 ==
                                 wrapping_add_i32(spec_cast_integer(old(perm).view().value),
                                                  spec_cast_integer(n))]);
                opens_invariants_none();
                return self.ato.fetch_add(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_sub_wrapping(&self,
                                      #[proof] perm: &mut PermissionI32,
                                      n: i32) -> i32 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             spec_cast_integer::<i32, int>(perm.view().value)
                                 ==
                                 wrapping_sub_i32(spec_cast_integer::<i32,
                                                                      int>(old(perm).view().value),
                                                  spec_cast_integer(n))]);
                opens_invariants_none();
                return self.ato.fetch_sub(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(atomic)]
            pub fn fetch_add(&self, #[proof] perm: &mut PermissionI32, n: i32)
             -> i32 {
                requires([equal(self.id(), old(perm).view().patomic),
                          -spec_literal_int("2147483648") <=
                              old(perm).view().value.spec_add(n),
                          old(perm).view().value.spec_add(n) <=
                              spec_literal_int("2147483647")]);
                ensures(|ret: i32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 old(perm).view().value + n]);
                opens_invariants_none();
                self.fetch_add_wrapping(&mut *perm, n)
            }
            #[inline(always)]
            #[verifier(atomic)]
            pub fn fetch_sub(&self, #[proof] perm: &mut PermissionI32, n: i32)
             -> i32 {
                requires([equal(self.id(), old(perm).view().patomic),
                          -spec_literal_int("2147483648") <=
                              old(perm).view().value.spec_sub(n),
                          old(perm).view().value.spec_sub(n) <=
                              spec_literal_int("2147483647")]);
                ensures(|ret: i32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 old(perm).view().value - n]);
                opens_invariants_none();
                self.fetch_sub_wrapping(&mut *perm, n)
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_and(&self, #[proof] perm: &mut PermissionI32, n: i32)
             -> i32 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value & n)]);
                opens_invariants_none();
                return self.ato.fetch_and(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_or(&self, #[proof] perm: &mut PermissionI32, n: i32)
             -> i32 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value | n)]);
                opens_invariants_none();
                return self.ato.fetch_or(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_xor(&self, #[proof] perm: &mut PermissionI32, n: i32)
             -> i32 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value ^ n)]);
                opens_invariants_none();
                return self.ato.fetch_or(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_nand(&self, #[proof] perm: &mut PermissionI32,
                              n: i32) -> i32 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 !(old(perm).view().value & n)]);
                opens_invariants_none();
                return self.ato.fetch_nand(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_max(&self, #[proof] perm: &mut PermissionI32, n: i32)
             -> i32 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (if old(perm).view().value > n {
                                      old(perm).view().value
                                  } else { n })]);
                opens_invariants_none();
                return self.ato.fetch_max(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_min(&self, #[proof] perm: &mut PermissionI32, n: i32)
             -> i32 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i32|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (if old(perm).view().value < n {
                                      old(perm).view().value
                                  } else { n })]);
                opens_invariants_none();
                return self.ato.fetch_min(n, Ordering::SeqCst);
            }
        }
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn wrapping_add_i64(a: int, b: int) -> int {
            if ((a).spec_add(b)).spec_gt(spec_literal_int("9223372036854775807"))
               {
                ((a).spec_add(b)).spec_sub((((spec_literal_int("9223372036854775807")).spec_sub((spec_literal_int("9223372036854775808")).spec_neg())).spec_add(::builtin::spec_literal_nat("1"))))
            } else if ((a).spec_add(b)).spec_lt((spec_literal_int("9223372036854775808")).spec_neg())
             {
                ((a).spec_add(b)).spec_add((((spec_literal_int("9223372036854775807")).spec_sub((spec_literal_int("9223372036854775808")).spec_neg())).spec_add(::builtin::spec_literal_nat("1"))))
            } else { (a).spec_add(b) }
        }
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn wrapping_sub_i64(a: int, b: int) -> int {
            if ((a).spec_sub(b)).spec_gt(spec_literal_int("9223372036854775807"))
               {
                ((a).spec_sub(b)).spec_sub((((spec_literal_int("9223372036854775807")).spec_sub((spec_literal_int("9223372036854775808")).spec_neg())).spec_add(::builtin::spec_literal_nat("1"))))
            } else if ((a).spec_sub(b)).spec_lt((spec_literal_int("9223372036854775808")).spec_neg())
             {
                ((a).spec_sub(b)).spec_add((((spec_literal_int("9223372036854775807")).spec_sub((spec_literal_int("9223372036854775808")).spec_neg())).spec_add(::builtin::spec_literal_nat("1"))))
            } else { (a).spec_sub(b) }
        }
        #[verifier(external_body)]
        pub struct PAtomicI64 {
            ato: AtomicI64,
        }
        #[proof]
        #[verifier(external_body)]
        pub struct PermissionI64 {
            no_copy: NoCopy,
        }
        #[spec]
        pub struct PermissionDataI64 {
            #[spec]
            pub patomic: int,
            #[spec]
            pub value: i64,
        }
        impl PermissionI64 {
            #[spec]
            #[verifier(external_body)]
            pub fn view(self) -> PermissionDataI64 {
                ::core::panicking::panic("not implemented");
            }
            #[spec]
            #[verifier(publish)]
            pub fn is_for(&self, patomic: PAtomicI64) -> bool {
                self.view().patomic == patomic.id()
            }
            #[spec]
            #[verifier(publish)]
            pub fn points_to(&self, v: i64) -> bool { self.view().value == v }
        }
        impl PAtomicI64 {
            #[spec]
            #[verifier(external_body)]
            pub fn id(&self) -> int {
                ::core::panicking::panic("not implemented")
            }
            #[inline(always)]
            #[verifier(external_body)]
            pub fn new(i: i64) -> (PAtomicI64, Proof<PermissionI64>) {
                ensures(|res: (PAtomicI64, Proof<PermissionI64>)|
                            equal(res.1.0.view(),
                                  PermissionDataI64{patomic: res.0.id(),
                                                    value: i,}));
                let p = PAtomicI64{ato: <AtomicI64>::new(i),};
                let Proof(t) = exec_proof_from_false();
                (p, Proof(t))
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn load(&self, #[proof] perm: &PermissionI64) -> i64 {
                requires([equal(self.id(), perm.view().patomic)]);
                ensures(|ret: i64| equal(perm.view().value, ret));
                opens_invariants_none();
                return self.ato.load(Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn store(&self, #[proof] perm: &mut PermissionI64, v: i64) {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(equal(perm.view().value, v) &&
                            equal(self.id(), perm.view().patomic));
                opens_invariants_none();
                self.ato.store(v, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn compare_exchange(&self, #[proof] perm: &mut PermissionI64,
                                    current: i64, new: i64)
             -> Result<i64, i64> {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: Result<i64, i64>|
                            equal(self.id(), perm.view().patomic) &&
                                match ret {
                                    Result::Ok(r) =>
                                    current == old(perm).view().value &&
                                        equal(perm.view().value, new) &&
                                        equal(r, old(perm).view().value),
                                    Result::Err(r) =>
                                    current != old(perm).view().value &&
                                        equal(perm.view().value,
                                              old(perm).view().value) &&
                                        equal(r, old(perm).view().value),
                                });
                opens_invariants_none();
                match self.ato.compare_exchange(current, new,
                                                Ordering::SeqCst,
                                                Ordering::SeqCst) {
                    Ok(x) => Result::Ok(x),
                    Err(x) => Result::Err(x),
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn compare_exchange_weak(&self,
                                         #[proof] perm: &mut PermissionI64,
                                         current: i64, new: i64)
             -> Result<i64, i64> {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: Result<i64, i64>|
                            equal(self.id(), perm.view().patomic) &&
                                match ret {
                                    Result::Ok(r) =>
                                    current == old(perm).view().value &&
                                        equal(perm.view().value, new) &&
                                        equal(r, old(perm).view().value),
                                    Result::Err(r) =>
                                    equal(perm.view().value,
                                          old(perm).view().value) &&
                                        equal(r, old(perm).view().value),
                                });
                opens_invariants_none();
                match self.ato.compare_exchange_weak(current, new,
                                                     Ordering::SeqCst,
                                                     Ordering::SeqCst) {
                    Ok(x) => Result::Ok(x),
                    Err(x) => Result::Err(x),
                }
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn swap(&self, #[proof] perm: &mut PermissionI64, v: i64)
             -> i64 {
                requires([equal(self.id(), old(perm).view().patomic)]);
                ensures(|ret: i64|
                            equal(perm.view().value, v) &&
                                equal(old(perm).view().value, ret) &&
                                equal(self.id(), perm.view().patomic));
                opens_invariants_none();
                return self.ato.swap(v, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            pub fn into_inner(self, #[proof] perm: PermissionI64) -> i64 {
                requires([equal(self.id(), perm.view().patomic)]);
                ensures(|ret: i64| equal(perm.view().value, ret));
                opens_invariants_none();
                return self.ato.into_inner();
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_add_wrapping(&self,
                                      #[proof] perm: &mut PermissionI64,
                                      n: i64) -> i64 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             spec_cast_integer::<i64, int>(perm.view().value)
                                 ==
                                 wrapping_add_i64(spec_cast_integer(old(perm).view().value),
                                                  spec_cast_integer(n))]);
                opens_invariants_none();
                return self.ato.fetch_add(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_sub_wrapping(&self,
                                      #[proof] perm: &mut PermissionI64,
                                      n: i64) -> i64 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             spec_cast_integer::<i64, int>(perm.view().value)
                                 ==
                                 wrapping_sub_i64(spec_cast_integer::<i64,
                                                                      int>(old(perm).view().value),
                                                  spec_cast_integer(n))]);
                opens_invariants_none();
                return self.ato.fetch_sub(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(atomic)]
            pub fn fetch_add(&self, #[proof] perm: &mut PermissionI64, n: i64)
             -> i64 {
                requires([equal(self.id(), old(perm).view().patomic),
                          -spec_literal_int("9223372036854775808") <=
                              old(perm).view().value.spec_add(n),
                          old(perm).view().value.spec_add(n) <=
                              spec_literal_int("9223372036854775807")]);
                ensures(|ret: i64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 old(perm).view().value + n]);
                opens_invariants_none();
                self.fetch_add_wrapping(&mut *perm, n)
            }
            #[inline(always)]
            #[verifier(atomic)]
            pub fn fetch_sub(&self, #[proof] perm: &mut PermissionI64, n: i64)
             -> i64 {
                requires([equal(self.id(), old(perm).view().patomic),
                          -spec_literal_int("9223372036854775808") <=
                              old(perm).view().value.spec_sub(n),
                          old(perm).view().value.spec_sub(n) <=
                              spec_literal_int("9223372036854775807")]);
                ensures(|ret: i64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 old(perm).view().value - n]);
                opens_invariants_none();
                self.fetch_sub_wrapping(&mut *perm, n)
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_and(&self, #[proof] perm: &mut PermissionI64, n: i64)
             -> i64 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value & n)]);
                opens_invariants_none();
                return self.ato.fetch_and(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_or(&self, #[proof] perm: &mut PermissionI64, n: i64)
             -> i64 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value | n)]);
                opens_invariants_none();
                return self.ato.fetch_or(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_xor(&self, #[proof] perm: &mut PermissionI64, n: i64)
             -> i64 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (old(perm).view().value ^ n)]);
                opens_invariants_none();
                return self.ato.fetch_or(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_nand(&self, #[proof] perm: &mut PermissionI64,
                              n: i64) -> i64 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 !(old(perm).view().value & n)]);
                opens_invariants_none();
                return self.ato.fetch_nand(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_max(&self, #[proof] perm: &mut PermissionI64, n: i64)
             -> i64 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (if old(perm).view().value > n {
                                      old(perm).view().value
                                  } else { n })]);
                opens_invariants_none();
                return self.ato.fetch_max(n, Ordering::SeqCst);
            }
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(atomic)]
            pub fn fetch_min(&self, #[proof] perm: &mut PermissionI64, n: i64)
             -> i64 {
                requires(equal(self.id(), old(perm).view().patomic));
                ensures(|ret: i64|
                            [equal(old(perm).view().value, ret),
                             perm.view().patomic == old(perm).view().patomic,
                             perm.view().value ==
                                 (if old(perm).view().value < n {
                                      old(perm).view().value
                                  } else { n })]);
                opens_invariants_none();
                return self.ato.fetch_min(n, Ordering::SeqCst);
            }
        }
    }
    pub mod atomic_ghost {
        #![allow(unused_imports)]
        use builtin::*;
        use builtin_macros::*;
        use crate::pervasive::invariant::*;
        use crate::pervasive::atomic::*;
        use crate::pervasive::modes::*;
        macro_rules! declare_atomic_type {
            ($at_ident : ident, $patomic_ty : ident, $perm_ty : ty, $value_ty
             : ty) =>
            {
                pub struct $at_ident < #[verifier(maybe_negative)] G >
                {
                    pub patomic : $patomic_ty, #[proof] pub atomic_inv :
                    AtomicInvariant < ($perm_ty, G) >,
                } impl < G > $at_ident < G >
                {
                    #[spec] #[verifier(publish)] pub fn
                    has_inv(& self, f : impl Fn($value_ty, G) -> bool) -> bool
                    {
                        forall(| p | #[trigger] self.atomic_inv.inv(p) ==
                               (self.patomic.id() == p.0.view().patomic &&
                                f(p.0.view().value, p.1)))
                    } #[spec] #[verifier(publish)] pub fn
                    has_inv_fn(& self, f : impl Fn($value_ty) -> G) -> bool
                    { self.has_inv(| v : $value_ty, g : G | equal(g, f(v))) }
                    #[inline(always)] pub fn
                    new(u : $value_ty, #[proof] g : G, #[spec] f : impl
                        Fn($value_ty, G) -> bool) -> Self
                    {
                        requires(f(u, g)) ; ensures(| t : Self | t.has_inv(f))
                        ; let(patomic, Proof(perm)) = $patomic_ty :: new(u) ;
                        #[proof] let pair = (perm, g) ; #[proof] let
                        atomic_inv = AtomicInvariant ::
                        new(pair, | p | patomic.id() == p.0.view().patomic &&
                            f(p.0.view().value, p.1), spec_literal_int("0")) ;
                        $at_ident { patomic, atomic_inv, }
                    }
                }
            }
        }
        pub struct AtomicU64<#[verifier(maybe_negative)] G> {
            pub patomic: PAtomicU64,
            #[proof]
            pub atomic_inv: AtomicInvariant<(PermissionU64, G)>,
        }
        impl <G> AtomicU64<G> {
            #[spec]
            #[verifier(publish)]
            pub fn has_inv(&self, f: impl Fn(u64, G) -> bool) -> bool {
                forall(|p|
                           #[trigger] self.atomic_inv.inv(p) ==
                               (self.patomic.id() == p.0.view().patomic &&
                                    f(p.0.view().value, p.1)))
            }
            #[spec]
            #[verifier(publish)]
            pub fn has_inv_fn(&self, f: impl Fn(u64) -> G) -> bool {
                self.has_inv(|v: u64, g: G| equal(g, f(v)))
            }
            #[inline(always)]
            pub fn new(u: u64, #[proof] g: G,
                       #[spec] f: impl Fn(u64, G) -> bool) -> Self {
                requires(f(u, g));
                ensures(|t: Self| t.has_inv(f));
                let (patomic, Proof(perm)) = PAtomicU64::new(u);
                #[proof]
                let pair = (perm, g);
                #[proof]
                let atomic_inv =
                    AtomicInvariant::new(pair,
                                         |p|
                                             patomic.id() ==
                                                 p.0.view().patomic &&
                                                 f(p.0.view().value, p.1),
                                         spec_literal_int("0"));
                AtomicU64{patomic, atomic_inv,}
            }
        }
        pub struct AtomicU32<#[verifier(maybe_negative)] G> {
            pub patomic: PAtomicU32,
            #[proof]
            pub atomic_inv: AtomicInvariant<(PermissionU32, G)>,
        }
        impl <G> AtomicU32<G> {
            #[spec]
            #[verifier(publish)]
            pub fn has_inv(&self, f: impl Fn(u32, G) -> bool) -> bool {
                forall(|p|
                           #[trigger] self.atomic_inv.inv(p) ==
                               (self.patomic.id() == p.0.view().patomic &&
                                    f(p.0.view().value, p.1)))
            }
            #[spec]
            #[verifier(publish)]
            pub fn has_inv_fn(&self, f: impl Fn(u32) -> G) -> bool {
                self.has_inv(|v: u32, g: G| equal(g, f(v)))
            }
            #[inline(always)]
            pub fn new(u: u32, #[proof] g: G,
                       #[spec] f: impl Fn(u32, G) -> bool) -> Self {
                requires(f(u, g));
                ensures(|t: Self| t.has_inv(f));
                let (patomic, Proof(perm)) = PAtomicU32::new(u);
                #[proof]
                let pair = (perm, g);
                #[proof]
                let atomic_inv =
                    AtomicInvariant::new(pair,
                                         |p|
                                             patomic.id() ==
                                                 p.0.view().patomic &&
                                                 f(p.0.view().value, p.1),
                                         spec_literal_int("0"));
                AtomicU32{patomic, atomic_inv,}
            }
        }
        pub struct AtomicU16<#[verifier(maybe_negative)] G> {
            pub patomic: PAtomicU16,
            #[proof]
            pub atomic_inv: AtomicInvariant<(PermissionU16, G)>,
        }
        impl <G> AtomicU16<G> {
            #[spec]
            #[verifier(publish)]
            pub fn has_inv(&self, f: impl Fn(u16, G) -> bool) -> bool {
                forall(|p|
                           #[trigger] self.atomic_inv.inv(p) ==
                               (self.patomic.id() == p.0.view().patomic &&
                                    f(p.0.view().value, p.1)))
            }
            #[spec]
            #[verifier(publish)]
            pub fn has_inv_fn(&self, f: impl Fn(u16) -> G) -> bool {
                self.has_inv(|v: u16, g: G| equal(g, f(v)))
            }
            #[inline(always)]
            pub fn new(u: u16, #[proof] g: G,
                       #[spec] f: impl Fn(u16, G) -> bool) -> Self {
                requires(f(u, g));
                ensures(|t: Self| t.has_inv(f));
                let (patomic, Proof(perm)) = PAtomicU16::new(u);
                #[proof]
                let pair = (perm, g);
                #[proof]
                let atomic_inv =
                    AtomicInvariant::new(pair,
                                         |p|
                                             patomic.id() ==
                                                 p.0.view().patomic &&
                                                 f(p.0.view().value, p.1),
                                         spec_literal_int("0"));
                AtomicU16{patomic, atomic_inv,}
            }
        }
        pub struct AtomicU8<#[verifier(maybe_negative)] G> {
            pub patomic: PAtomicU8,
            #[proof]
            pub atomic_inv: AtomicInvariant<(PermissionU8, G)>,
        }
        impl <G> AtomicU8<G> {
            #[spec]
            #[verifier(publish)]
            pub fn has_inv(&self, f: impl Fn(u8, G) -> bool) -> bool {
                forall(|p|
                           #[trigger] self.atomic_inv.inv(p) ==
                               (self.patomic.id() == p.0.view().patomic &&
                                    f(p.0.view().value, p.1)))
            }
            #[spec]
            #[verifier(publish)]
            pub fn has_inv_fn(&self, f: impl Fn(u8) -> G) -> bool {
                self.has_inv(|v: u8, g: G| equal(g, f(v)))
            }
            #[inline(always)]
            pub fn new(u: u8, #[proof] g: G,
                       #[spec] f: impl Fn(u8, G) -> bool) -> Self {
                requires(f(u, g));
                ensures(|t: Self| t.has_inv(f));
                let (patomic, Proof(perm)) = PAtomicU8::new(u);
                #[proof]
                let pair = (perm, g);
                #[proof]
                let atomic_inv =
                    AtomicInvariant::new(pair,
                                         |p|
                                             patomic.id() ==
                                                 p.0.view().patomic &&
                                                 f(p.0.view().value, p.1),
                                         spec_literal_int("0"));
                AtomicU8{patomic, atomic_inv,}
            }
        }
        pub struct AtomicI64<#[verifier(maybe_negative)] G> {
            pub patomic: PAtomicI64,
            #[proof]
            pub atomic_inv: AtomicInvariant<(PermissionI64, G)>,
        }
        impl <G> AtomicI64<G> {
            #[spec]
            #[verifier(publish)]
            pub fn has_inv(&self, f: impl Fn(i64, G) -> bool) -> bool {
                forall(|p|
                           #[trigger] self.atomic_inv.inv(p) ==
                               (self.patomic.id() == p.0.view().patomic &&
                                    f(p.0.view().value, p.1)))
            }
            #[spec]
            #[verifier(publish)]
            pub fn has_inv_fn(&self, f: impl Fn(i64) -> G) -> bool {
                self.has_inv(|v: i64, g: G| equal(g, f(v)))
            }
            #[inline(always)]
            pub fn new(u: i64, #[proof] g: G,
                       #[spec] f: impl Fn(i64, G) -> bool) -> Self {
                requires(f(u, g));
                ensures(|t: Self| t.has_inv(f));
                let (patomic, Proof(perm)) = PAtomicI64::new(u);
                #[proof]
                let pair = (perm, g);
                #[proof]
                let atomic_inv =
                    AtomicInvariant::new(pair,
                                         |p|
                                             patomic.id() ==
                                                 p.0.view().patomic &&
                                                 f(p.0.view().value, p.1),
                                         spec_literal_int("0"));
                AtomicI64{patomic, atomic_inv,}
            }
        }
        pub struct AtomicI32<#[verifier(maybe_negative)] G> {
            pub patomic: PAtomicI32,
            #[proof]
            pub atomic_inv: AtomicInvariant<(PermissionI32, G)>,
        }
        impl <G> AtomicI32<G> {
            #[spec]
            #[verifier(publish)]
            pub fn has_inv(&self, f: impl Fn(i32, G) -> bool) -> bool {
                forall(|p|
                           #[trigger] self.atomic_inv.inv(p) ==
                               (self.patomic.id() == p.0.view().patomic &&
                                    f(p.0.view().value, p.1)))
            }
            #[spec]
            #[verifier(publish)]
            pub fn has_inv_fn(&self, f: impl Fn(i32) -> G) -> bool {
                self.has_inv(|v: i32, g: G| equal(g, f(v)))
            }
            #[inline(always)]
            pub fn new(u: i32, #[proof] g: G,
                       #[spec] f: impl Fn(i32, G) -> bool) -> Self {
                requires(f(u, g));
                ensures(|t: Self| t.has_inv(f));
                let (patomic, Proof(perm)) = PAtomicI32::new(u);
                #[proof]
                let pair = (perm, g);
                #[proof]
                let atomic_inv =
                    AtomicInvariant::new(pair,
                                         |p|
                                             patomic.id() ==
                                                 p.0.view().patomic &&
                                                 f(p.0.view().value, p.1),
                                         spec_literal_int("0"));
                AtomicI32{patomic, atomic_inv,}
            }
        }
        pub struct AtomicI16<#[verifier(maybe_negative)] G> {
            pub patomic: PAtomicI16,
            #[proof]
            pub atomic_inv: AtomicInvariant<(PermissionI16, G)>,
        }
        impl <G> AtomicI16<G> {
            #[spec]
            #[verifier(publish)]
            pub fn has_inv(&self, f: impl Fn(i16, G) -> bool) -> bool {
                forall(|p|
                           #[trigger] self.atomic_inv.inv(p) ==
                               (self.patomic.id() == p.0.view().patomic &&
                                    f(p.0.view().value, p.1)))
            }
            #[spec]
            #[verifier(publish)]
            pub fn has_inv_fn(&self, f: impl Fn(i16) -> G) -> bool {
                self.has_inv(|v: i16, g: G| equal(g, f(v)))
            }
            #[inline(always)]
            pub fn new(u: i16, #[proof] g: G,
                       #[spec] f: impl Fn(i16, G) -> bool) -> Self {
                requires(f(u, g));
                ensures(|t: Self| t.has_inv(f));
                let (patomic, Proof(perm)) = PAtomicI16::new(u);
                #[proof]
                let pair = (perm, g);
                #[proof]
                let atomic_inv =
                    AtomicInvariant::new(pair,
                                         |p|
                                             patomic.id() ==
                                                 p.0.view().patomic &&
                                                 f(p.0.view().value, p.1),
                                         spec_literal_int("0"));
                AtomicI16{patomic, atomic_inv,}
            }
        }
        pub struct AtomicI8<#[verifier(maybe_negative)] G> {
            pub patomic: PAtomicI8,
            #[proof]
            pub atomic_inv: AtomicInvariant<(PermissionI8, G)>,
        }
        impl <G> AtomicI8<G> {
            #[spec]
            #[verifier(publish)]
            pub fn has_inv(&self, f: impl Fn(i8, G) -> bool) -> bool {
                forall(|p|
                           #[trigger] self.atomic_inv.inv(p) ==
                               (self.patomic.id() == p.0.view().patomic &&
                                    f(p.0.view().value, p.1)))
            }
            #[spec]
            #[verifier(publish)]
            pub fn has_inv_fn(&self, f: impl Fn(i8) -> G) -> bool {
                self.has_inv(|v: i8, g: G| equal(g, f(v)))
            }
            #[inline(always)]
            pub fn new(u: i8, #[proof] g: G,
                       #[spec] f: impl Fn(i8, G) -> bool) -> Self {
                requires(f(u, g));
                ensures(|t: Self| t.has_inv(f));
                let (patomic, Proof(perm)) = PAtomicI8::new(u);
                #[proof]
                let pair = (perm, g);
                #[proof]
                let atomic_inv =
                    AtomicInvariant::new(pair,
                                         |p|
                                             patomic.id() ==
                                                 p.0.view().patomic &&
                                                 f(p.0.view().value, p.1),
                                         spec_literal_int("0"));
                AtomicI8{patomic, atomic_inv,}
            }
        }
        pub struct AtomicBool<#[verifier(maybe_negative)] G> {
            pub patomic: PAtomicBool,
            #[proof]
            pub atomic_inv: AtomicInvariant<(PermissionBool, G)>,
        }
        impl <G> AtomicBool<G> {
            #[spec]
            #[verifier(publish)]
            pub fn has_inv(&self, f: impl Fn(bool, G) -> bool) -> bool {
                forall(|p|
                           #[trigger] self.atomic_inv.inv(p) ==
                               (self.patomic.id() == p.0.view().patomic &&
                                    f(p.0.view().value, p.1)))
            }
            #[spec]
            #[verifier(publish)]
            pub fn has_inv_fn(&self, f: impl Fn(bool) -> G) -> bool {
                self.has_inv(|v: bool, g: G| equal(g, f(v)))
            }
            #[inline(always)]
            pub fn new(u: bool, #[proof] g: G,
                       #[spec] f: impl Fn(bool, G) -> bool) -> Self {
                requires(f(u, g));
                ensures(|t: Self| t.has_inv(f));
                let (patomic, Proof(perm)) = PAtomicBool::new(u);
                #[proof]
                let pair = (perm, g);
                #[proof]
                let atomic_inv =
                    AtomicInvariant::new(pair,
                                         |p|
                                             patomic.id() ==
                                                 p.0.view().patomic &&
                                                 f(p.0.view().value, p.1),
                                         spec_literal_int("0"));
                AtomicBool{patomic, atomic_inv,}
            }
        }
        /// Performs a given atomic operation on a given atomic
        /// while providing access to its ghost state.
        ///
        /// `atomic_with_ghost!` supports the types
        /// [`AtomicU64`] [`AtomicU32`], [`AtomicU16`], [`AtomicU8`],
        /// [`AtomicI64`], [`AtomicI32`], [`AtomicI16`], [`AtomicI8`], and [`AtomicBool`].
        ///
        /// For each type, it supports all applicable atomic operations among
        /// `load`, `store`, `swap`, `compare_exchange`, `compare_exchange_weak`,
        /// `fetch_add`, `fetch_add_wrapping`, `fetch_sub`, `fetch_sub_wrapping`,
        /// `fetch_or`, `fetch_and`, `fetch_xor`, `fetch_nand`, `fetch_max`, and `fetch_min`.
        ///
        /// Naturally, `AtomicBool` does not support the arithmetic-specific operations.
        ///
        /// In general, the syntax is:
        ///
        ///     let result = atomic_with_ghost!(
        ///         $atomic => $operation_name($operands...);
        ///         update $prev -> $next;         // `update` line is optional
        ///         returning $ret;                // `returning` line is optional
        ///         ghost $g => {
        ///             /* Proof code with access to `tracked` variable `g: G` */
        ///         }
        ///     );
        ///
        /// Here, the `$operation_name` is one of `load`, `store`, etc. Meanwhile,
        /// `$prev`, `$next`, and `$ret` are all identifiers which 
        /// will be available as spec variable inside the block to describe the
        /// atomic action which is performed.
        ///
        /// For example, suppose the user performs `fetch_add(1)`. The atomic
        /// operation might load the value 5, add 1, store the value 6,
        /// and return the original value, 5. In that case, we would have
        /// `prev == 5`, `next == 6`, and `ret == 5`.
        ///
        /// The specification for a given operation is given as a relation between
        /// `prev`, `next`, and `ret`; that is, at the beginning of the proof block,
        /// the user may assume the given specification holds:
        ///
        /// | operation                     | specification                                                                                                              |
        /// |-------------------------------|----------------------------------------------------------------------------------------------------------------------------|
        /// | `load()`                      | `next == prev && rev == prev`                                                                                              |
        /// | `store(x)`                    | `next == x && ret == ()`                                                                                                   |
        /// | `swap(x)`                     | `next == x && ret == prev`                                                                                                 |
        /// | `compare_exchange(x, y)`      | `prev == x && next == y && ret == Ok(prev)` ("success") OR<br> `prev != x && next == prev && ret == Err(prev)` ("failure") |
        /// | `compare_exchange_weak(x, y)` | `prev == x && next == y && ret == Ok(prev)` ("success") OR<br> `next == prev && ret == Err(prev)` ("failure")              |
        /// | `fetch_add(x)` (*)            | `next == prev + x && ret == prev`                                                                                          |
        /// | `fetch_add_wrapping(x)`       | `next == wrapping_add(prev, x) && ret == prev`                                                                             |
        /// | `fetch_sub(x)` (*)            | `next == prev - x && ret == prev`                                                                                          |
        /// | `fetch_sub_wrapping(x)`       | `next == wrapping_sub(prev, x) && ret == prev`                                                                             |
        /// | `fetch_or(x)`                 | <code>next == prev \| x && ret == prev</code>                                                                              |
        /// | `fetch_and(x)`                | `next == prev & x && ret == prev`                                                                                          |
        /// | `fetch_xor(x)`                | `next == prev ^ x && ret == prev`                                                                                          |
        /// | `fetch_nand(x)`               | `next == !(prev & x) && ret == prev`                                                                                       |
        /// | `fetch_max(x)`                | `next == max(prev, x) && ret == prev`                                                                                      |
        /// | `fetch_min(x)`                | `next == max(prev, x) && ret == prev`                                                                                      |
        /// | `no_op()` (**)                | `next == prev && ret == ()`                                                                                                |
        ///
        /// (*) Note that `fetch_add` and `fetch_sub` do not specify
        /// wrapping-on-overflow; instead, they require the user to
        /// prove that overflow _does not occur_, i.e., the user must show
        /// that `next` is in bounds for the integer type in question.
        /// Furthermore, for `fetch_add` and `fetch_sub`, the spec values of
        /// `prev`, `next`, and `ret` are all given with type `int`, so the
        /// user may reason about boundedness within the proof block.
        ///
        /// (As executable code, `fetch_add` is equivalent to `fetch_add_wrapping`,
        /// and likewise for `fetch_sub` and `fetch_sub_wrapping`.
        /// We have both because it's frequently the case that the user needs to verify
        /// lack-of-overflow _anyway_, and having it as an explicit precondition by default
        /// then makes verification errors easier to diagnose. Furthermore, when overflow is
        /// intended, the wrapping operations document that intent.)
        ///
        /// (**) `no_op` is entirely a ghost operation and doesn't emit any actual instruction.
        /// This allows the user to access the ghost state and the stored value (as `spec` data)
        /// without actually performing a load.
        ///
        /// ---
        ///
        /// At the beginning of the proof block, the user may assume, in addition
        /// to the specified relation between `prev`, `next`, and `ret`, that
        /// `atomic.inv(prev, g)` holds. The user is required to update `g` such that
        /// `atomic.inv(next, g)` holds at the end of the block.
        /// In other words, the ghost block has the implicit pre- and post-conditions:
        ///
        ///     let result = atomic_with_ghost!(
        ///         $atomic => $operation_name($operands...);
        ///         update $prev -> $next;
        ///         returning $ret;
        ///         ghost $g => {
        ///             assume(specified relation on (prev, next, ret));
        ///             assume(atomic.inv(prev, g));
        ///
        ///             // User code here; may update variable `g` with full
        ///             // access to variables in the outer context.
        ///
        ///             assert(atomic.inv(next, g));
        ///         }
        ///     );
        ///
        /// Note that the necessary action on ghost state might depend
        /// on the result of the operation; for example, if the user performs a
        /// compare-and-swap, then the ghost action that they then need to do
        /// will probably depend on whether the operation succeeded or not.
        ///
        /// The value returned by the `atomic_with_ghost!(...)` expression will be equal
        /// to `ret`, although the return value is an `exec` value (the actual result of
        /// the operation) while `ret` is a `spec` value.
        ///
        /// ### Example (TODO)
        #[macro_export]
        macro_rules! atomic_with_ghost {
            ($atomic : expr => $operation_name : ident($($operands : tt) *) ;
             update $prev : ident -> $next : ident ; returning $ret : ident ;
             ghost $g : ident => $b : block) =>
            {
                atomic_with_ghost_inner!
                ($operation_name, $atomic, ($($operands) *), $prev, $next,
                 $ret, $g, $b)
            } ;
            ($atomic : expr => $operation_name : ident($($operands : tt) *) ;
             update $prev : ident -> $next : ident ; ghost $g : ident => $b :
             block) =>
            {
                atomic_with_ghost_inner!
                ($operation_name, $atomic, ($($operands) *), $prev, $next, _,
                 $g, $b)
            } ;
            ($atomic : expr => $operation_name : ident($($operands : tt) *) ;
             returning $ret : ident ; ghost $g : ident => $b : block) =>
            {
                atomic_with_ghost_inner!
                ($operation_name, $atomic, ($($operands) *), _, _, $ret, $g,
                 $b)
            } ;
            ($atomic : expr => $operation_name : ident($($operands : tt) *) ;
             ghost $g : ident => $b : block) =>
            {
                atomic_with_ghost_inner!
                ($operation_name, $atomic, ($($operands) *), _, _, _, $g, $b)
            } ;
        }
        #[doc(hidden)]
        #[macro_export]
        macro_rules! atomic_with_ghost_inner {
            (load, $e : expr, (), $prev : pat, $next : pat, $ret : pat, $g :
             ident, $b : block) =>
            { atomic_with_ghost_load! ($e, $prev, $next, $ret, $g, $b) } ;
            (store, $e : expr, ($operand : expr), $prev : pat, $next : pat,
             $ret : pat, $g : ident, $b : block) =>
            {
                atomic_with_ghost_store!
                ($e, $operand, $prev, $next, $ret, $g, $b)
            } ;
            (swap, $e : expr, ($operand : expr), $prev : pat, $next : pat,
             $ret : pat, $g : ident, $b : block) =>
            {
                atomic_with_ghost_update_with_1_operand!
                (swap, $e, $operand, $prev, $next, $ret, $g, $b)
            } ;
            (fetch_or, $e : expr, ($operand : expr), $prev : pat, $next : pat,
             $ret : pat, $g : ident, $b : block) =>
            {
                atomic_with_ghost_update_with_1_operand!
                (fetch_or, $e, $operand, $prev, $next, $ret, $g, $b)
            } ;
            (fetch_and, $e : expr, ($operand : expr), $prev : pat, $next :
             pat, $ret : pat, $g : ident, $b : block) =>
            {
                atomic_with_ghost_update_with_1_operand!
                (fetch_and, $e, $operand, $prev, $next, $ret, $g, $b)
            } ;
            (fetch_xor, $e : expr, ($operand : expr), $prev : pat, $next :
             pat, $ret : pat, $g : ident, $b : block) =>
            {
                atomic_with_ghost_update_with_1_operand!
                (fetch_xor, $e, $operand, $prev, $next, $ret, $g, $b)
            } ;
            (fetch_nand, $e : expr, ($operand : expr), $prev : pat, $next :
             pat, $ret : pat, $g : ident, $b : block) =>
            {
                atomic_with_ghost_update_with_1_operand!
                (fetch_nand, $e, $operand, $prev, $next, $ret, $g, $b)
            } ;
            (fetch_max, $e : expr, ($operand : expr), $prev : pat, $next :
             pat, $ret : pat, $g : ident, $b : block) =>
            {
                atomic_with_ghost_update_with_1_operand!
                (fetch_max, $e, $operand, $prev, $next, $ret, $g, $b)
            } ;
            (fetch_min, $e : expr, ($operand : expr), $prev : pat, $next :
             pat, $ret : pat, $g : ident, $b : block) =>
            {
                atomic_with_ghost_update_with_1_operand!
                (fetch_min, $e, $operand, $prev, $next, $ret, $g, $b)
            } ;
            (fetch_add_wrapping, $e : expr, ($operand : expr), $prev : pat,
             $next : pat, $ret : pat, $g : ident, $b : block) =>
            {
                atomic_with_ghost_update_with_1_operand!
                (fetch_add_wrapping, $e, $operand, $prev, $next, $ret, $g, $b)
            } ;
            (fetch_sub_wrapping, $e : expr, ($operand : expr), $prev : pat,
             $next : pat, $ret : pat, $g : ident, $b : block) =>
            {
                atomic_with_ghost_update_with_1_operand!
                (fetch_sub_wrapping, $e, $operand, $prev, $next, $ret, $g, $b)
            } ;
            (fetch_add, $e : expr, ($operand : expr), $prev : pat, $next :
             pat, $ret : pat, $g : ident, $b : block) =>
            {
                atomic_with_ghost_update_fetch_add!
                ($e, $operand, $prev, $next, $ret, $g, $b)
            } ;
            (fetch_sub, $e : expr, ($operand : expr), $prev : pat, $next :
             pat, $ret : pat, $g : ident, $b : block) =>
            {
                atomic_with_ghost_update_fetch_sub!
                ($e, $operand, $prev, $next, $ret, $g, $b)
            } ;
            (compare_exchange, $e : expr,
             ($operand1 : expr, $operand2 : expr), $prev : pat, $next : pat,
             $ret : pat, $g : ident, $b : block) =>
            {
                atomic_with_ghost_update_with_2_operand!
                (compare_exchange, $e, $operand1, $operand2, $prev, $next,
                 $ret, $g, $b)
            } ;
            (compare_exchange_weak, $e : expr,
             ($operand1 : expr, $operand2 : expr), $prev : pat, $next : pat,
             $ret : pat, $g : ident, $b : block) =>
            {
                atomic_with_ghost_update_with_2_operand!
                (compare_exchange_weak, $e, $operand1, $operand2, $prev,
                 $next, $ret, $g, $b)
            } ;
            (no_op, $e : expr, (), $prev : pat, $next : pat, $ret : pat, $g :
             ident, $b : block) =>
            { atomic_with_ghost_no_op! ($e, $prev, $next, $ret, $g, $b) } ;
        }
        #[doc(hidden)]
        #[macro_export]
        macro_rules! atomic_with_ghost_store {
            ($e : expr, $operand : expr, $prev : pat, $next : pat, $res : pat,
             $g : ident, $b : block) =>
            {
                {
                    let atomic = & $e ; crate :: open_atomic_invariant!
                    (& atomic.atomic_inv => pair =>
                     {
                         #[allow(unused_mut)] #[proof] let(mut perm, mut $g) =
                         pair ; #[spec] let $prev = perm.view().value ;
                         atomic.patomic.store(& mut perm, $operand) ; #[spec]
                         let $next = perm.view().value ; #[spec] let $res = ()
                         ; { $b } pair = (perm, $g) ;
                     }) ;
                }
            }
        }
        #[doc(hidden)]
        #[macro_export]
        macro_rules! atomic_with_ghost_load {
            ($e : expr, $prev : pat, $next : pat, $res : pat, $g : ident, $b :
             block) =>
            {
                {
                    let result ; let atomic = & $e ; crate ::
                    open_atomic_invariant!
                    (& atomic.atomic_inv => pair =>
                     {
                         #[allow(unused_mut)] #[proof] let(perm, mut $g) =
                         pair ; result = atomic.patomic.load(& perm) ; #[spec]
                         let $res = result ; #[spec] let $prev = result ;
                         #[spec] let $next = result ; { $b } pair = (perm, $g)
                         ;
                     }) ; result
                }
            }
        }
        #[doc(hidden)]
        #[macro_export]
        macro_rules! atomic_with_ghost_no_op {
            ($e : expr, $prev : pat, $next : pat, $res : pat, $g : ident, $b :
             block) =>
            {
                {
                    let atomic = & $e ; crate :: open_atomic_invariant!
                    (& atomic.atomic_inv => pair =>
                     {
                         #[allow(unused_mut)] #[proof] let(perm, mut $g) =
                         pair ; #[spec] let $res = result ; #[spec] let $prev
                         = result ; #[spec] let $next = result ; { $b } pair =
                         (perm, $g) ;
                     }) ;
                }
            }
        }
        #[doc(hidden)]
        #[macro_export]
        macro_rules! atomic_with_ghost_update_with_1_operand {
            ($name : ident, $e : expr, $operand : expr, $prev : pat, $next :
             pat, $res : pat, $g : ident, $b : block) =>
            {
                {
                    let result ; let atomic = & $e ; crate ::
                    open_atomic_invariant!
                    (& atomic.atomic_inv => pair =>
                     {
                         #[allow(unused_mut)] #[proof] let(mut perm, mut $g) =
                         pair ; #[spec] let $prev = perm.view().value ; result
                         = atomic.patomic.$name(& mut perm, $operand) ;
                         #[spec] let $res = result ; #[spec] let $next =
                         perm.view().value ; { $b } pair = (perm, $g) ;
                     }) ; result
                }
            }
        }
        #[doc(hidden)]
        #[macro_export]
        macro_rules! atomic_with_ghost_update_with_2_operand {
            ($name : ident, $e : expr, $operand1 : expr, $operand2 : expr,
             $prev : pat, $next : pat, $res : pat, $g : ident, $b : block) =>
            {
                {
                    let result ; let atomic = & $e ; crate ::
                    open_atomic_invariant!
                    (& atomic.atomic_inv => pair =>
                     {
                         #[allow(unused_mut)] #[proof] let(mut perm, mut $g) =
                         pair ; #[spec] let $prev = perm.view().value ; result
                         =
                         atomic.patomic.$name(& mut perm, $operand1,
                                              $operand2) ; #[spec] let $res =
                         result ; #[spec] let $next = perm.view().value ;
                         { $b } pair = (perm, $g) ;
                     }) ; result
                }
            }
        }
        #[doc(hidden)]
        #[macro_export]
        macro_rules! atomic_with_ghost_update_fetch_add {
            ($e : expr, $operand : expr, $prev : pat, $next : pat, $res : pat,
             $g : ident, $b : block) =>
            {
                {
                    let result ; let atomic = & $e ; crate ::
                    open_atomic_invariant!
                    (& atomic.atomic_inv => pair =>
                     {
                         #[allow(unused_mut)] #[proof] let(mut perm, mut $g) =
                         pair ; #[spec] let $prev = perm.view().value as int ;
                         let op = $operand ; #[spec] let computed =
                         perm.view().value + (op as int) ; #[spec] let $res =
                         computed ; #[spec] let $next = computed ; { $b }
                         result = atomic.patomic.fetch_add(& mut perm, op) ;
                         pair = (perm, $g) ;
                     }) ; result
                }
            }
        }
        #[doc(hidden)]
        #[macro_export]
        macro_rules! atomic_with_ghost_update_fetch_sub {
            ($e : expr, $operand : expr, $prev : pat, $next : pat, $res : pat,
             $g : ident, $b : block) =>
            {
                {
                    let result ; let atomic = & $e ; crate ::
                    open_atomic_invariant!
                    (& atomic.atomic_inv => pair =>
                     {
                         #[allow(unused_mut)] #[proof] let(mut perm, mut $g) =
                         pair ; #[spec] let $prev = perm.view().value as int ;
                         let op = $operand ; #[spec] let computed =
                         perm.view().value - (op as int) ; #[spec] let $res =
                         computed ; #[spec] let $next = computed ; { $b }
                         result = atomic.patomic.fetch_sub(& mut perm, op) ;
                         pair = (perm, $g) ;
                     }) ; result
                }
            }
        }
    }
    pub mod modes {
        #[allow(unused_imports)]
        use builtin::*;
        #[allow(unused_imports)]
        use builtin_macros::*;
        #[allow(unused_imports)]
        use crate::pervasive::*;
        use core::marker::PhantomData;
        #[verifier(external_body)]
        pub fn ghost_exec<A>(#[spec] a: A) -> Ghost<A> {
            ensures(|s: Ghost<A>| equal(a, s.view()));
            Ghost::assume_new()
        }
        #[verifier(external_body)]
        pub fn tracked_exec<A>(#[proof] a: A) -> Tracked<A> {
            ensures(|s: Tracked<A>| equal(a, s.view()));
            opens_invariants_none();
            Tracked::assume_new()
        }
        #[verifier(external_body)]
        pub fn tracked_exec_borrow<'a, A>(#[proof] a: &'a A)
         -> &'a Tracked<A> {
            ensures(|s: Tracked<A>| equal(*a, s.view()));
            opens_invariants_none();
            ::core::panicking::panic("not implemented");
        }
        pub struct Gho<A>(
                          #[spec]
                          pub A);
        pub struct Trk<A>(
                          #[proof]
                          pub A);
        #[inline(always)]
        #[verifier(external_body)]
        #[verifier(verus_macro)]
        pub fn ghost_unwrap_gho<A>(a: Ghost<Gho<A>>) -> Ghost<A> {
            ::builtin::ensures(|ret: Ghost<A>|
                                   [::builtin::equal((a.view()).0,
                                                     (ret.view()))]);
            Ghost::assume_new()
        }
        #[inline(always)]
        #[verifier(external_body)]
        #[verifier(verus_macro)]
        pub fn tracked_unwrap_gho<A>(a: Tracked<Gho<A>>) -> Tracked<A> {
            ::builtin::ensures(|ret: Tracked<A>|
                                   [::builtin::equal((a.view()).0,
                                                     (ret.view()))]);
            Tracked::assume_new()
        }
        #[inline(always)]
        #[verifier(external_body)]
        #[verifier(verus_macro)]
        pub fn tracked_unwrap_trk<A>(a: Tracked<Trk<A>>) -> Tracked<A> {
            ::builtin::ensures(|ret: Tracked<A>|
                                   [::builtin::equal((a.view()).0,
                                                     (ret.view()))]);
            Tracked::assume_new()
        }
        #[verifier(external_body)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn tracked_swap<V>(#[proof] a: &mut V, #[proof] b: &mut V) {
            ::builtin::ensures([::builtin::equal(a, old(b)),
                                ::builtin::equal(b, old(a))]);
            ::core::panicking::panic("not implemented");
        }
        #[verifier(external_body)]
        pub struct Spec<#[verifier(strictly_positive)] A> {
            phantom: PhantomData<A>,
        }
        pub struct Proof<A>(
                            #[proof]
                            pub A);
        impl <A> Spec<A> {
            #[spec]
            #[verifier(external_body)]
            pub fn value(self) -> A {
                ::core::panicking::panic("not implemented")
            }
            #[verifier(external_body)]
            pub fn exec(#[spec] a: A) -> Spec<A> {
                ensures(|s: Spec<A>| equal(a, s.value()));
                Spec{phantom: PhantomData,}
            }
            #[proof]
            #[verifier(returns(proof))]
            #[verifier(external_body)]
            pub fn proof(a: A) -> Spec<A> {
                ensures(|s: Spec<A>| equal(a, s.value()));
                Spec{phantom: PhantomData,}
            }
        }
        impl <A> Clone for Spec<A> {
            #[verifier(external_body)]
            fn clone(&self) -> Self { Spec{phantom: PhantomData,} }
        }
        impl <A> Copy for Spec<A> { }
        impl <A> PartialEq for Spec<A> {
            #[verifier(external_body)]
            fn eq(&self, _rhs: &Spec<A>) -> bool { true }
        }
        impl <A> Eq for Spec<A> { }
        impl <A> PartialEq for Proof<A> {
            #[verifier(external_body)]
            fn eq(&self, _rhs: &Proof<A>) -> bool { true }
        }
        impl <A> Eq for Proof<A> { }
        #[allow(dead_code)]
        #[inline(always)]
        pub fn exec_proof_from_false<A>() -> Proof<A> {
            requires(false);
            Proof(proof_from_false())
        }
    }
    pub mod multiset {
        use core::{marker};
        #[allow(unused_imports)]
        use builtin::*;
        #[allow(unused_imports)]
        use builtin_macros::*;
        #[allow(unused_imports)]
        use crate::pervasive::*;
        #[allow(unused_imports)]
        use crate::pervasive::set::*;
        #[doc =
          " `Multiset<V>` is an abstract multiset type for specifications."]
        #[doc = ""]
        #[doc =
          " `Multiset<V>` can be encoded as a (total) map from elements to natural numbers,"]
        #[doc = " where the number of nonzero entries is finite."]
        #[doc = ""]
        #[doc = " Multisets can be constructed in a few different ways:"]
        #[doc = "  * [`Multiset::empty()`] constructs an empty multiset."]
        #[doc =
          "  * By manipulating existings multisets with [`Multiset::add`], [`Multiset::insert`],"]
        #[doc =
          "    [`Multiset::sub`], [`Multiset::remove`], or [`Multiset::filter`]."]
        #[doc =
          "  * TODO: `multiset!` constructor macro, multiset from set, from map, etc."]
        #[doc = ""]
        #[doc =
          " To prove that two multisets are equal, it is usually easiest to use the "]
        #[doc = " [`assert_multisets_equal!`] macro."]
        #[verifier(external_body)]
        pub struct Multiset<#[verifier(strictly_positive)] V> {
            dummy: marker::PhantomData<V>,
        }
        impl <V> Multiset<V> {
            #[doc =
              " Returns the _count_, or _multiplicity_ of a single value within the multiset."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn count(self, value: V) -> nat {
                ::core::panicking::panic("not implemented")
            }
            #[doc =
              " The total size of the multiset, i.e., the sum of all multiplicities over all values."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn len(self) -> nat {
                ::core::panicking::panic("not implemented")
            }
            #[doc = " An empty multiset."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn empty() -> Self {
                ::core::panicking::panic("not implemented")
            }
            #[doc =
              " A singleton multiset, i.e., a multiset with a single element of multiplicity 1."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn singleton(v: V) -> Self {
                ::core::panicking::panic("not implemented")
            }
            #[doc =
              " Takes the union of two multisets. For a given element, its multiplicity in"]
            #[doc =
              " the resulting multiset is the sum of its multiplicities in the operands."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn add(self, m2: Self) -> Self {
                ::core::panicking::panic("not implemented")
            }
            #[doc = " Takes the difference of two multisets."]
            #[doc =
              " The multiplicities of `m2` are subtracted from those of `self`; if any element"]
            #[doc =
              " occurs more in `m2` then the resulting multiplicity bottoms out at 0."]
            #[doc =
              " (See [`axiom_multiset_sub`] for the precise definition.)"]
            #[doc = ""]
            #[doc =
              " Note in particular that `self === self.sub(m).add(m)` only holds if"]
            #[doc = " `m` is included in `self`."]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn sub(self, m2: Self) -> Self {
                ::core::panicking::panic("not implemented")
            }
            #[doc = " Inserts one instance the value `v` into the multiset."]
            #[doc = ""]
            #[doc =
              " This always increases the total size of the multiset by 1."]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn insert(self, v: V) -> Self { self.add(Self::singleton(v)) }
            #[doc =
              " Removes one instance of the value `v` from the multiset."]
            #[doc = ""]
            #[doc =
              " If `v` was absent from the multiset, then the multiset is unchanged."]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn remove(self, v: V) -> Self { self.sub(Self::singleton(v)) }
            #[doc =
              " Returns `true` is the left argument is contained in the right argument,"]
            #[doc =
              " that is, if for each value `v`, the number of occurences in the left"]
            #[doc = " is at most the number of occurences in the right."]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn le(self, m2: Self) -> bool {
                ::builtin::forall(|v: V| (self.count(v)).spec_le(m2.count(v)))
            }
            #[doc =
              " Returns true if the two multisets are pointwise equal, i.e.,"]
            #[doc =
              " for every value `v: V`, the counts are the same in each multiset."]
            #[doc =
              " This is equivalent to the multisets actually being equal"]
            #[doc = " by [`axiom_multiset_ext_equal`]."]
            #[doc = ""]
            #[doc =
              " To prove that two maps are equal via extensionality, it is generally easier"]
            #[doc =
              " to use the [`assert_multisets_equal!`] macro, rather than using `ext_equal` directly."]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn ext_equal(self, m2: Self) -> bool {
                ::builtin::forall(|v: V|
                                      ::builtin::spec_eq(self.count(v),
                                                         m2.count(v)))
            }
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn filter(self, f: impl Fn(V) -> bool) -> Self {
                ::core::panicking::panic("not implemented")
            }
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_multiset_empty<V>(v: V) {
            ::builtin::ensures([::builtin::spec_eq(Multiset::empty().count(v),
                                                   ::builtin::spec_literal_nat("0"))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_multiset_singleton<V>(v: V) {
            ::builtin::ensures([::builtin::spec_eq(Multiset::singleton(v).count(v),
                                                   ::builtin::spec_literal_nat("1"))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_multiset_singleton_different<V>(v: V, w: V) {
            ::builtin::ensures([::builtin::imply(!::builtin::equal(v, w),
                                                 ::builtin::spec_eq(Multiset::singleton(v).count(w),
                                                                    ::builtin::spec_literal_nat("0")))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_multiset_add<V>(m1: Multiset<V>, m2: Multiset<V>, v: V) {
            ::builtin::ensures([::builtin::spec_eq(m1.add(m2).count(v),
                                                   (m1.count(v)).spec_add(m2.count(v)))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_multiset_sub<V>(m1: Multiset<V>, m2: Multiset<V>, v: V) {
            ::builtin::ensures([::builtin::spec_eq(m1.sub(m2).count(v),
                                                   if (m1.count(v)).spec_ge(m2.count(v))
                                                      {
                                                       (m1.count(v)).spec_sub(m2.count(v))
                                                   } else {
                                                       ::builtin::spec_literal_integer("0")
                                                   })]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_multiset_ext_equal<V>(m1: Multiset<V>, m2: Multiset<V>) {
            ::builtin::ensures([::builtin::spec_eq(m1.ext_equal(m2),
                                                   equal(m1, m2))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_len_empty<V>() {
            ::builtin::ensures([::builtin::spec_eq((#[trigger] Multiset::<V>::empty().len()),
                                                   ::builtin::spec_literal_nat("0"))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_len_singleton<V>(v: V) {
            ::builtin::ensures([::builtin::spec_eq((#[trigger] Multiset::<V>::singleton(v).len()),
                                                   ::builtin::spec_literal_nat("1"))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_len_add<V>(m1: Multiset<V>, m2: Multiset<V>) {
            ::builtin::ensures([::builtin::spec_eq((#[trigger] m1.add(m2).len()),
                                                   (m1.len()).spec_add(m2.len()))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_count_le_len<V>(m: Multiset<V>, v: V) {
            ::builtin::ensures([(#[trigger] m.count(v)).spec_le(#[trigger] m.len())]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_filter_count<V>(m: Multiset<V>, f: impl Fn(V) -> bool,
                                     v: V) {
            ::builtin::ensures([::builtin::spec_eq((#[trigger] m.filter(f).count(v)),
                                                   if f(v) {
                                                       m.count(v)
                                                   } else {
                                                       ::builtin::spec_literal_integer("0")
                                                   })]);
        }
        #[macro_export]
        macro_rules! assert_multisets_equal {
            ($m1 : expr, $m2 : expr $(,) ?) =>
            { assert_multisets_equal! ($m1, $m2, key => { }) } ;
            ($m1 : expr, $m2 : expr, $k : ident $(: $t : ty) ? => $bblock :
             block) =>
            {
                #[spec] let m1 = $m1 ; #[spec] let m2 = $m2 ; :: builtin ::
                assert_by(:: builtin :: equal(m1, m2),
                          {
                              :: builtin ::
                              assert_forall_by(| $k $(: $t) ? |
                                               {
                                                   :: builtin ::
                                                   ensures([:: builtin ::
                                                            equal(m1.count($k),
                                                                  m2.count($k))])
                                                   ; { $bblock }
                                               }) ; $crate :: pervasive ::
                              assert(m1.ext_equal(m2)) ;
                          }) ;
            }
        }
    }
    pub mod state_machine_internal {
        //! Helper utilities used by the `state_machine_macros` codegen.
        #![allow(unused_imports)]
        #![doc(hidden)]
        use builtin::*;
        use builtin_macros::*;
        use crate::pervasive::*;
        use crate::pervasive::seq::*;
        use crate::pervasive::map::*;
        use crate::pervasive::option::*;
        #[verifier(external_body)]
        pub struct SyncSendIfSyncSend<#[verifier(strictly_positive)] T> {
            _sync_send: builtin::SyncSendIfSyncSend<T>,
        }
        #[verifier(external_body)]
        pub struct NoCopy {
            _no_copy: builtin::NoCopy,
        }
        #[proof]
        #[verifier(custom_req_err("unable to prove assertion safety condition"))]
        pub fn assert_safety(b: bool) { requires(b); ensures(b); }
        #[proof]
        #[verifier(custom_req_err("unable to prove safety condition that the pattern matches"))]
        pub fn assert_let_pattern(b: bool) { requires(b); ensures(b); }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: to add a value Some(_), field must be None before the update"))]
        pub fn assert_add_option(b: bool) { requires(b); ensures(b); }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: to add a singleton set, the value must not be in the set before the update"))]
        pub fn assert_add_set(b: bool) { requires(b); ensures(b); }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: to add a value `true`, field must be `false` before the update"))]
        pub fn assert_add_bool(b: bool) { requires(b); ensures(b); }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: the given key must be absent from the map before the update"))]
        pub fn assert_add_map(b: bool) { requires(b); ensures(b); }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: if the key is already in the map, its existing value must agree with the provided value"))]
        pub fn assert_add_persistent_map(b: bool) { requires(b); ensures(b); }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: if the previous value is Some(_), then this existing value must agree with the newly provided value"))]
        pub fn assert_add_persistent_option(b: bool) {
            requires(b);
            ensures(b);
        }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: the given value to be withdrawn must be stored before the withdraw"))]
        pub fn assert_withdraw_option(b: bool) { requires(b); ensures(b); }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: to deposit a value into Some(_), the field must be None before the deposit"))]
        pub fn assert_deposit_option(b: bool) { requires(b); ensures(b); }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: the value being guarded must be stored"))]
        pub fn assert_guard_option(b: bool) { requires(b); ensures(b); }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: the value to be withdrawn must be stored at the given key before the withdraw"))]
        pub fn assert_withdraw_map(b: bool) { requires(b); ensures(b); }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: the given key must be absent from the map before the deposit"))]
        pub fn assert_deposit_map(b: bool) { requires(b); ensures(b); }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: the value being guarded must be stored at the given key"))]
        pub fn assert_guard_map(b: bool) { requires(b); ensures(b); }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: the optional values being composed cannot both be Some(_)"))]
        pub fn assert_general_add_option(b: bool) { requires(b); ensures(b); }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: the sets being composed must be disjoint"))]
        pub fn assert_general_add_set(b: bool) { requires(b); ensures(b); }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: the boolean values being composed cannot both be `true`"))]
        pub fn assert_general_add_bool(b: bool) { requires(b); ensures(b); }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: the key domains of the maps being composed must be disjoint"))]
        pub fn assert_general_add_map(b: bool) { requires(b); ensures(b); }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: the maps being composed must agree on their values for any key in both domains"))]
        pub fn assert_general_add_persistent_map(b: bool) {
            requires(b);
            ensures(b);
        }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: if the previous value and the newly added values are both Some(_), then their values must agree"))]
        pub fn assert_general_add_persistent_option(b: bool) {
            requires(b);
            ensures(b);
        }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: the optional value to be withdrawn must be stored before the withdraw"))]
        pub fn assert_general_withdraw_option(b: bool) {
            requires(b);
            ensures(b);
        }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: the optional values being composed cannot both be Some(_)"))]
        pub fn assert_general_deposit_option(b: bool) {
            requires(b);
            ensures(b);
        }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: the value being guarded must be stored"))]
        pub fn assert_general_guard_option(b: bool) {
            requires(b);
            ensures(b);
        }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: the map being withdrawn must be a submap of the stored map"))]
        pub fn assert_general_withdraw_map(b: bool) {
            requires(b);
            ensures(b);
        }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: the key domains of the maps being composed must be disjoint"))]
        pub fn assert_general_deposit_map(b: bool) {
            requires(b);
            ensures(b);
        }
        #[proof]
        #[verifier(custom_req_err("unable to prove inherent safety condition: the map being guarded must be a submap of the stored map"))]
        pub fn assert_general_guard_map(b: bool) { requires(b); ensures(b); }
        #[doc(hidden)]
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn nat_max(a: nat, b: nat) -> nat {
            if (a).spec_gt(b) { a } else { b }
        }
        #[doc(hidden)]
        impl <A> Seq<A> {
            #[verifier(inline)]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn update_at_index(self, i: int, a: A) -> Self {
                ::builtin::recommends([::builtin::spec_chained_cmp(::builtin::spec_chained_lt(::builtin::spec_chained_le(::builtin::spec_chained_value(::builtin::spec_literal_nat("0")),
                                                                                                                         i),
                                                                                              self.len()))]);
                self.update(i, a)
            }
        }
        #[doc(hidden)]
        impl <K, V> Map<K, V> {
            #[verifier(inline)]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn update_at_index(self, k: K, v: V) -> Self {
                self.insert(k, v)
            }
        }
        #[doc(hidden)]
        #[verifier(inline)]
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn opt_is_none<V>(a: Option<V>) -> bool { a.is_None() }
        #[doc(hidden)]
        #[verifier(inline)]
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn opt_ge<V>(a: Option<V>, b: Option<V>) -> bool {
            ::builtin::imply(b.is_Some(), ::builtin::equal(a, b))
        }
        #[doc(hidden)]
        #[verifier(inline)]
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn opt_add<V>(a: Option<V>, b: Option<V>) -> Option<V> {
            if b.is_Some() { b } else { a }
        }
        #[doc(hidden)]
        #[verifier(inline)]
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn opt_agree<V>(a: Option<V>, b: Option<V>) -> bool {
            ::builtin::imply(a.is_Some() && b.is_Some(),
                             ::builtin::equal(a.get_Some_0(), b.get_Some_0()))
        }
        #[doc(hidden)]
        #[verifier(inline)]
        #[verifier(verus_macro)]
        #[verifier(publish)]
        #[spec]
        pub fn opt_sub<V>(a: Option<V>, b: Option<V>) -> Option<V> {
            if b.is_Some() { Option::None } else { a }
        }
    }
    #[cfg(not(feature = "non_std"))]
    pub mod thread {
        #[allow(unused_imports)]
        use builtin::*;
        #[allow(unused_imports)]
        use builtin_macros::*;
        #[allow(unused_imports)]
        use crate::pervasive::*;
        #[allow(unused_imports)]
        use crate::pervasive::result::*;
        pub trait Spawnable<Ret: Sized>: Sized {
            #[verifier(verus_macro)]
            #[spec]
            fn pre(self) -> bool { ::builtin::no_method_body() }
            #[verifier(verus_macro)]
            #[spec]
            fn post(self, ret: Ret) -> bool { ::builtin::no_method_body() }
            #[verifier(verus_macro)]
            #[exec]
            fn run(self) -> Ret {
                ::builtin::requires([self.pre()]);
                ::builtin::ensures(|r: Ret| [self.post(r)]);
                ::builtin::no_method_body()
            }
        }
        #[verifier(external_body)]
        pub struct JoinHandle<#[verifier(maybe_negative)] Ret> {
            handle: std::thread::JoinHandle<Ret>,
        }
        impl <Ret> JoinHandle<Ret> {
            #[spec]
            #[verifier(external_body)]
            pub fn predicate(&self, ret: Ret) -> bool {
                ::core::panicking::panic("not implemented")
            }
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn join(self) -> Result<Ret, ()> {
                ensures(|r: Result<Ret, ()>|
                            ::builtin::imply(r.is_Ok(),
                                             self.predicate(r.get_Ok_0())));
                let res =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                                                                              {
                                                                                  match self.handle.join()
                                                                                      {
                                                                                      Ok(r)
                                                                                      =>
                                                                                      Result::Ok(r),
                                                                                      Err(_)
                                                                                      =>
                                                                                      Result::Err(()),
                                                                                  }
                                                                              }));
                match res {
                    Ok(res) => res,
                    Err(_) => {
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(&["panic on join\n"],
                                                                             &match ()
                                                                                  {
                                                                                  _args
                                                                                  =>
                                                                                  [],
                                                                              }));
                        };
                        std::process::abort();
                    }
                }
            }
        }
        #[verifier(external_body)]
        #[verifier(verus_macro)]
        pub fn spawn<Param: Spawnable<Ret> + Send + 'static, Ret: Send +
                     'static>(p: Param) -> JoinHandle<Ret> {
            requires(p.pre());
            ensures(|handle: JoinHandle<Ret>|
                        forall(|ret: Ret|
                                   ::builtin::imply(handle.predicate(ret),
                                                    p.post(ret))));
            let res =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                                                                          {
                                                                              let handle =
                                                                                  std::thread::spawn(move
                                                                                                         ||
                                                                                                         p.run());
                                                                              JoinHandle{handle,}
                                                                          }));
            match res {
                Ok(res) => res,
                Err(_) => {
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(&["panic on spawn\n"],
                                                                         &match ()
                                                                              {
                                                                              _args
                                                                              =>
                                                                              [],
                                                                          }));
                    };
                    std::process::abort();
                }
            }
        }
    }
    #[cfg(not(feature = "no_global_allocator"))]
    pub mod ptr {
        use core::{marker, mem, mem::MaybeUninit};
        extern crate alloc;
        #[allow(unused_imports)]
        use builtin::*;
        #[allow(unused_imports)]
        use builtin_macros::*;
        #[allow(unused_imports)]
        use crate::pervasive::*;
        #[allow(unused_imports)]
        use crate::pervasive::modes::*;
        #[doc = " `PPtr<V>` (which stands for \"permissioned pointer\")"]
        #[doc = " is a wrapper around a raw pointer to `V` on the heap."]
        #[doc = ""]
        #[doc =
          " Technically, it is a wrapper around `*mut mem::MaybeUninit<V>`, that is, the object"]
        #[doc = " it points to may be uninitialized."]
        #[doc = ""]
        #[doc =
          " In order to access (read or write) the value behind the pointer, the user needs"]
        #[doc =
          " a special _ghost permission token_, [`PermissionOpt<V>`](PermissionOpt). This object is `tracked`,"]
        #[doc =
          " which means that it is \"only a proof construct\" that does not appear in code,"]
        #[doc =
          " but its uses _are_ checked by the borrow-checker. This ensures memory safety,"]
        #[doc = " data-race-freedom, prohibits use-after-free, etc."]
        #[doc = ""]
        #[doc = " ### PermissionOpt objects."]
        #[doc = ""]
        #[doc =
          " The [`PermissionOpt`] object represents both the ability to access the data behind the"]
        #[doc =
          " pointer _and_ the ability to free it (return it to the memory allocator)."]
        #[doc = ""]
        #[doc = " In particular:"]
        #[doc =
          "  * When the user owns a `PermissionOpt<V>` object associated to a given pointer,"]
        #[doc =
          "    they can either read or write its contents, or deallocate (\"free\") it."]
        #[doc =
          "  * When the user has a shared borrow, `&PermissionOpt<V>`, they can read"]
        #[doc = "    the contents (i.e., obtained a shared borrow `&V`)."]
        #[doc = ""]
        #[doc =
          " The `perm: PermissionOpt<V>` object tracks two pieces of data:"]
        #[doc =
          "  * `perm.pptr` is the pointer that the permission is associated to,"]
        #[doc = "     given by [`ptr.id()`](PPtr::id)."]
        #[doc =
          "  * `perm.value` tracks the data that is behind the pointer. Thereby:"]
        #[doc =
          "      * When the user uses the permission to _read_ a value, they always"]
        #[doc = "        read the value as given by the `perm.value`."]
        #[doc =
          "      * When the user uses the permission to _write_ a value, the `perm.value`"]
        #[doc = "        data is updated."]
        #[doc = ""]
        #[doc =
          " For those familiar with separation logic, the `PermissionOpt` object plays a role"]
        #[doc =
          " similar to that of the \"points-to\" operator, _ptr_ ↦ _value_."]
        #[doc = ""]
        #[doc = " ### Differences from `PCell`."]
        #[doc = ""]
        #[doc =
          " `PPtr` is similar to [`cell::PCell`], but has a few key differences:"]
        #[doc =
          "  * In `PCell<T>`, the type `T` is placed internally to the `PCell`, whereas with `PPtr`,"]
        #[doc = "    the type `T` is placed at some location on the heap."]
        #[doc =
          "  * Since `PPtr` is just a pointer (represented by an integer), it can be `Copy`."]
        #[doc =
          "  * The `ptr::PermissionOpt` token represents not just the permission to read/write"]
        #[doc = "    the contents, but also to deallocate."]
        #[doc = ""]
        #[doc = " ### Example (TODO)"]
        #[verifier(external_body)]
        pub struct PPtr<#[verifier(strictly_positive)] V> {
            uptr: *mut MaybeUninit<V>,
        }
        #[verifier(external)]
        unsafe impl <T> Sync for PPtr<T> { }
        #[verifier(external)]
        unsafe impl <T> Send for PPtr<T> { }
        #[doc =
          " A `tracked` ghost object that gives the user permission to dereference a pointer"]
        #[doc =
          " for reading or writing, or to free the memory at that pointer."]
        #[doc = ""]
        #[doc =
          " The meaning of a `PermissionOpt` object is given by the data in its"]
        #[doc = " `View` object, [`PermissionOptData`]."]
        #[doc = ""]
        #[doc = " See the [`PPtr`] documentation for more details."]
        #[verifier(external_body)]
        #[proof]
        pub struct PermissionOpt<#[verifier(strictly_positive)] V> {
            phantom: marker::PhantomData<V>,
            no_copy: NoCopy,
        }
        #[doc = " Represents the meaning of a [`PermissionOpt`] object."]
        #[spec]
        pub struct PermissionOptData<V> {
            #[doc =
              " Indicates that this token is for a pointer `ptr: PPtr<V>`"]
            #[doc = " such that [`ptr.id()`](PPtr::id) equal to this value."]
            pub pptr: int,
            #[doc =
              " Indicates that this token gives the ability to read a value `V` from memory."]
            #[doc =
              " When `None`, it indicates that the memory is uninitialized."]
            pub value: option::Option<V>,
        }
        impl <V> PermissionOpt<V> {
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn view(self) -> PermissionOptData<V> {
                ::core::panicking::panic("not implemented")
            }
            #[doc = " Any dereferenceable pointer must be non-null."]
            #[doc =
              " (Note that null pointers _do_ exist and are representable by `PPtr`;"]
            #[doc =
              " however, it is not possible to obtain a `PermissionOpt` token for"]
            #[doc = " any such a pointer.)"]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            #[proof]
            pub fn is_nonnull(#[proof] &self) {
                ::builtin::ensures([!::builtin::spec_eq((self.view()).pptr,
                                                        ::builtin::spec_literal_nat("0"))]);
                ::core::panicking::panic("not implemented");
            }
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            #[proof]
            pub fn leak_contents(#[proof] &mut self) {
                ::builtin::ensures([::builtin::spec_eq((self.view()).pptr,
                                                       (old(self).view()).pptr)
                                        && (self.view()).value.is_None()]);
                ::core::panicking::panic("not implemented");
            }
        }
        impl <V> PPtr<V> {
            #[doc = " Cast a pointer to an integer."]
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn to_usize(&self) -> usize {
                ::builtin::ensures(|u: usize|
                                       [::builtin::spec_eq(::builtin::spec_cast_integer::<_,
                                                                                          int>(u),
                                                           self.id())]);
                self.uptr as usize
            }
            #[doc = " integer address of the pointer"]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn id(&self) -> int {
                ::core::panicking::panic("not implemented")
            }
            #[doc = " Cast an integer to a pointer."]
            #[doc = " "]
            #[doc =
              " Note that this does _not_ require or ensure that the pointer is valid."]
            #[doc =
              " Of course, if the user creates an invalid pointer, they would still not be able to"]
            #[doc =
              " create a valid [`PermissionOpt`] token for it, and thus they would never"]
            #[doc = " be able to access the data behind the pointer."]
            #[doc = ""]
            #[doc =
              " This is analogous to normal Rust, where casting to a pointer is always possible,"]
            #[doc = " but dereferencing a pointer is an `unsafe` operation."]
            #[doc =
              " In Verus, casting to a pointer is likewise always possible,"]
            #[doc =
              " while dereferencing it is only allowed when the right preconditions are met."]
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn from_usize(u: usize) -> Self {
                ::builtin::ensures(|p: Self|
                                       [::builtin::spec_eq(p.id(),
                                                           ::builtin::spec_cast_integer::<_,
                                                                                          int>(u))]);
                let uptr = u as *mut MaybeUninit<V>;
                PPtr{uptr,}
            }
            #[doc =
              " Allocates heap memory for type `V`, leaving it uninitialized."]
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn empty() -> (PPtr<V>, Tracked<PermissionOpt<V>>) {
                ::builtin::ensures(|pt: (PPtr<V>, Tracked<PermissionOpt<V>>)|
                                       [::builtin::equal(((pt.1.view()).view()),
                                                         (PermissionOptData{pptr:
                                                                                pt.0.id(),
                                                                            value:
                                                                                option::Option::None,}))]);
                opens_invariants_none();
                let p =
                    PPtr{uptr:
                             alloc::boxed::Box::leak(alloc::boxed::Box::new(MaybeUninit::uninit())).as_mut_ptr(),};
                let _exposed_addr = p.uptr as usize;
                (p, Tracked::assume_new())
            }
            #[doc = " Clones the pointer."]
            #[doc = " TODO implement the `Clone` and `Copy` traits"]
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn clone(&self) -> PPtr<V> {
                ::builtin::ensures(|pt: PPtr<V>|
                                       [::builtin::equal(pt.id(),
                                                         self.id())]);
                opens_invariants_none();
                PPtr{uptr: self.uptr,}
            }
            #[doc =
              " Moves `v` into the location pointed to by the pointer `self`."]
            #[doc =
              " Requires the memory to be uninitialized, and leaves it initialized."]
            #[doc = ""]
            #[doc = " In the ghost perspective, this updates `perm.value`"]
            #[doc = " from `None` to `Some(v)`."]
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn put(&self, perm: &mut Tracked<PermissionOpt<V>>, v: V) {
                ::builtin::requires([::builtin::equal(self.id(),
                                                      ((old(perm).view()).view()).pptr),
                                     ::builtin::equal(((old(perm).view()).view()).value,
                                                      option::Option::None)]);
                ::builtin::ensures([::builtin::equal(((perm.view()).view()).pptr,
                                                     ((old(perm).view()).view()).pptr),
                                    ::builtin::equal(((perm.view()).view()).value,
                                                     option::Option::Some(v))]);
                opens_invariants_none();
                unsafe { *(self.uptr) = MaybeUninit::new(v); }
            }
            #[doc =
              " Moves `v` out of the location pointed to by the pointer `self`"]
            #[doc = " and returns it."]
            #[doc =
              " Requires the memory to be initialized, and leaves it uninitialized."]
            #[doc = ""]
            #[doc = " In the ghost perspective, this updates `perm@.value`"]
            #[doc = " from `Some(v)` to `None`,"]
            #[doc = " while returning the `v` as an `exec` value."]
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn take(&self, perm: &mut Tracked<PermissionOpt<V>>) -> V {
                ::builtin::requires([::builtin::equal(self.id(),
                                                      ((old(perm).view()).view()).pptr),
                                     ((old(perm).view()).view()).value.is_Some()]);
                ::builtin::ensures(|v: V|
                                       [::builtin::equal(((perm.view()).view()).pptr,
                                                         ((old(perm).view()).view()).pptr),
                                        ::builtin::equal(((perm.view()).view()).value,
                                                         option::Option::None),
                                        ::builtin::equal(v,
                                                         ((old(perm).view()).view()).value.get_Some_0())]);
                opens_invariants_none();
                unsafe {
                    let mut m = MaybeUninit::uninit();
                    mem::swap(&mut m, &mut *self.uptr);
                    m.assume_init()
                }
            }
            #[doc =
              " Swaps the `in_v: V` passed in as an argument with the value in memory."]
            #[doc =
              " Requires the memory to be initialized, and leaves it initialized with the new value."]
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn replace(&self, perm: &mut Tracked<PermissionOpt<V>>,
                           in_v: V) -> V {
                ::builtin::requires([::builtin::equal(self.id(),
                                                      ((old(perm).view()).view()).pptr),
                                     ((old(perm).view()).view()).value.is_Some()]);
                ::builtin::ensures(|out_v: V|
                                       [::builtin::equal(((perm.view()).view()).pptr,
                                                         ((old(perm).view()).view()).pptr),
                                        ::builtin::equal(((perm.view()).view()).value,
                                                         option::Option::Some(in_v)),
                                        ::builtin::equal(out_v,
                                                         ((old(perm).view()).view()).value.get_Some_0())]);
                opens_invariants_none();
                unsafe {
                    let mut m = MaybeUninit::new(in_v);
                    mem::swap(&mut m, &mut *self.uptr);
                    m.assume_init()
                }
            }
            #[doc =
              " Given a shared borrow of the `PermissionOpt<V>`, obtain a shared borrow of `V`."]
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn borrow<'a>(&self, perm: &'a Tracked<PermissionOpt<V>>)
             -> &'a V {
                ::builtin::requires([::builtin::equal(self.id(),
                                                      ((perm.view()).view()).pptr),
                                     ((perm.view()).view()).value.is_Some()]);
                ::builtin::ensures(|v: &'a V|
                                       [::builtin::equal(*v,
                                                         ((perm.view()).view()).value.get_Some_0())]);
                opens_invariants_none();
                unsafe { (*self.uptr).assume_init_ref() }
            }
            #[doc = " Free the memory pointed to be `perm`."]
            #[doc = " Requires the memory to be uninitialized."]
            #[doc = ""]
            #[doc =
              " This consumes `perm`, since it will no longer be safe to access"]
            #[doc = " that memory location."]
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn dispose(&self, perm: Tracked<PermissionOpt<V>>) {
                ::builtin::requires([::builtin::equal(self.id(),
                                                      ((perm.view()).view()).pptr),
                                     ::builtin::equal(((perm.view()).view()).value,
                                                      option::Option::None)]);
                opens_invariants_none();
                unsafe {
                    alloc::alloc::dealloc(self.uptr as *mut u8,
                                          alloc::alloc::Layout::for_value(&*self.uptr));
                }
            }
            #[doc = " Free the memory pointed to be `perm` and return the "]
            #[doc = " value that was previously there."]
            #[doc = " Requires the memory to be initialized."]
            #[doc =
              " This consumes the [`PermissionOpt`] token, since the user is giving up"]
            #[doc = " access to the memory by freeing it."]
            #[inline(always)]
            #[verifier(verus_macro)]
            pub fn into_inner(self, perm: Tracked<PermissionOpt<V>>) -> V {
                ::builtin::requires([::builtin::equal(self.id(),
                                                      ((perm.view()).view()).pptr),
                                     ((perm.view()).view()).value.is_Some()]);
                ::builtin::ensures(|v: V|
                                       [::builtin::equal(v,
                                                         ((perm.view()).view()).value.get_Some_0())]);
                opens_invariants_none();
                let mut perm = perm;
                let v = self.take(&mut perm);
                self.dispose(perm);
                v
            }
            #[doc =
              " Allocates heap memory for type `V`, leaving it initialized"]
            #[doc = " with the given value `v`."]
            #[inline(always)]
            #[verifier(verus_macro)]
            pub fn new(v: V) -> (PPtr<V>, Tracked<PermissionOpt<V>>) {
                ::builtin::ensures(|pt: (PPtr<V>, Tracked<PermissionOpt<V>>)|
                                       [(::builtin::equal(((pt.1.view()).view()),
                                                          PermissionOptData{pptr:
                                                                                pt.0.id(),
                                                                            value:
                                                                                option::Option::Some(v),}))]);
                let (p, mut t) = Self::empty();
                p.put(&mut t, v);
                (p, t)
            }
        }
    }
    #[cfg(not(feature = "no_global_allocator"))]
    pub mod ptr_old_style {
        use core::{marker, mem, mem::MaybeUninit};
        extern crate alloc;
        #[allow(unused_imports)]
        use builtin::*;
        #[allow(unused_imports)]
        use builtin_macros::*;
        #[allow(unused_imports)]
        use crate::pervasive::*;
        #[allow(unused_imports)]
        use crate::pervasive::modes::*;
        #[doc = " `PPtr<V>` (which stands for \"permissioned pointer\")"]
        #[doc = " is a wrapper around a raw pointer to `V` on the heap."]
        #[doc = ""]
        #[doc =
          " Technically, it is a wrapper around `*mut mem::MaybeUninit<V>`, that is, the object"]
        #[doc = " it points to may be uninitialized."]
        #[doc = ""]
        #[doc =
          " In order to access (read or write) the value behind the pointer, the user needs"]
        #[doc =
          " a special _ghost permission token_, [`PermissionOpt<V>`](PermissionOpt). This object is `tracked`,"]
        #[doc =
          " which means that it is \"only a proof construct\" that does not appear in code,"]
        #[doc =
          " but its uses _are_ checked by the borrow-checker. This ensures memory safety,"]
        #[doc = " data-race-freedom, prohibits use-after-free, etc."]
        #[doc = ""]
        #[doc = " ### PermissionOpt objects."]
        #[doc = ""]
        #[doc =
          " The [`PermissionOpt`] object represents both the ability to access the data behind the"]
        #[doc =
          " pointer _and_ the ability to free it (return it to the memory allocator)."]
        #[doc = ""]
        #[doc = " In particular:"]
        #[doc =
          "  * When the user owns a `PermissionOpt<V>` object associated to a given pointer,"]
        #[doc =
          "    they can either read or write its contents, or deallocate (\"free\") it."]
        #[doc =
          "  * When the user has a shared borrow, `&PermissionOpt<V>`, they can read"]
        #[doc = "    the contents (i.e., obtained a shared borrow `&V`)."]
        #[doc = ""]
        #[doc =
          " The `perm: PermissionOpt<V>` object tracks two pieces of data:"]
        #[doc =
          "  * `perm.pptr` is the pointer that the permission is associated to,"]
        #[doc = "     given by [`ptr.id()`](PPtr::id)."]
        #[doc =
          "  * `perm.value` tracks the data that is behind the pointer. Thereby:"]
        #[doc =
          "      * When the user uses the permission to _read_ a value, they always"]
        #[doc = "        read the value as given by the `perm.value`."]
        #[doc =
          "      * When the user uses the permission to _write_ a value, the `perm.value`"]
        #[doc = "        data is updated."]
        #[doc = ""]
        #[doc =
          " For those familiar with separation logic, the `PermissionOpt` object plays a role"]
        #[doc =
          " similar to that of the \"points-to\" operator, _ptr_ ↦ _value_."]
        #[doc = ""]
        #[doc = " ### Differences from `PCell`."]
        #[doc = ""]
        #[doc =
          " `PPtr` is similar to [`cell::PCell`], but has a few key differences:"]
        #[doc =
          "  * In `PCell<T>`, the type `T` is placed internally to the `PCell`, whereas with `PPtr`,"]
        #[doc = "    the type `T` is placed at some location on the heap."]
        #[doc =
          "  * Since `PPtr` is just a pointer (represented by an integer), it can be `Copy`."]
        #[doc =
          "  * The `ptr::PermissionOpt` token represents not just the permission to read/write"]
        #[doc = "    the contents, but also to deallocate."]
        #[doc = ""]
        #[doc = " ### Example (TODO)"]
        #[verifier(external_body)]
        pub struct PPtr<#[verifier(strictly_positive)] V> {
            uptr: *mut MaybeUninit<V>,
        }
        #[verifier(external)]
        unsafe impl <T> Sync for PPtr<T> { }
        #[verifier(external)]
        unsafe impl <T> Send for PPtr<T> { }
        #[doc =
          " A `tracked` ghost object that gives the user permission to dereference a pointer"]
        #[doc =
          " for reading or writing, or to free the memory at that pointer."]
        #[doc = ""]
        #[doc =
          " The meaning of a `PermissionOpt` object is given by the data in its"]
        #[doc = " `View` object, [`PermissionOptData`]."]
        #[doc = ""]
        #[doc = " See the [`PPtr`] documentation for more details."]
        #[verifier(external_body)]
        #[proof]
        pub struct PermissionOpt<#[verifier(strictly_positive)] V> {
            phantom: marker::PhantomData<V>,
            no_copy: NoCopy,
        }
        #[doc = " Represents the meaning of a [`PermissionOpt`] object."]
        #[spec]
        pub struct PermissionOptData<V> {
            #[doc =
              " Indicates that this token is for a pointer `ptr: PPtr<V>`"]
            #[doc = " such that [`ptr.id()`](PPtr::id) equal to this value."]
            pub pptr: int,
            #[doc =
              " Indicates that this token gives the ability to read a value `V` from memory."]
            #[doc =
              " When `None`, it indicates that the memory is uninitialized."]
            pub value: option::Option<V>,
        }
        #[doc(hidden)]
        #[macro_export]
        macro_rules! ptr_perm_internal {
            [$pcell : expr => $val : expr] =>
            {
                $crate :: pervasive :: ptr_old_style :: PermissionOptData
                { pptr : $pcell, value : $val, }
            } ;
        }
        #[macro_export]
        macro_rules! ptr_perm {
            [$($tail : tt) *] =>
            {
                :: builtin_macros :: verus_proof_macro_exprs!
                ($crate :: pervasive :: ptr_old_style :: ptr_perm_internal!
                 ($($tail) *))
            }
        }
        pub use ptr_perm_internal;
        pub use ptr_perm;
        impl <V> PermissionOpt<V> {
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn view(self) -> PermissionOptData<V> {
                ::core::panicking::panic("not implemented")
            }
            #[doc = " Any dereferenceable pointer must be non-null."]
            #[doc =
              " (Note that null pointers _do_ exist and are representable by `PPtr`;"]
            #[doc =
              " however, it is not possible to obtain a `PermissionOpt` token for"]
            #[doc = " any such a pointer.)"]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            #[proof]
            pub fn is_nonnull(#[proof] &self) {
                ::builtin::ensures([!::builtin::spec_eq(self.view().pptr,
                                                        ::builtin::spec_literal_nat("0"))]);
                ::core::panicking::panic("not implemented");
            }
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            #[proof]
            pub fn leak_contents(#[proof] &mut self) {
                ::builtin::ensures([::builtin::spec_eq(self.view().pptr,
                                                       old(self).view().pptr)
                                        && self.view().value.is_None()]);
                ::core::panicking::panic("not implemented");
            }
        }
        impl <V> PPtr<V> {
            #[doc = " Cast a pointer to an integer."]
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn to_usize(&self) -> usize {
                ::builtin::ensures(|u: usize|
                                       [::builtin::spec_eq(::builtin::spec_cast_integer::<_,
                                                                                          int>(u),
                                                           self.id())]);
                self.uptr as usize
            }
            #[doc = " integer address of the pointer"]
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn id(&self) -> int {
                ::core::panicking::panic("not implemented")
            }
            #[doc = " Cast an integer to a pointer."]
            #[doc = " "]
            #[doc =
              " Note that this does _not_ require or ensure that the pointer is valid."]
            #[doc =
              " Of course, if the user creates an invalid pointer, they would still not be able to"]
            #[doc =
              " create a valid [`PermissionOpt`] token for it, and thus they would never"]
            #[doc = " be able to access the data behind the pointer."]
            #[doc = ""]
            #[doc =
              " This is analogous to normal Rust, where casting to a pointer is always possible,"]
            #[doc = " but dereferencing a pointer is an `unsafe` operation."]
            #[doc =
              " In Verus, casting to a pointer is likewise always possible,"]
            #[doc =
              " while dereferencing it is only allowed when the right preconditions are met."]
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn from_usize(u: usize) -> Self {
                ::builtin::ensures(|p: Self|
                                       [::builtin::spec_eq(p.id(),
                                                           ::builtin::spec_cast_integer::<_,
                                                                                          int>(u))]);
                let uptr = u as *mut MaybeUninit<V>;
                PPtr{uptr,}
            }
            /// Allocates heap memory for type `V`, leaving it uninitialized.
            #[inline(always)]
            #[verifier(external_body)]
            pub fn empty() -> (PPtr<V>, Trk<PermissionOpt<V>>) {
                ensures(|pt: (PPtr<V>, Trk<PermissionOpt<V>>)|
                            equal(pt.1.0.view(),
                                  PermissionOptData{pptr: pt.0.id(),
                                                    value:
                                                        option::Option::None,}));
                opens_invariants_none();
                let p =
                    PPtr{uptr:
                             alloc::boxed::Box::leak(alloc::boxed::Box::new(MaybeUninit::uninit())).as_mut_ptr(),};
                let _exposed_addr = p.uptr as usize;
                (p, Trk(proof_from_false()))
            }
            #[doc = " Clones the pointer."]
            #[doc = " TODO implement the `Clone` and `Copy` traits"]
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn clone(&self) -> PPtr<V> {
                ::builtin::ensures(|pt: PPtr<V>|
                                       [::builtin::equal(pt, *self)]);
                opens_invariants_none();
                PPtr{uptr: self.uptr,}
            }
            /// Moves `v` into the location pointed to by the pointer `self`.
            /// Requires the memory to be uninitialized, and leaves it initialized.
            ///
            /// In the ghost perspective, this updates `perm.value`
            /// from `None` to `Some(v)`.
            #[inline(always)]
            #[verifier(external_body)]
            pub fn put(&self, #[proof] perm: &mut PermissionOpt<V>, v: V) {
                requires([self.id() == old(perm).view().pptr,
                          equal(old(perm).view().value,
                                option::Option::None)]);
                ensures([equal(perm.view().pptr, old(perm).view().pptr),
                         equal(perm.view().value, option::Option::Some(v))]);
                opens_invariants_none();
                unsafe { *(self.uptr) = MaybeUninit::new(v); }
            }
            /// Moves `v` out of the location pointed to by the pointer `self`
            /// and returns it.
            /// Requires the memory to be initialized, and leaves it uninitialized.
            ///
            /// In the ghost perspective, this updates `perm.view().value`
            /// from `Some(v)` to `None`,
            /// while returning the `v` as an `exec` value.
            #[inline(always)]
            #[verifier(external_body)]
            pub fn take(&self, #[proof] perm: &mut PermissionOpt<V>) -> V {
                requires([self.id() == old(perm).view().pptr,
                          old(perm).view().value.is_Some()]);
                ensures(|v: V|
                            [perm.view().pptr == old(perm).view().pptr,
                             equal(perm.view().value, option::Option::None),
                             equal(v, old(perm).view().value.get_Some_0())]);
                opens_invariants_none();
                unsafe {
                    let mut m = MaybeUninit::uninit();
                    mem::swap(&mut m, &mut *self.uptr);
                    m.assume_init()
                }
            }
            #[doc =
              " Swaps the `in_v: V` passed in as an argument with the value in memory."]
            #[doc =
              " Requires the memory to be initialized, and leaves it initialized with the new value."]
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn replace(&self, #[proof] perm: &mut PermissionOpt<V>,
                           in_v: V) -> V {
                ::builtin::requires([::builtin::equal(self.id(),
                                                      old(perm).view().pptr),
                                     old(perm).view().value.is_Some()]);
                ::builtin::ensures(|out_v: V|
                                       [::builtin::equal(perm.view().pptr,
                                                         old(perm).view().pptr),
                                        ::builtin::equal(perm.view().value,
                                                         option::Option::Some(in_v)),
                                        ::builtin::equal(out_v,
                                                         old(perm).view().value.get_Some_0())]);
                opens_invariants_none();
                unsafe {
                    let mut m = MaybeUninit::new(in_v);
                    mem::swap(&mut m, &mut *self.uptr);
                    m.assume_init()
                }
            }
            /// Given a shared borrow of the `PermissionOpt<V>`, obtain a shared borrow of `V`.
            #[inline(always)]
            #[verifier(external_body)]
            pub fn borrow<'a>(&self, #[proof] perm: &'a PermissionOpt<V>)
             -> &'a V {
                requires([equal(self.id(), perm.view().pptr),
                          perm.view().value.is_Some()]);
                ensures(|v: &V| equal(*v, perm.view().value.get_Some_0()));
                opens_invariants_none();
                unsafe { (*self.uptr).assume_init_ref() }
            }
            #[doc = " Free the memory pointed to be `perm`."]
            #[doc = " Requires the memory to be uninitialized."]
            #[doc = ""]
            #[doc =
              " This consumes `perm`, since it will no longer be safe to access"]
            #[doc = " that memory location."]
            #[inline(always)]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn dispose(&self, #[proof] perm: PermissionOpt<V>) {
                ::builtin::requires([::builtin::equal(self.id(),
                                                      perm.view().pptr),
                                     ::builtin::equal(perm.view().value,
                                                      option::Option::None)]);
                opens_invariants_none();
                unsafe {
                    alloc::alloc::dealloc(self.uptr as *mut u8,
                                          alloc::alloc::Layout::for_value(&*self.uptr));
                }
            }
            /// Free the memory pointed to be `perm` and return the 
            /// value that was previously there.
            /// Requires the memory to be initialized.
            /// This consumes the [`PermissionOpt`] token, since the user is giving up
            /// access to the memory by freeing it.
            #[inline(always)]
            pub fn into_inner(self, #[proof] perm: PermissionOpt<V>) -> V {
                requires([equal(self.id(), perm.view().pptr),
                          perm.view().value.is_Some()]);
                ensures(|v: V| [equal(v, perm.view().value.get_Some_0())]);
                opens_invariants_none();
                #[proof]
                let mut perm = perm;
                let v = self.take(&mut perm);
                self.dispose(perm);
                v
            }
            #[doc =
              " Allocates heap memory for type `V`, leaving it initialized"]
            #[doc = " with the given value `v`."]
            #[inline(always)]
            #[verifier(verus_macro)]
            pub fn new(v: V) -> (PPtr<V>, Trk<PermissionOpt<V>>) {
                ::builtin::ensures(|pt: (PPtr<V>, Trk<PermissionOpt<V>>)|
                                       [(::builtin::equal(pt.1.0.view(),
                                                          PermissionOptData{pptr:
                                                                                pt.0.id(),
                                                                            value:
                                                                                option::Option::Some(v),}))]);
                let (p, Trk(mut t)) = Self::empty();
                p.put(&mut t, v);
                (p, Trk(t))
            }
        }
        impl <V: Copy> PPtr<V> {
            #[inline(always)]
            pub fn read(&self, #[proof] perm: &PermissionOpt<V>) -> V {
                requires([equal(self.id(), perm.view().pptr),
                          perm.view().value.is_Some()]);
                ensures(|v: V|
                            equal(option::Option::Some(v),
                                  perm.view().value));
                *self.borrow(perm)
            }
            #[inline(always)]
            #[exec]
            pub fn write(&self, #[proof] perm: &mut PermissionOpt<V>, v: V) {
                requires(equal(self.id(), old(perm).view().pptr));
                ensures([equal(perm.view().pptr, self.id()),
                         equal(perm.view().value, option::Option::Some(v))]);
                perm.leak_contents();
                self.put(perm, v);
            }
            #[inline(always)]
            pub fn free(&self, #[proof] perm: PermissionOpt<V>) {
                requires(equal(self.id(), perm.view().pptr));
                #[proof]
                let mut perm = perm;
                perm.leak_contents();
                self.dispose(perm);
            }
        }
    }
    #[cfg(not(feature = "no_global_allocator"))]
    pub mod string {
        #![feature(rustc_attrs)]
        extern crate alloc;
        use alloc::string;
        #[allow(unused_imports)]
        use super::seq::Seq;
        use super::vec::Vec;
        #[allow(unused_imports)]
        use builtin::*;
        use builtin_macros::verus;
        #[verifier(external_body)]
        pub struct String {
            inner: string::String,
        }
        #[rustc_diagnostic_item = "pervasive::string::StrSlice"]
        #[verifier(external_body)]
        pub struct StrSlice<'a> {
            inner: &'a str,
        }
        #[rustc_diagnostic_item = "pervasive::string::new_strlit"]
        #[verifier(external_body)]
        #[verifier(verus_macro)]
        pub const fn new_strlit<'a>(s: &'a str) -> StrSlice<'a> {
            StrSlice{inner: s,}
        }
        impl <'a> StrSlice<'a> {
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn view(&self) -> Seq<char> {
                ::core::panicking::panic("not implemented")
            }
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn is_ascii(&self) -> bool {
                ::core::panicking::panic("not implemented")
            }
            #[doc = " The len() function in rust returns the byte length."]
            #[doc =
              " It is more useful to talk about the length of characters and therefore this function was added."]
            #[doc =
              " Please note that this function counts the unicode variation selectors as characters."]
            #[doc = " Warning: O(n)"]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn unicode_len(&self) -> usize {
                ::builtin::ensures(|l: usize|
                                       [::builtin::equal(::builtin::spec_cast_integer::<_,
                                                                                        nat>(l),
                                                         (self.view()).len())]);
                self.inner.chars().count()
            }
            #[doc = " Warning: O(n) not O(1) due to unicode decoding needed"]
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn get_char(&self, i: usize) -> char {
                ::builtin::requires([(i).spec_lt((self.view()).len())]);
                ::builtin::ensures(|c: char|
                                       [::builtin::equal((self.view()).index(::builtin::spec_cast_integer::<_,
                                                                                                            int>(i)),
                                                         c),
                                        ::builtin::imply(self.is_ascii(),
                                                         ::builtin::forall(|i:
                                                                                int|
                                                                               ::builtin::imply((i).spec_lt((self.view()).len()),
                                                                                                ((::builtin::spec_cast_integer::<_,
                                                                                                                                 nat>((self.view()).index(i)))).spec_lt(::builtin::spec_literal_nat("256")))))]);
                self.inner.chars().nth(i).unwrap()
            }
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn substring_ascii(&self, from: usize, to: usize)
             -> StrSlice<'a> {
                ::builtin::requires([self.is_ascii(),
                                     (from).spec_lt((self.view()).len()),
                                     (to).spec_le((self.view()).len())]);
                ::builtin::ensures(|ret: StrSlice<'a>|
                                       [::builtin::equal((ret.view()),
                                                         (self.view()).subrange(::builtin::spec_cast_integer::<_,
                                                                                                               int>(from),
                                                                                ::builtin::spec_cast_integer::<_,
                                                                                                               int>(to))),
                                        ::builtin::equal(ret.is_ascii(),
                                                         self.is_ascii())]);
                StrSlice{inner: &self.inner[from..to],}
            }
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn substring_char(&self, from: usize, to: usize)
             -> StrSlice<'a> {
                ::builtin::requires([(from).spec_lt((self.view()).len()),
                                     (to).spec_le((self.view()).len())]);
                ::builtin::ensures(|ret: StrSlice<'a>|
                                       [::builtin::equal((ret.view()),
                                                         (self.view()).subrange(::builtin::spec_cast_integer::<_,
                                                                                                               int>(from),
                                                                                ::builtin::spec_cast_integer::<_,
                                                                                                               int>(to))),
                                        ::builtin::equal(ret.is_ascii(),
                                                         self.is_ascii())]);
                let mut char_pos = 0;
                let mut byte_start = None;
                let mut byte_end = None;
                let mut byte_pos = 0;
                let mut it = self.inner.chars();
                loop {
                    if char_pos == from { byte_start = Some(byte_pos); }
                    if char_pos == to { byte_end = Some(byte_pos); break ; }
                    if let Some(c) = it.next() {
                        char_pos += 1;
                        byte_pos += c.len_utf8();
                    } else { break ; }
                }
                let byte_start = byte_start.unwrap();
                let byte_end = byte_end.unwrap();
                StrSlice{inner: &self.inner[byte_start..byte_end],}
            }
            #[verifier(verus_macro)]
            pub fn to_string(self) -> String {
                ::builtin::ensures(|ret: String|
                                       [::builtin::equal((self.view()),
                                                         (ret.view())),
                                        ::builtin::equal(self.is_ascii(),
                                                         ret.is_ascii())]);
                String::from_str(self)
            }
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn get_ascii(&self, i: usize) -> u8 {
                ::builtin::requires([self.is_ascii()]);
                ::builtin::ensures(|b: u8|
                                       [::builtin::equal(::builtin::spec_cast_integer::<_,
                                                                                        u8>(self.view().index(::builtin::spec_cast_integer::<_,
                                                                                                                                             int>(i))),
                                                         b)]);
                self.inner.as_bytes()[i]
            }
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn as_bytes(&self) -> Vec<u8> {
                ::builtin::requires([self.is_ascii()]);
                ::builtin::ensures(|ret: Vec<u8>|
                                       [::builtin::equal(ret.view(),
                                                         Seq::new(self.view().len(),
                                                                  ::builtin::closure_to_fn_spec(|i|
                                                                                                    ::builtin::spec_cast_integer::<_,
                                                                                                                                   u8>(self.view().index(i)))))]);
                let mut v = Vec::new();
                for c in self.inner.as_bytes().iter() { v.push(*c); }
                v
            }
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_str_literal_is_ascii<'a>(s: StrSlice<'a>) {
            ::builtin::ensures([::builtin::equal(#[trigger] s.is_ascii(),
                                                 builtin::strslice_is_ascii(s))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_str_literal_len<'a>(s: StrSlice<'a>) {
            ::builtin::ensures([::builtin::equal(#[trigger] (s.view()).len(),
                                                 builtin::strslice_len(s))]);
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_str_literal_get_char<'a>(s: StrSlice<'a>, i: int) {
            ::builtin::ensures([::builtin::equal(#[trigger] (s.view()).index(i),
                                                 builtin::strslice_get_char(s,
                                                                            i))]);
        }
        impl String {
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn view(&self) -> Seq<char> {
                ::core::panicking::panic("not implemented")
            }
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn is_ascii(&self) -> bool {
                ::core::panicking::panic("not implemented")
            }
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn from_str<'a>(s: StrSlice<'a>) -> String {
                ::builtin::ensures(|ret: String|
                                       [::builtin::equal((s.view()),
                                                         (ret.view())),
                                        ::builtin::equal(s.is_ascii(),
                                                         ret.is_ascii())]);
                String{inner: s.inner.to_string(),}
            }
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn as_str<'a>(&'a self) -> StrSlice<'a> {
                ::builtin::ensures(|ret: StrSlice<'a>|
                                       [::builtin::equal((self.view()),
                                                         (ret.view())),
                                        ::builtin::equal(self.is_ascii(),
                                                         ret.is_ascii())]);
                let inner = self.inner.as_str();
                StrSlice{inner,}
            }
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn append<'a, 'b>(&'a mut self, other: StrSlice<'b>) {
                ::builtin::ensures([::builtin::equal((self.view()),
                                                     ((old(self).view())).spec_add((other.view()))),
                                    ::builtin::equal(self.is_ascii(),
                                                     old(self).is_ascii()) &&
                                        other.is_ascii()]);
                self.inner += other.inner;
            }
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn concat<'b>(self, other: StrSlice<'b>) -> String {
                ::builtin::ensures(|ret: String|
                                       [::builtin::equal((ret.view()),
                                                         ((self.view())).spec_add((other.view()))),
                                        ::builtin::equal(ret.is_ascii(),
                                                         self.is_ascii()) &&
                                            other.is_ascii()]);
                String{inner: self.inner + other.inner,}
            }
        }
    }
    #[cfg(not(feature = "no_global_allocator"))]
    pub mod vec {
        #[allow(unused_imports)]
        use builtin::*;
        #[allow(unused_imports)]
        use builtin_macros::*;
        #[allow(unused_imports)]
        use crate::pervasive::*;
        #[allow(unused_imports)]
        use crate::pervasive::seq::*;
        extern crate alloc;
        use alloc::vec;
        #[verifier(external_body)]
        pub struct Vec<#[verifier(strictly_positive)] A> {
            pub vec: vec::Vec<A>,
        }
        impl <A> Vec<A> {
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn view(&self) -> Seq<A> {
                ::core::panicking::panic("not implemented")
            }
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn new() -> Self {
                ::builtin::ensures(|v: Self|
                                       [::builtin::equal((v.view()),
                                                         Seq::empty())]);
                Vec{vec: vec::Vec::new(),}
            }
            #[verifier(verus_macro)]
            pub fn empty() -> Self {
                ::builtin::ensures(|v: Self|
                                       [::builtin::equal((v.view()),
                                                         Seq::empty())]);
                Vec::new()
            }
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn push(&mut self, value: A) {
                ::builtin::ensures([::builtin::equal((self.view()),
                                                     (old(self).view()).push(value))]);
                self.vec.push(value);
            }
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn pop(&mut self) -> A {
                ::builtin::requires([(old(self).len()).spec_gt(::builtin::spec_literal_nat("0"))]);
                ::builtin::ensures(|value: A|
                                       [::builtin::equal(value,
                                                         old(self).spec_index((old(self).len()).spec_sub(::builtin::spec_literal_nat("1")))),
                                        ::builtin::equal((self.view()),
                                                         (old(self).view()).subrange(::builtin::spec_literal_integer("0"),
                                                                                     (old(self).len()).spec_sub(::builtin::spec_literal_nat("1"))))]);
                unsafe { self.vec.pop().unwrap_unchecked() }
            }
            #[verifier(inline)]
            #[verifier(verus_macro)]
            #[verifier(publish)]
            #[spec]
            pub fn spec_index(self, i: int) -> A {
                (self.view()).spec_index(i)
            }
            #[verifier(external_body)]
            #[verifier(autoview)]
            #[verifier(verus_macro)]
            pub fn index(&self, i: usize) -> &A {
                ::builtin::requires([(i).spec_lt(self.len())]);
                ::builtin::ensures(|r: &A|
                                       [::builtin::equal(*r,
                                                         self.spec_index(::builtin::spec_cast_integer::<_,
                                                                                                        int>(i)))]);
                &self.vec[i]
            }
            #[verifier(external_body)]
            #[verifier(verus_macro)]
            pub fn set(&mut self, i: usize, a: A) {
                ::builtin::requires([(i).spec_lt(old(self).len())]);
                ::builtin::ensures([::builtin::equal((self.view()),
                                                     (old(self).view()).update(::builtin::spec_cast_integer::<_,
                                                                                                              int>(i),
                                                                               a))]);
                self.vec[i] = a;
            }
            #[verifier(verus_macro)]
            #[spec]
            #[verifier(external_body)]
            pub fn spec_len(&self) -> usize {
                ::core::panicking::panic("not implemented")
            }
            #[verifier(external_body)]
            #[verifier(when_used_as_spec(spec_len))]
            #[verifier(autoview)]
            #[verifier(verus_macro)]
            pub fn len(&self) -> usize {
                ::builtin::ensures(|l: usize|
                                       [::builtin::spec_eq(l, self.len())]);
                self.vec.len()
            }
        }
        #[verifier(external_body)]
        #[verifier(broadcast_forall)]
        #[verifier(verus_macro)]
        #[proof]
        pub fn axiom_spec_len<A>(v: Vec<A>) {
            ::builtin::ensures([::builtin::spec_eq(#[trigger] v.spec_len(),
                                                   v.view().len())]);
        }
    }
    #[allow(unused_imports)]
    use builtin::*;
    #[proof]
    pub fn assume(b: bool) { ensures(b); admit(); }
    #[proof]
    #[verifier(custom_req_err("assertion failure"))]
    pub fn assert(b: bool) { requires(b); ensures(b); }
    #[proof]
    pub fn affirm(b: bool) { requires(b); }
    /// A tool to check one's reasoning while writing complex spec functions.
    /// Not intended to be used as a mechanism for instantiating quantifiers, `spec_affirm` should
    /// be removed from spec functions once they are complete.
    ///
    /// ## Example
    ///
    /// ```rust
    /// #[spec(checked)] fn some_predicate(a: nat) -> bool {
    ///     recommends(a < 100);
    ///     if (a >= 50) {
    ///         let _ = spec_affirm(50 <= a && a < 100);
    ///         a >= 75
    ///     } else {
    ///         let _ = spec_affirm(a < 50);
    ///         // let _ = spec_affirm(a < 40); would raise a recommends note here
    ///         a < 25
    ///     }
    /// }
    /// ```
    #[spec]
    pub fn spec_affirm(b: bool) -> bool { recommends(b); b }
    /// In spec, all types are inhabited
    #[spec]
    #[verifier(external_body)]
    #[allow(dead_code)]
    pub fn arbitrary<A>() -> A { ::core::panicking::panic("not implemented") }
    #[proof]
    #[verifier(returns(proof))]
    #[verifier(external_body)]
    #[allow(dead_code)]
    pub fn proof_from_false<A>() -> A {
        requires(false);
        ::core::panicking::panic("not implemented")
    }
    #[verifier(external_body)]
    #[allow(dead_code)]
    pub fn unreached<A>() -> A {
        requires(false);
        { ::std::rt::begin_panic("unreached_external") }
    }
    #[verifier(external_body)]
    pub fn print_u64(i: u64) {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(&["", "\n"],
                                                             &match (&i,) {
                                                                  _args =>
                                                                  [::core::fmt::ArgumentV1::new(_args.0,
                                                                                                ::core::fmt::Display::fmt)],
                                                              }));
        };
    }
    /// Allows you to prove a boolean predicate by assuming its negation and proving
    /// a contradiction.
    ///
    /// `assert_by_contradiction!(b, { /* proof */ });`
    /// Equivalent to writing `if !b { /* proof */; assert(false); }`
    /// but is more concise and documents intent.
    ///
    /// ```rust
    /// assert_by_contradiction!(b, {
    ///     // assume !b here
    ///     // prove `false`
    /// });
    /// ```
    #[macro_export]
    macro_rules! assert_by_contradiction {
        ($($a : tt) *) =>
        {
            verus_proof_macro_exprs!
            (assert_by_contradiction_internal! ($($a) *))
        }
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! assert_by_contradiction_internal {
        ($predicate : expr, $bblock : block) =>
        {
            :: builtin ::
            assert_by($predicate,
                      {
                          if! $predicate
                          { $bblock crate :: pervasive :: assert(false) ; }
                      }) ;
        }
    }
