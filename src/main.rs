// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {

    // Use the values of the input arguments
    // All new cars always have zero mileage
    let car: Car = Car {
        color,
        transmission,
        convertible,
        mileage: 0,
    };

    car
}

fn main() {
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);


    /*fn divide_by_5(num: u32) -> u32 {
            if (0 == num) {
                return 0;
            }

            num / 5
        }*/

    /*let we_load = WebEvent::WELoad(true);

    let click = MouseClick { x: 100, y: 250 };
    println!("Mouse click location: {} {}", click.x, click.y);
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("\nKeys pressed: {}{}", keys.0, keys.1);
    println!("\nKeys pressed: {}{}", keys.0, keys.1);

    let we_load = WebEvent::WELoad(true);
    let we_click = WebEvent::WEClikc(click);
    let we_key = WebEvent::WEKey(keys);
    let a = "asa";*/



    /*  let user_2 = Student { name: "Dyson Tan".to_string(), level: 5, remote: false };
      println!(user_2.level)*/
    // let user_1 = Student {
    //     name: "Constanc Sharama".to_string(),
    //     level: 2,
    //     remote: false,
    // };
    //
    // let grades1 = Grades('A', 'B', 'B', 'A', 3.75);
    // println!("{} {}", user_1.name, grades1.0);

    // let string_2: str = "ace";

    // println!("{}", string_2);
    // let is_bigger = 1> 4;
    // println!("Is 1 > 4 {}",is_bigger);
    /* println!("1 +2 ={} and 8-5 = {}, and 15*3={}", 1u32 + 2, 8i32 - 5, 15 * 3);
    println!("9/2={} but 9.0/2.0={}", 9u32 / 2, 9.0 / 2.0);*/

    /* let number_64=4.0;
    let number_32: f32 = 5.0;*/

    /* let number:u32=14;
    println!("{}", number);*/

    /* let shadow_num=5;
    let shadow_num = shadow_num + 5;
    let shadow_num=shadow_num*2;
    println!("{}", shadow_num);*/

    /*let mut a_number=10;
    println!("{}", a_number);
    a_number = 22;

    println!("{}", a_number);*/
    // let a_number;
    // let a_word = "Ten";
    // println!("The number is {}", a_number);
    // a_number=10;
    // println!("The word is {}", a_word);
}
/*// enum WebEvent{
//     WELoad,
//     WEKey(String,char),
//     WEClick{x:i64,y:i64}
// }

struct KeyPress(String, char);

struct MouseClick {
    x: i64,
    y: i64,
}

enum WebEvent { WELoad(bool), WEClikc(MouseClick), WEKey(KeyPress) }

*/
// struct Student {
//     name: String,
//     level: u8,
//     remote: bool,
// }
// struct Grades(char, char, char, char, f32);
// struct Unit;

// struct Student { name: String, level: u8, remote: bool }

// fn main() {
//     let a =1;
//     let b = a;
//     println!("{} -> {:p}", b,&b);
//
//     println!("{} -> {:p}", a, &a);
// }
// fn main() {
//     let mut data = vec![1, 2, 3, 4];
//     let data1 = vec![&data[0]];
//     println!("data[0]:{:p}", &data[0]);
//     for i in 0..100 {
//         data.push(i);
//     }
//
//     println!("data[0]: {:p}", &data[0]);
//     println!("boxed:{:p}", &data1);
// }
// fn main() {
//     let mut data = vec![1, 2, 3];
//
//     for item in data.iter_mut() {
//         data.push(*item + 1);
//     }
// }

// fn main() {
//     let mut data:Vec<&u32> = Vec::new();
//     push_local_ref(&mut data);
//     println!("data:{:?}", data);
// }
//
// fn push_local_ref(data: &mut Vec<&u32>) {
//     let v = 42;
//     data.push(&v);
// }
// fn main() {
//     let mut data:Vec<&u32> = Vec::new();
//     let v = 42;
//     data.push(&v);
//     println!("data: {:?}", data);
// }
// fn main() {
//     let r = local_ref();
//     println!("r: {:p}", r);
//
// }
// fn local_ref<'a>() -> &'a i32 {
//     let a = 40;
//     &a
// }

