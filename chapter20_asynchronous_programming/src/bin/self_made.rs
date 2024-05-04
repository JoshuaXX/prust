
// trait and enum needed for async
pub trait Future {
    type Output;
    fn poll(&mut self) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}



/// Async code will be transformed into a state machine by compiler:
/// 1. State is represented as the enum.
/// 2. Computation will be transformed into Future's poll method.
/// 
/// 1. Every .await point is a potential state should be generated as a state.
/// 2. The computation logic before every .await point is generated as code to complete
///    as a way to reach that specific state.
/// 
#[derive(Debug)]
enum MyFuture {
    Init,
    Step1(usize),
    Done,
}

impl Future for MyFuture {
    type Output = usize;
    fn poll(&mut self) -> Poll<Self::Output> {
        // take ownership of the current state, we pinky-promise to put it back
        let this = std::mem::replace(self, Self::Done);

        let new = match this {
            Self::Init => Self::Step1(6),                  
            Self::Step1(n) => return Poll::Ready(7 * n),
            Self::Done => panic!("please stop polling me"), 
        };

        *self = new;
        Poll::Pending
    }
}

fn main() {
    // initialise the future
    let mut fut = MyFuture::Init;
    let n = loop {
        println!("polling - {fut:?}");
        // call poll
        match fut.poll() {
            // if ready, break the poll loop with our value
            Poll::Ready(n) => break n,
            // if pending, continue the loop
            Poll::Pending => println!("pending"),
        }
    };
    // Done!
    println!("ready!  - {n:?}");
}