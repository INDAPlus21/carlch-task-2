# Questions
### Method Chaining
`let input = io::stdin();

let mut lines = input
    .lock()
    .lines()
    .map(|_line| _line.ok().unwrap().to_string());

// for every line, assuming input strings with the characters '0' and '1' seperated by whitelines

let binary_line = lines
    .next().unwrap()
    .split(" ")
    .map(|_title| {
        _title
            .chars()
            .map(|_character| {
                match _character {
                    '0' => false,
                    _ => true
                }
            })
            .collect::<Vec<bool>>()
    })
    .collect::<Vec<Vec<bool>>>();
`

The value of `binary_line` is boolean `true` if input is `1` or `false` if input is `0`.

### Iterables
`use std::collections::{ HashMap, HashSet };

/*...*/

let limit: usize = 10;

let mut index_store: HashMap<usize, usize> = HashMap::with_capacity(limit + 1);
let mut value_store: Vec<HashSet<usize>> = Vec::with_capacity(limit + 1);
        
for _value in 1..(limit + 1) {
    index_store.insert(_value, _value - 1);

    let mut sequence: HashSet<usize> = HashSet::new();
    sequence.insert(_value);

    value_store.push(sequence);
}`

The value of `index_store` is:
Key     Value
 1       0
 2       1
 3       2
 4       3
 5       4
 6       5
 7       6
 8       7
 9       8
 10      9

The value of `value_store` is:
  {1}
  {2}
  {3}
  {4}
  {5}
  {6}
  {7}
  {8}
  {9}
  {10}

