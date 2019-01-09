fn main() {
    #[derive(Debug)]
    enum IpAddrKind {
        // enumはそれぞれにTuple的なデータ型を指定できる
        // 構造体はできないらしい
        V4(u8, u8, u8, u8),
        V6(String),
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4(127, 0, 0, 1),
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6(String::from("::1")),
        address: String::from("::1"),
    };
    println!("home {:?}", home);
    println!("loopback {:?}", loopback);
    // enum
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    // 構造体

    struct QuitMessage; // ユニット構造体
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // タプル構造体
    struct ChangeColorMessage(i32, i32, i32); // タプル構造体
}
