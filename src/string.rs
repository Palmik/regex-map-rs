/// Associative container where the keys are regular expressions, based on the `regex::RegexSet` data structure.
pub struct RegexMap<V> {
    set: regex::RegexSet,
    values: Vec<V>,
}

impl<V> RegexMap<V> {
    /// Create a new `RegexMap` from iterator over (expression, value) pairs, where the expression is `&str`-like.
    ///
    /// ```
    /// use regex_map::RegexMap;
    ///
    /// let map = RegexMap::new([
    ///    ("foo", 1),
    ///    ("bar", 2),
    ///    ("foobar", 3),
    ///    ("^foo$", 4),
    ///    ("^bar$", 5),
    ///    ("^foobar$", 6),
    /// ]);
    ///
    /// assert_eq!(map.get("foo").cloned().collect::<Vec<_>>(), vec![1, 4]);
    /// assert_eq!(map.get("bar").cloned().collect::<Vec<_>>(), vec![2, 5], );
    /// assert_eq!(map.get("foobar").cloned().collect::<Vec<_>>(), vec![1, 2, 3, 6]);
    /// assert_eq!(map.get("XXX foo XXX").cloned().collect::<Vec<_>>(), vec![1]);
    /// assert_eq!(map.get("XXX bar XXX").cloned().collect::<Vec<_>>(), vec![2]);
    /// ```
    pub fn new<I, S>(items: I) -> Self
    where
        I: IntoIterator<Item = (S, V)>,
        S: AsRef<str>,
    {
        let mut exprs = Vec::new();
        let mut values = Vec::new();
        for (expr, value) in items {
            exprs.push(expr);
            values.push(value);
        }

        let set = regex::RegexSet::new(exprs).unwrap();
        RegexMap { set, values }
    }

    /// Get an iterator over all values whose regular expression matches the given key.
    ///
    /// To get first matching value, use can use `.next()` on the returned iterator:
    ///
    /// ```
    /// use regex_map::RegexMap;
    ///
    /// let map = RegexMap::new([
    ///    ("foo", 1),
    ///    ("bar", 2),
    /// ]);
    ///
    /// assert_eq!(map.get("foo").next(), Some(&1));
    /// ```
    pub fn get(&self, key: &str) -> impl Iterator<Item = &V> {
        self.set
            .matches(key)
            .into_iter()
            .map(move |i| &self.values[i])
    }

    /// Check if the given key matches any of the regular expressions.
    pub fn contains_key(&self, key: &str) -> bool {
        self.set.is_match(key)
    }
}
