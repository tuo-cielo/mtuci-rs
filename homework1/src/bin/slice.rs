// // // fn first_word(s: &String) -> usize {
// // //     let bytes = s.as_bytes();
// // //
// // //     for (i, &item) in bytes.iter().enumerate() {
// // //         if item == b' ' {
// // //             return i;
// // //         }
// // //     }
// // //
// // //     s.len()
// // // }
// // //
// // // fn main() {
// // //     let mut s = String::from("hello world");
// // //
// // //     let word = first_word(&s); // word will get the value 5
// // //
// // //     s.clear(); // this empties the String, making it equal to ""
// // //
// // //     // word still has the value 5 here, but there's no more string that
// // //     // we could meaningfully use the value 5 with. word is now totally invalid!
// // // }
// // //
// // //Строковые срезы
// //
// // // fn main() {
// // //     let s = String::from("hello world");
// // //
// // //     let hello = &s[0..5];
// // //     let world = &s[6..11];
// // // }
// //
// // fn first_word(s: &String) -> &str {
// //     let bytes = s.as_bytes();
// //
// //     for (i, &item) in bytes.iter().enumerate() {
// //         if item == b' ' {
// //             return &s[0..i];
// //         }
// //     }
// //
// //     &s[..]
// // }
// //
// // fn main() {}
//
// // fn first_word(s: &str) -> &str {
// //     let bytes = s.as_bytes();
// //
// //     for (i, &item) in bytes.iter().enumerate() {
// //         if item == b' ' {
// //             return &s[0..i];
// //         }
// //     }
// //
// //     &s[..]
// // }
//
//
// // fn main() {
// //     let my_string = String::from("hello world");
// //
// //     // `first_word` works on slices of `String`s, whether partial or whole
// //     let word = first_word(&my_string[0..6]);
// //     let word = first_word(&my_string[..]);
// //     // `first_word` also works on references to `String`s, which are equivalent
// //     // to whole slices of `String`s
// //     let word = first_word(&my_string);
// //
// //     let my_string_literal = "hello world";
// //
// //     // `first_word` works on slices of string literals, whether partial or whole
// //     let word = first_word(&my_string_literal[0..6]);
// //     let word = first_word(&my_string_literal[..]);
// //
// //     // Because string literals *are* string slices already,
// //     // this works too, without the slice syntax!
// //     let word = first_word(my_string_literal);
// // }
//
// #![allow(unused)]
// fn main() {
// let a = [1, 2, 3, 4, 5];
//
// let slice = &a[1..3];
//
//     assert_eq!(slice, &[2, 3]);
// }
//
