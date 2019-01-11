fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("{}", val);
    }
}

#[test]
fn iterator_sum() {
    let v = vec![1, 2, 3, 4];
    let mut iter = v.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
}

#[test]
fn iterator_consum_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // consuming adaptor はiteratorを消費する = 所有権を奪う
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}
// iterator adaptor

#[test]
fn iterator_adapter() {
    let v = vec![1, 2, 3];
    let v2: Vec<i32> = v.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);

    assert_eq!(v2, vec![2, 3, 4])
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect() // consumeしないといけない
}


#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}

// 独自Iterator
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            return Some(self.count);
        } else {
            return None;
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}