// mod myfile;
// mod Implement
//
// pub trait Draw {
//     fn draw(&self);
// }
//
// pub struct Screen {
//     pub components: Vec<Box<dyn Draw>>,
// }
//
// impl Screen {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }
//
// pub struct Button {
//     pub width: u32,
//     pub height: u32,
//     pub label: String,
// }
//
// impl Draw for Button {
//     fn draw(&self) {
//         println!("Draw Button")
//     }
// }
//
// // pub trait Messenger {
// //     fn send(&self, msg: &str);
// // }
// //
// // pub struct LimitTracker<'a, T: Messenger> {
// //     messenger: &'a T,
// //     value: usize,
// //     max: usize,
// // }
// //
// // impl<'a, T> LimitTracker<'a, T>
// //     where T: Messenger {
// //     pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
// //         LimitTracker {
// //             messenger,
// //             value: 0,
// //             max,
// //         }
// //     }
// //
// //     pub fn set_value(&mut self, value: usize) {
// //         self.value = value;
// //         let percentage_of_max = self.value as f64 / self.max as f64;
// //
// //         if percentage_of_max >= 1.0 {
// //             self.messenger.send("Error: You are over your quota!");
// //         } else if percentage_of_max >= 0.9 {
// //             self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
// //         } else if percentage_of_max >= 0.75 {
// //             self.messenger.send("Warning: You've used up over 75% of your quota!");
// //         }
// //     }
// // }
// //
// // #[cfg(test)]
// // mod tests {
// //     use super::*;
// //     use std::cell::RefCell;
// //
// //     struct MockMessenger {
// //         sent_messages: RefCell<Vec<String>>
// //     }
// //
// //     impl MockMessenger {
// //         fn new() -> MockMessenger {
// //             MockMessenger {
// //                 sent_messages: RefCell::new(vec![])
// //             }
// //         }
// //     }
// //
// //     impl Messenger for MockMessenger {
// //         fn send(&self, message: &str) {
// //             self.sent_messages.borrow_mut().push(String::from(message))
// //         }
// //     }
// //
// //     #[test]
// //     fn it_sends_an_over_75_percent_warning_message() {
// //         let mock_messeger = MockMessenger::new();
// //         let mut limit_tracker =
// //             LimitTracker::new(&mock_messeger, 100);
// //
// //         limit_tracker.set_value(80);
// //
// //         assert_eq!(mock_messeger.sent_messages.borrow().len(), 1);
// //     }
// // }
// //
// // //
// // // #[test]
// // // fn iterator_demonstration() {
// // //     let v1 = vec![1, 2, 3];
// // //     let mut v1_iter = v1.iter();
// // //
// // //     assert_eq!(v1_iter.next(), Some(&1));
// // //     assert_eq!(v1_iter.next(), Some(&2));
// // //     assert_eq!(v1_iter.next(), Some(&3));
// // //     assert_eq!(v1_iter.next(), None);
// // // }
// // //
// // // #[test]
// // // fn iterator_sum() {
// // //     let v1 = vec![1, 2, 3];
// // //     let v1_iter = v1.iter();
// // //     let total: i32 = v1_iter.sum();
// // //
// // //     assert_eq!(total, 6);
// // // }
// // //
// // // #[test]
// // // fn map_adapter() {
// // //     let v1 = vec![1, 2, 3];
// // //     let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
// // //     assert_eq!(v2, vec![2, 3, 4]);
// // // }
// // //
// // // #[derive(PartialEq, Debug)]
// // // struct Shoe {
// // //     size: u32,
// // //     style: String,
// // // }
// // //
// // // fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
// // //     shoes.into_iter()
// // //         .filter(|s| s.size == shoe_size)
// // //         .collect()
// // // }
// // //
// // // #[test]
// // // fn filters_by_size() {
// // //     let shoes = vec![
// // //         Shoe { size: 10, style: "sneaker".to_string() },
// // //         Shoe { size: 13, style: "sandal".to_string() },
// // //         Shoe { size: 10, style: "boot".to_string() },
// // //     ];
// // //
// // //     let in_my_size = shoes_in_my_size(shoes, 10);
// // //     assert_eq!(
// // //         in_my_size,
// // //         vec![
// // //             Shoe { size: 10, style: String::from("sneaker") },
// // //             Shoe { size: 10, style: String::from("boot") },
// // //         ]
// // //     )
// // // }
// // //
// // // struct Counter {
// // //     count: u32
// // // }
// // //
// // // impl Counter {
// // //     fn new() -> Counter {
// // //         Counter { count: 0 }
// // //     }
// // // }
// // //
// // // impl Iterator for Counter {
// // //     type Item = u32;
// // //
// // //     fn next(&mut self) -> Option<Self::Item> {
// // //         self.count += 1;
// // //
// // //         if self.count < 6 {
// // //             Some(self.count)
// // //         } else {
// // //             None
// // //         }
// // //     }
// // // }
// // //
// // // #[test]
// // // fn calling_next_directly() {
// // //     let mut counter = Counter::new();
// // //     assert_eq!(counter.next(), Some(1));
// // //     assert_eq!(counter.next(), Some(2));
// // //     assert_eq!(counter.next(), Some(3));
// // //     assert_eq!(counter.next(), Some(4));
// // //     assert_eq!(counter.next(), Some(5));
// // //     assert_eq!(counter.next(), None);
// // //     assert_eq!(counter.next(), None);
// // //
// // // }
// // //
// // // #[test]
// // // fn using_other_iterator_trait_methods() {
// // //     let sum: u32 = Counter::new().zip(Counter::new().skip(1))
// // //         .map(|(a, b)| a * b)
// // //         .filter(|x| x % 3==0)
// // //         .sum();
// // //
// // //     assert_eq!(18, sum);
// // //
// // // }
// // //
