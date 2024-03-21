// enum IpAddrKind {
//     V4, 
//     V6,
// }

// struct IpAddr {
//     kind : IpAddrKind,
//     address : String,
// }

// let home = IpAddr {
//     kind : IpAddrKind ::V4,
//     address : String::from("127.0.0.1"),
// }

// let loopback = IpAddr {
//     kind : IpAddrKind::V6, 
//     address : String::from("::1")
// }


// enum Message {
//     Quit, 
//     Move { x : i32, y: i32},
//     Write(String),
//     ChangeColor(i32,i32,i32),
// }

// struct QuitMessage; // 유닛 구조체
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // 튜플 구조체
// struct ChangeColorMessage(i32, i32, i32); // 튜플 구조체

enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1, 
        Coin::Nickel => 5, 
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}