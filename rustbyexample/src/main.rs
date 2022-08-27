use std::fmt::{Display, Formatter};
use std::future::pending;

use crate::List::{Cons, Nil};

fn fmt() {
    println!("{:#?}", 1);//{} pretty Display
    println!("{:?}", 1);//?Debug
    println!("{:#x}", 32);//0x20
    println!("{:.2}", 3.14159);//3.14
    println!("{:04}", 12);//0012
    println!("{:0>4}", 12);//0012(==:04)
    println!("{:0<4}", 12);//1200
    //format!()类似
    let s = format!("{:#x}", 0x300);
    let s = format!(r#"hello {}"#, "not ok");//str should not contain {}
    println!("{}", s);
    //不支持自定义打印 可以使用
    // #[derive(Debug)]
    // struct DebugPrintable(i32);
    let list = vec![1, 2, 3];
    println!("{:?}", list);
}

fn std_types() {
    let a = 1;
    let b = true;
    let c = 1u32;
    let d = 1_000u32;
    let d = ();
    let e = (1, 2, 3);
}

fn array_slice() {
    let list = [1, 2, 3];
    let a: [i32; 3] = [0; 3];//init all equal 0
    println!("{:?}", a);
    println!("{:?}", &a[1..2]);
}

fn vec() -> Vec<i32> {
    //Copy是隐式的、廉价的，并且不能重新实现（memcpy）。
    // Clone是显式的，可能很昂贵，并且可以任意重新实现。
    let mut a = vec![];
    a.push(1);
    a.push(2);
    a.push(3);
    println!("{:?}", &a[0..1]);
    let c = (&a[2..]).to_owned();
    println!("{:?}", c);
    println!("{:?}", c);

    return c;
}

#[derive(Debug)]
struct Conf {
    id: i32,
    name: &'static str,
    url: String,
}

impl Conf {
    fn hi(&self) {
        println!("hello from {:?}", self.name);
    }
}

impl Display for Conf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "name {} url {}", self.name, self.url)
    }
}

fn struct_demo() {
    let c = Conf {
        id: 1,
        name: "a",
        url: "b".to_string(),
    };
    c.hi();
    println!("{:?}", c);
    println!("{}", c.to_string());
    println!("{}", c);
}

#[derive(Debug)]
enum Event {
    Click { x: i32, y: i32 },
    Paste(String),
    KeyPress(char),
    PageLoad,
}

fn enum_demo() {
    let a = Event::Click { x: 3, y: 2 };
    // let a = Event::Paste("hello".to_string());
    // let a = Event::KeyPress('a');
    // let a = Event::PageLoad;
    match a {
        Event::Click { x, y } => {
            println!("{:?}{:?}", x, y);
        }
        Event::Paste(s) => {
            println!("{:?}", s);
        }
        Event::PageLoad => {
            println!("{:?}", "load");
        }
        Event::KeyPress(x) => {
            println!("{:?}", x);
        }
    }
}


type EventAlias = Event;

fn enum2() {
    let a = EventAlias::PageLoad;
    println!("{:?}", a);
}

fn use_demo() {
    use Event::{PageLoad, KeyPress};
    let a = PageLoad;
    let a = KeyPress('a');
    println!("{:?}", a);
}

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Blue = 0x0000ff,
}

//链表
#[derive(Debug)]
enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }
    fn append(self, v: u32) -> List {
        Cons(v, Box::new(self))
    }
    fn len(&self) -> i32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
}

fn linked_list_demo() {
    let mut list = List::new();
    list = list.append(1);
    list = list.append(1);
    list = list.append(1);
    println!("{:?}", list.len());
    println!("{:?}", list);
}

// 常量
const aa: i32 = 1;
static bb: &str = "hello";

//默认static
fn const_demo() {
    println!("{:?}", aa);
    println!("{:?}", bb);
}

fn variable() {
    {
        let a = "a";
    }
    println!("{:?}", "a");
    let a;
    // println!("{:?}", a);//err
    a = 1;
    println!("{:?}", a);
}

fn cast_demo() {
    let a = 1.1;
    let b = a as i32;//copy
    println!("{:?}", b);
    println!("{:?}", b as u64);
    println!("{:?}", a);

    //字符串转int
    let c: i32 = "122".parse().unwrap();
    println!("{:?}", c);
    println!("{:?}", format!("{}", c));
}

fn size_of() {
    let a = 1;
    println!("{:?}", std::mem::size_of_val(&a));
    println!("{:?}", std::mem::size_of::<i64>());
}

fn from_demo() {
    let a = "hello";
    let b = String::from(a);//copy
    println!("{:?}", a);
    println!("{:?}", b);
    let c = &b.into_boxed_str();//move
    println!("{:?}", c);
    // println!("{:?}", &b);//err
    let d: i32 = 1.try_into().unwrap();
    println!("{:?}", d);
}

fn if_while() {
    let mut a = 1;
    if a > 0 {
        println!("{:?}", a);
    } else if a > 2 {
        println!("{:?}", a);
    }
    while a < 10 {
        println!("{:?}", a);
        a = a + 1;
    }
    println!("{:?}", a);
    loop {
        if a > 20 {
            break;
        }
        a = a + 1;
    }
    let b = loop {
        break a;
    };
    println!("{:?}", b);
}

fn for_demo() {
    for i in 0..2 {
        println!("{:?}", i);
    }
    let a = vec![1, 3, 3];
    // 默认使用into_iter，会move集合中的元素
    for item in a {
        println!("{:?}", item);
    }
    //println!("{:?}", a);//err
    let a = vec![1, 2];
    // iter是借用元素
    for i in a.iter() {
        println!("{:?}", i);
    }
}

fn match_demo() {
    //normal
    let a = 13;
    match a {
        1 => println!("{:?}", 1),
        2 | 4 => println!("{:?}", 2),
        5..=10 => println!("{:?}", 5),
        _ => println!("{:?}", "None")
    }
    //enum
    //tuple
    let t = (1, 23, -1);
    match t {
        // (1, y, z) => {
        //     println!("{:?}", y);
        // }
        // (_, 23, -1) => {
        //     println!("{:?}", "_");
        // }
        (x, y, z) => {
            println!("{:?}", x);
        }
    }
    //struct
}

fn match_ptr() {
    //引用类型匹配
    let r = &4;
    // 匹配引用
    match r {
        &val => println!("{:?}", val)
    }
    // 匹配值（先解引用）
    match *r {
        val => println!("{:?}", val),
    }

    //正常类型匹配
    let v = 5;
    match v {
        //匹配上再引用 == let ref r=5;
        ref r => println!("{:?}", r),
    }
    let mut v2 = 6;
    match v2 {
        //匹配上再引用==let ref mut r = 6
        ref mut r => {
            *r += 10;
            println!("{:?}", v2)
        }
    }
    let ref mut r = 6;
    println!("{:?}", r);
}

fn match_guard() {
    let a = 5;
    match a {
        (x)if x > 3 => {
            println!(">3{:?}", x);
            println!("{:?}", a);
        }
        (x)if x < 0 => println!("<0{:?}", x),
        _ => { println!("{:?}", a); }
    }
}

fn match_bind() {
    fn age() -> i32 {
        15
    }
    match age() {
        n @ 1..=10 => println!("<10{:?}", n),
        n @ 10..=15 => println!(">10{:?}", n),
        n => println!("{:?}", n),
    }
}

fn if_let_demo() {
    let b = Some(7);
    if let Some(a) = b {
        println!("{:?}", a);
    }
    while let Some(a) = b {
        println!("{:?}", a);
        if a < 10 {
            break;
        }
    }
}

fn main() {
    if_let_demo();
}