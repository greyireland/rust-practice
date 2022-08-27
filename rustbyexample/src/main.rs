use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::future::pending;
use std::process::id;
use std::string::ParseError;

use anyhow::bail;

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

struct Func {
    a: String,
}

impl Func {
    fn new() -> Self {
        Func {
            a: "a".to_owned()
        }
    }
    fn hi(self) {
        println!("{:?}", self.a);
    }
    fn hi2(&self) {
        println!("{:?}", self.a);
    }
    fn hi3(&mut self) {
        self.a = "aa".to_string();
        println!("{:?}", self.a);
    }
}

fn func_demo() {
    fn test(i: i32) -> (i32, i32) {
        (i, i + 1)
    }
    println!("{:?}", test(1));
    let mut f = Func::new();
    // f.hi();//move
    f.hi2();
    f.hi3();
    // 借用改变值，需要函数声明为mut，move|x|{}会强制move
    let mut f2 = |a| {
        println!("{:?}", a);
        (&f).hi2();
    };
    f2(1111);
}

fn fn_iter() {
    let a = vec![1, 2, 3];
    let contains = a.iter().any(|&x| x == 2);
    println!("{:?}", contains);
    // iter引用一次，find引用一次，所以是两次引用
    let item = a.iter().find(|&&x| x == 2).unwrap();
    println!("{:?}", item);
    let b = a.into_iter().any(|x| x == 2);
    println!("{:?}", b);
    // println!("{:?}", a);//err moved
}

fn str_str() {
    let hello = "hello world";
    let h = "wor";
    if let Some(idx) = hello.find(h) {
        println!("{:?}", idx);
        let cut = &hello[idx + 3..idx + 5];
        println!("{:?}", cut);
    }
}
//pub mod x;
//pub use mod::x;


mod my_mod {
    pub fn hello() {
        println!("{:?}", "hello");
    }
}

fn mod_demo() {
    hello();
    use my_mod::hello;
    hello();
}

// crate
// extern crate rary; // 在 Rust 2015 版或更早版本需要这个导入语句
//cargo
// attribute
#[allow(dead_code)]
fn unused_fn() {
    println!("{:?}", "a");
}

//泛型（类型参数）
// 定义泛型类型或泛型函数之类的东西时，我们会用 <A> 或者 <T> 这类标记 作为类型的代号
// 就像函数的形参一样。在使用时，为把 <A>、<T> 具体化，我们 会把类型说明像实参一样使用
// 像是 <i32> 这样。这两种把（泛型的或具体的）类型 当作参数的用法就是类型参数。
struct A {}

struct Single(A);

struct SingleGeneric<T>(T);

// 用于函数
fn min<T>(a: T, b: T) -> T {
    a
}

// 用于impl实现
impl<T> SingleGeneric<T> {
    fn new() {}
}

//泛型以trait约束
fn print<T: Display>(t: T) {
    println!("{}", t);
}

// 多重约束
fn compare<T: Debug + Display>(t: &T) {
    println!("{:?}", t);//Debug
    println!("{}", t);//Display
}
// where约束

// 作用域
//RAII获取资源及初始化，完结后销毁 constructor开始--中间可使用--finally结束
// 析构函数Drop
struct ToDrop;

impl Drop for ToDrop {
    // 这个drop算是销毁之前的事件？
    fn drop(&mut self) {
        println!("{:?}", "instance droped");
    }
}

fn ownership() {
    let immutable_box = Box::new(5u32);

    let mut b = immutable_box;//改变所有权并改变可变性

    *b = 10;
    println!("{:?}", b);

    // ref模式
    let c = 'q';
    let ref b = c;
    let ref d = &c;//b和d是等价的
}
//还可以部分移动
// 正常情况都使用借用

//生命周期
fn failed_borrow<'a>(v: &'a i32) {
    let x = 1;
    // let y: &'a i32 = &x; //报错，因为生命周期不够长
    // let b = v;
}

