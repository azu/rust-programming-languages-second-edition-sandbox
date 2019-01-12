extern crate oop;

use oop::AveragedCollection;

fn main() {
    // フィールドが非公開なので、この方法では作れない
//    let collection = AveragedCollection {
//        average: 10.0,
//        list: vec![1, 10, 20],
//    };
    let collection = AveragedCollection::new(vec![1, 10, 20]);
    println!("average {}", collection.average());
}
