use summary::Summary;
use summary::NewsArticle;

fn main() {
    let article = NewsArticle {
        // ペンギンチームがスタンレーカップチャンピオンシップを勝ち取る！
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        // ピッツバーグ、ペンシルベニア州、アメリカ
        location: String::from("Pittsburgh, PA, USA"),
        // アイスバーグ
        author: String::from("Iceburgh"),
        // ピッツバーグ・ペンギンが再度NHL(National Hockey League)で最強のホッケーチームになった
        content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
    };

    // 新しい記事が利用可能です！ {}
    println!("New article available! {}", article.summarize());
}