// fn main() {
//     let data = vec![1, 2, 3, 4];
//     let data1 = &data;
//     println!(
//         "addr of value: {:p} ( {:p} ), \naddr of data {:p}, data1: {:p}",
//         &data, data1, &&data, &data1
//     );
//     println!("sum of data1:{}", sum(data1));
//
//     // 堆上数据的地址是什么？
//     println!(
//         "addr of items :[{:p}, {:p}, {:p}, {:p}]",
//         &data[0], &data[1], &data[2], &data[3]
//     );
//
//     println!(
//         "addr of items :[{}, {}, {}, {}]",
//         &data[0], &data[1], &data[2], &data[3]
//     );
//
//     println!(
//         "addr of items :[{}, {}, {}, {}]",
//         data[0], data[1], data[2], data[3]
//     );
// }
//
// fn sum(data: &Vec<u32>) -> u32 {
//     println!("> addr of value: {:p}, addr of ref: {:p}", data, &data);
//     data.iter().fold(0, |acc, x| acc + x)
// }

// fn main() {
//     // let a1=11;
//     // let b1 = a1;
//     // println!("{}", a1);
//     let data = vec![1, 2, 3, 4];
//     let data1 = &data;
//     println!("sum of data1 {}", sum(data1));
//     // println!("data1: {:?}", data1);
//     println!("sum of data: {}", sum(data));
// }