//结构体中的变量生命周期必须比结构体生命周期长（或者一样长）
struct NamedBorrowed<'a> {
    x: &'a i32,
}
//大部分生命周期都可以省略，由编译器自动判定

//trait==接口golang interface
//Clone 从&T创建副本T
//Copy 复制语义，非转移语义
//Debug打印 Default空实例

struct Sheep {}

struct Cow {}

trait Animal {
    // 实例方法签名
    fn noise(&self) -> &'static str;
}

// 实现 `Sheep` 的 `Animal` trait。
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// 实现 `Cow` 的 `Animal` trait。
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// 返回一些实现 Animal 的结构体，但是在编译时我们不知道哪个结构体。
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}
//Drop常见的资源类的都实现了Drop，如Box,Vec,String,File,Process

//Iterator迭代器，默认for使用into_iter()，会转移资源,iter()是借用，不会转移资源
// impl Iterator<Item=i32>简化返回的
fn combine_vecs(
    v: Vec<i32>,
    u: Vec<i32>,
) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

//rust赋值默认行为是移动，有时候需要用到两份资源，所以可以使用clone
// 实现Clone和Copy
#[derive(Debug, Clone, Copy)]
struct HasCopyA {
    a: i32,//i32简单类型，实现了Copy所有整个结构体是copy的
}

// 只实现了 `Clone` trait，需要手动clone
#[derive(Clone, Debug)]
struct HasCloneB(Box<i32>, Box<i32>);

//如果都是内置简单类型相当于自动实现了Clone和Copy，
// 如果是结构体想实现Copy必须内部所有字段也实现Copy，不然是没用的

fn clone_copy() {
    let a = HasCopyA { a: 1 };
    let b = a;//隐式Copy
    println!("{:?}", a);
    println!("{:?}", b);

    let c = HasCloneB(Box::new(1), Box::new(2));
    let d = c.clone();//显式clone
    println!("{:?}", c);
    println!("{:?}", d);
}

//父trait，trait可以继承
trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn age(&self) -> i32;
}

struct Me {
    name: String,
    age: i32,
}

impl Student for Me {
    fn age(&self) -> i32 {
        self.age
    }
}

impl Person for Me {
    fn name(&self) -> String {
        self.name.clone()
    }
}

fn hello(v: &dyn Student) {
    println!("name{:?},age{}", v.name(), v.age());
}

fn parent_trait_demo() {
    hello(&Me { name: "a".to_string(), age: 1 })
}

//宏 模版代码，可以解析语法
macro_rules! say_hello {
    ()=>(
        println!("hello!")
    )
}
fn macro_demo() {
    say_hello!();
}

//错误处理 Option 和Result<T,E>更加丰富的Option
//anyhow::Result<T,E>
//panic!和golang panic("")一致
// Option<T>两个选择Some(T),None
fn option_demo() {
    let a = Some(1);

    if let Some(v) = a {
        println!("{:?}", v);
    } else {
        println!("{:?}", "error");
    }
    let b = a.unwrap_or_default();
    println!("{:?}", b);
}

fn result_demo() -> Result<i32, Box<dyn Error>> {
    let a = "a".parse::<i32>()?;
    let b = "2".parse::<i32>()?;
    Ok(a * b)
}

fn anyhow_demo() -> anyhow::Result<i32> {
    let a = "1".parse::<i32>()?;
    let b = "2".parse::<i32>()?;
    let c = a * b;
    println!("{:?}", c);
    Ok(c)
}

fn anyhow2_demo() -> anyhow::Result<i32> {
    let a = "a".parse::<i32>().map_err(|err| {
        println!("{:?}", err);//提前日志记录
        err//原路返回
    }).unwrap_or_default();
    let b = "2".parse::<i32>()?;
    let c = a * b;
    println!("{:?}", c);
    Ok(c)
}

fn main() {
    anyhow2_demo().unwrap();
}