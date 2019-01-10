use std::collections::HashMap;

fn main() {
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        let yellow = scores.get("Yellow");
        println!("{}", yellow.unwrap());
    }
    {
        let teams = vec!["Blue", "Yellow"];
        let initial_scores = vec![10, 50];
        let score: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        println!("{:?}", score);
    }
    // ハッシュマップと所有権
    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        // この時点で所有権はmapにmoveされる
        map.insert(field_name, field_value);
        // field_nameには参照できなくなる
        // borrowすればok &
    }
    // get
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
        println!("score {}", score.unwrap());
    }
    // iterate
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }
    // if ないならinsert
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
        println!("{:?}", scores);
    }
    // insertした値を更新するパターン
    {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            // insertした値を返す
            // entryでif exist and insert
            let inserted_value = map.entry(word).or_insert(0);
            *inserted_value += 1;
        }
        println!("{:?}", map);
    }
    //
    {}
}
