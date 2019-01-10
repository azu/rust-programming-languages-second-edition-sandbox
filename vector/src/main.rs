fn main() {
    {
        let v: Vec<i32> = Vec::new();
        println!("{:?}", v);
    }
    {
        let v = vec![1, 2, 3];
        println!("{:?}", v);
    }
    {
        let mut vec = vec![];
        vec.push(1);
        vec.push(2);
        println!("{:?}", vec);
    }
    // read
    {
        let v = vec![1, 2, 3, 4, 5];

        // インデックスで取得
        // &で借用してるのは、moveしないようにするため
        let third = &v[3];
        println!("{:?}", third);
        // getはOptionを返す
        let third: Option<&i32> = v.get(333);
        match third {
            Some(value) => {
                println!("{}", value);
            }
            None => {
                println!("Not found");
            }
        }
    }
    // iterate
    {
        let v = vec![100, 32, 4];
        // &をしてるのはmoveしないようにするため
        // readのみの場合は&をつけるイメージ
        for i in &v {
            println!("{}", i);
        }
        println!("v is {:?}", v)
    }
    {
        let mut v = vec![100, 32, 57];
        for item in &mut v {
            *item += 100
        }
        println!("items {:?}", v);
    }
    // enum
    {
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
        println!("{:?}", row)
    }
}
