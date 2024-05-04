// #[derive(Debug)]
// pub struct LifeType<'a, T> {
//     content: &'a T,
// }


// #[derive(Debug)]
// pub struct NoLifeType<T> {
//     content: T,
// }


fn main() {
  // panic should not happen.
  // panic is per thread.
  // 1. parent thread can find out when a child thread panics and 
  //    handle the error gracefully.
  // 2. There is also a way to catch stack unwinding, allow the thread
  //    to survive and continue running.


  // Result should be handled.
  // let r: LifeType<String>;
  // {
  //   let msg = "this is a message".to_string();
  //   r = LifeType { content: &msg};
  // }
  // println!("{:?}", r);

  // let r: NoLifeType<&str>;
  // {
  //   let msg = "this is a message".to_string();
  //   r = NoLifeType { content: &msg[0..4]};
  // }
  // println!("{:?}", r);



}
