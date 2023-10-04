Associative container where the keys are regular expressions, based on the `regex::RegexSet` data structure.

## Example use:

```rs
use regex_map::RegexMap;

let map = RegexMap::new([
   ("foo", 1),
   ("bar", 2),
   ("foobar", 3),
   ("^foo$", 4),
   ("^bar$", 5),
   ("^foobar$", 6),
]);

assert_eq!(map.get("foo").cloned().collect::<Vec<_>>(), vec![1, 4]);
assert_eq!(map.get("bar").cloned().collect::<Vec<_>>(), vec![2, 5], );
assert_eq!(map.get("foobar").cloned().collect::<Vec<_>>(), vec![1, 2, 3, 6]);
assert_eq!(map.get("XXX foo XXX").cloned().collect::<Vec<_>>(), vec![1]);
assert_eq!(map.get("XXX bar XXX").cloned().collect::<Vec<_>>(), vec![2]);

for value in map.get("foo") {
    println!("Foo match: {}", value);
}
```

## TODOs:

- Consider adding `get_with_match`, which would return iterator over the values and the matches for the individual regexes.