// fn sum(data: Vec<u32>) -> u32 {
//     data.iter().fold(0, |acc,x| acc + x)
// }
// struct Point{
// x:i32,
// y:i32,
// }
// fn main() {
//     let a = Box::new(Point { x: 1, y: 2 });
//
// }
// use std::rc::Rc;
// use std::cell::RefCell;
// use crate::List::Nil;
//
//
// #[derive(Debug)]
// enum List{
//     Cons(Rc<RefCell<i32>>,Rc<List>),
//     Nil,
// }
//
// fn main() {
//     let value = Rc::new(RefCell::new(5));
//     let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
//     let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
//     let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
//
//     *value.borrow()+=10;
//
// }
//
// fn Cons(p0: Rc<RefCell<i32>>, p1: Rc<List>) -> T {
//     todo!()
// }
//
// // use std::fs::File;
// // use std::io::{Error, ErrorKind, Read};
// // use std::io;
// // include!("myfile.rs");
// //
// // // // // // use std::thread;
// // // // // // use std::time::Duration;
// // // // // //
// // // // // // // use std::cell::RefCell;
// // // // // // // use std::rc::Rc;
// // // // // // // use crate::List::{Cons, Nil};
// // // // // // //
// // // // // // // #[derive(Debug)]
// // // // // // // enum List{
// // // // // // //     Cons(Rc<RefCell<i32>>, Rc<List>),
// // // // // // //     Nil,
// // // // // // // }
// // // // // // //
// // // // // // // fn main() {
// // // // // // //     let value = Rc::new(RefCell::new(5));
// // // // // // //     let a = Rc::new(Cons(Rc::clone(&value),Rc::new(Nil)));
// // // // // // //     let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
// // // // // // //     let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
// // // // // // //
// // // // // // //     *value.borrow_mut() += 10;
// // // // // // //     println!("a after = {:?}", a);
// // // // // // //     println!("b after = {:?}", b);
// // // // // // //     println!("c after = {:?}", c);
// // // // // // // }
// // // // // // // // use std::thread;
// // // // // // // // use std::time::Duration;
// // // // // // // //
// // // // // // // // pub trait Iterator {
// // // // // // // //     type Item;
// // // // // // // //     fn next(&mut self) -> Option<Self::Item>;
// // // // // // // // }
// // // // // // // //
// // // // // // // // enum List {
// // // // // // // //     Cons(i32, Rc<List>),
// // // // // // // //     Nil,
// // // // // // // // }
// // // // // // // //
// // // // // // // // use crate::List::{Cons, Nil};
// // // // // // // // use std::rc::Rc;
// // // // // // // //
// // // // // // // //
// // // // // // // // fn main() {
// // // // // // // //     let x  =5;
// // // // // // // //     let y = &mut x;
// // // // // // // //     // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
// // // // // // // //     // println!("count after creating a = {}", Rc::strong_count(&a));
// // // // // // // //     //
// // // // // // // //     // let b = Cons(3, Rc::clone(&a));
// // // // // // // //     // println!("count after creating b = {}", Rc::strong_count(&a));
// // // // // // // //     // {
// // // // // // // //     //     let c = Cons(4, Rc::clone(&a));
// // // // // // // //     //     println!("count after creating c = {}", Rc::strong_count(&a));
// // // // // // // //     //
// // // // // // // //     // }
// // // // // // // //     // println!("count after c goes out of scope = {}", Rc::strong_count(&a));
// // // // // // // //
// // // // // // // //
// // // // // // // //
// // // // // // // //     // let a = Rc::new(
// // // // // // // //     //     Cons(5,
// // // // // // // //     //          Rc::new(
// // // // // // // //     //              Cons(10,
// // // // // // // //     //                   Rc::new(Nil)))));
// // // // // // // //     // let b = Cons(3, Rc::clone(&a));
// // // // // // // //     // let c = Cons(4, Rc::clone(&a));
// // // // // // // //     // let v1 = vec![1, 2, 3];
// // // // // // // //     // let v1_iter = v1.iter();
// // // // // // // //     // for x in v1_iter {
// // // // // // // //     //     println!("{}", x);
// // // // // // // //     // }
// // // // // // // //     // let example_closure = |x| x;
// // // // // // // //     // let demo2 = example_closure(2);
// // // // // // // //     // let s = example_closure("asdfa".to_string());
// // // // // // // //     //
// // // // // // // //     // println!("{}", demo2);
// // // // // // // //     // println!("{}", s);
// // // // // // // //     // let simulated_user_specified_value = 10;
// // // // // // // //     // let simulated_random_number = 7;
// // // // // // // //     //
// // // // // // // //     // generate_workout(
// // // // // // // //     //     simulated_user_specified_value,
// // // // // // // //     //     simulated_random_number
// // // // // // // //     // );
// // // // // // // // }
// // // // // // // //
// // // // // // // // struct Cacher<T>
// // // // // // // //     where T: Fn(u32) -> u32
// // // // // // // // {
// // // // // // // //     calculation: T,
// // // // // // // //     value: Option<u32>,
// // // // // // // // }
// // // // // // // //
// // // // // // // // impl<T> Cacher<T>
// // // // // // // //     where T: Fn(u32) -> u32
// // // // // // // // {
// // // // // // // //     fn new(calculation: T) -> Cacher<T> {
// // // // // // // //         Cacher {
// // // // // // // //             calculation,
// // // // // // // //             value: None,
// // // // // // // //         }
// // // // // // // //     }
// // // // // // // //
// // // // // // // //     fn value(&mut self, arg: u32) -> u32 {
// // // // // // // //         match self.value {
// // // // // // // //             None => {
// // // // // // // //                 let v = (self.calculation)(arg);
// // // // // // // //                 self.value = Some(v);
// // // // // // // //                 v
// // // // // // // //             }
// // // // // // // //             Some(v) => { v }
// // // // // // // //         }
// // // // // // // //     }
// // // // // // // // }
// // // // // // // //
// // // // // // // //
// // // // // // // // fn generate_workout(intensity: u32, random_number: u32) {
// // // // // // // //     // let expensive_result = simulated_expensive_calculation(intensity);
// // // // // // // //
// // // // // // // //     let expensive_closure = |num| {
// // // // // // // //         println!("calculatig slowly...");
// // // // // // // //         thread::sleep(Duration::from_secs(2));
// // // // // // // //         num
// // // // // // // //     };
// // // // // // // //
// // // // // // // //     if intensity < 25 {
// // // // // // // //         println!(
// // // // // // // //             "Today, do {} pushups!",
// // // // // // // //             expensive_closure(intensity)
// // // // // // // //         );
// // // // // // // //         println!(
// // // // // // // //             "Next, do {} situps!",
// // // // // // // //             expensive_closure(intensity)
// // // // // // // //         );
// // // // // // // //     } else {
// // // // // // // //         if random_number == 3 {
// // // // // // // //             println!("Take a break today! Remember to stay hydrated!");
// // // // // // // //         } else {
// // // // // // // //             println!(
// // // // // // // //                 "Today, run for {} minutes!",
// // // // // // // //                 expensive_closure(intensity)
// // // // // // // //             );
// // // // // // // //         }
// // // // // // // //     }
// // // // // // // // }
// // // // // // // //
// // // // // // // // fn simulated_expensive_calculation(intensity: u32) -> u32 {
// // // // // // // //     println!("calculating slowly...");
// // // // // // // //     thread::sleep(Duration::from_secs(2));
// // // // // // // //     intensity
// // // // // // // // }
// // // // // // // //
// // // // // // fn main() {
// // // // // //    let handle= thread::spawn(||{
// // // // // //         for i in 1..10{
// // // // // //             println!("{}", i);
// // // // // //             thread::sleep(Duration::from_millis(1));
// // // // // //         }
// // // // // //     });
// // // // // //
// // // // // //     handle.join();//.unwrap();
// // // // // //     for i in 1..5{
// // // // // //         println!(">>> {}", i);
// // // // // //         thread::sleep(Duration::from_millis(1));
// // // // // //     }
// // // // // //
// // // // // // }
// // // // // use std::thread;
// // // // // use std::sync::Mutex;
// // // // //
// // // // // fn main() {
// // // // //     let m = Mutex::new(5);
// // // // //     {
// // // // //         let mut num = m.lock().unwrap();
// // // // //         *num = 6;
// // // // //     }
// // // // //     println!("{:?}", m)
// // // // //
// // // // //     // let v = vec![1, 2, 3];
// // // // //     //
// // // // //     // let handle = thread::spawn(move || {
// // // // //     //     println!("{:?}", v);
// // // // //     // });
// // // // //     // // drop(v);
// // // // //     // handle.join().unwrap();
// // // // // }
// // // // use std::sync::{Mutex, Arc};
// // // // use std::thread;
// // // //
// // // // fn main() {
// // // //     let counter = Arc::new(Mutex::new(0));
// // // //
// // // //     let mut handles = vec![];
// // // //
// // // //     for _ in 0..10 {
// // // //         let counter = Arc::clone(&counter);
// // // //         let handle = thread::spawn(move || {
// // // //             let mut num = counter.lock().unwrap();
// // // //
// // // //             *num += 1;
// // // //         });
// // // //
// // // //         handles.push(handle);
// // // //     }
// // // //
// // // //     for handle in handles {
// // // //         handle.join().unwrap();
// // // //     }
// // // //
// // // //     println!("Result: {}", *counter.lock().unwrap());
// // // // }
// // //
// // // pub struct AveragedCollection {
// // //     list: Vec<i32>,
// // //     average: f64,
// // // }
// // //
// // // impl AveragedCollection {
// // //     pub fn add(&mut self, value: i32) {
// // //         self.list.push(value);
// // //         self.update_average();
// // //     }
// // //
// // //     pub fn remove(&mut self) -> Option<i32> {
// // //         let result = self.list.pop();
// // //         match result {
// // //             None => { None }
// // //             Some(v) => {
// // //                 self.update_average();
// // //                 v
// // //             }
// // //         }
// // //     }
// // //
// // //     pub fn average(&self) -> f64 {
// // //         self.average;
// // //     }
// // //
// // //
// // //     fn update_average(&mut self) {
// // //         let total :i32 = self.list.iter().sum();
// // //         self.average = total as f64 / self.list.len() as f64;
// // //     }
// // // }
// // //
// // // fn main() {}
// // // use hello_world::{Draw, Screen, Button};
// // //
// // // struct SelectBox {
// // //     width: u32,
// // //     height: u32,
// // //     options: Vec<String>,
// // // }
// // //
// // // impl Draw for SelectBox {
// // //     fn draw(&self) {
// // //         println!("Draw SelectBox")
// // //     }
// // // }
// // //
// // // fn main() {
// // //     // let screen = Screen{ components: vec![
// // //     //     Box::new(SelectBox{
// // //     //         width: 75,
// // //     //         height: 10,
// // //     //         options: vec![
// // //     //             String::from("Yes"),
// // //     //             String::from("Maybe"),
// // //     //             String::from("No"),
// // //     //         ],
// // //     //     }),
// // //     //     Box::new(Button{
// // //     //         width: 50,
// // //     //         height: 10,
// // //     //         label: "Ok".to_string(),
// // //     //     }),
// // //     // ] };
// // //
// // //     let screen = Screen {
// // //         components: vec![
// // //             Box::new(String::from("Hi"))
// // //         ]
// // //     };
// // //
// // //     screen.run();
// // // }
// //
// // fn main() {
// //    //  let f = File::open("hello.txt");
// //    // let f =  match f {
// //    //      Ok(file) => {file}
// //    //      Err(err) => match err.kind(){
// //    //          ErrorKind::NotFound => match File::create("hello.txt"){
// //    //              Ok(fc) => {fc}
// //    //              Err(e) => panic!("Error createing file: {:?}",e)
// //    //          }
// //    //          other_err => {panic!("Error open the file {:?}",other_err)}
// //    //      }
// //    //  };
// //
// //     // let f = File::open("hello.txt").unwrap_or_else(|error|{
// //     //     if error.kind()==ErrorKind::NotFound {
// //     //         File::create("hello.txt").unwrap_or_else(|error|{
// //     //             panic!("Error createing file: {:?}",error);
// //     //         })
// //     //     }else {
// //     //         panic!("Error open the file {:?}",other_err)
// //     //     }
// //     // });
// //
// //     //////
// // let result = read_username_from_file();
// //
// //     // // println!("{}","00008".parse::<i32>().unwrap());
// //     // let num = 12399400;
// //     // let r = Solution::reverse(num);
// //     //
// //     // // let aaa =num.to_string().chars().skip(1)
// //     // println!("{}", r);
// // }
// //
// // fn read_username_from_file() -> Result<String,io::Error>{
// //     // let f = File::open("hello.txt");
// //     // let mut f =match f {
// //     //     Ok(file) => {file}
// //     //     Err(e) => { return Err(e); }
// //     // };
// //     //
// //     // let mut s =String::new();
// //     // match f.read_to_string(&mut s) {
// //     //     Ok(_) => {Ok(s)}
// //     //     Err(e) => {e}
// //     // }
// //     // ////////
// //     // let mut f = File::open("hello.txt")?;
// //     // let mut s = String::new();
// //     // f.read_to_string(&mut s)?;
// //     // Ok(s)
// //
// //     ////////
// //     let mut s = String::new();
// //     let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
// //     Ok(s)
// // }
// //
// // struct Solution;
// //
// // impl Solution {
// //     pub fn reverse(x: i32) -> i32 {
// //         // let x1 = x.to_string();
// //         // for a in x.chars().rev() {
// //         //     println!("{}", a);
// //         // }
// //         let mut x2 = x;
// //         let mut x3 = "".to_string();
// //         let pre_symbol = if x < 0 {
// //             x2 *= -1;
// //             for c in x.to_string().chars().skip(1) {
// //                 println!("{}", c);
// //                 x3.push(c);
// //             }
// //             -1
// //         } else {
// //              x3.push_str(x.to_string().rev().as_str());
// //             1
// //         };
// //
// //         let mut x3 = x3.parse::<i32>().unwrap();
// //         x3 *= pre_symbol;
// //
// //         x3
// //     }
// // }
