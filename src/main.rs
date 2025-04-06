// these Code Snipptes are from a Medium article (from Black Hat Rust by S Kerkour and Rust Doc'tion)
// Smart pointers prevent common memory safety issues by enforcing strict ownership and borrowing rules
// Ensures resources are properly cleaned up when they are no longer needed
// It also offers thread-safe mechanism for sharing between threads

// Example A - Box<T>:

// - Box<T> allocates values on the heap
// - Provides heap allocationfor values - it is used when you need to stor data on the heap rather than a stack
// Its useful for recursive data structures and when you need to enforce size contraints.

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    println!("Hello, world!");


    // Example B Rc<T> (Reference Counting)
    // IN the book "Black Hat Rust", the author suggested smart pointers provide an effective solution for managing long-lived references, whether they are shared or exclusive,
    // and whether they are are mutable or immutable.

    use std::rc::Rc;

    let pointer = Rc::new(1);
    {
        let second_pointer = pointer.clone(); // or Rc::clone(&pointer)
        println!("{}", *second_pointer);
    }
    println!("{}", *pointer);

    // Example C. Rc<T>(Reference Counting) - a reference counting type that enables multiple ownership
   
    // It keeps track of the number of references to the data
    // Suitable for a single-threaded scenarios where multiple parts of the program need to read from the same data.
    // This example is to obtain a mutable, shared pointer (S Kerkour, 2002)

    use std::cell::{RefCell, RefMut};
    //use std::rc::Rc;

    //fn main()..

    let shared_string = Rc::new(RefCell::new("Hello".to_string()));

    {
        let mut hello_world: RefMut<String> = shared_string.borrow_mut();
        hello_world.push_str("World");
    }
    println!("{}", shared_string.take());

    // Example D.

    //Arc<T>(Atomic Reference Counting)

    // It is similar to Rc<T>, but safe to use in multi-threaded contexts (A.K.A concurrent situations) due to atomic reference counting
    // It's used when data needs to be shared across multiple threads

    use std::sync::Arc;
    use std::thread;
    
    let a = Arc::new(5);
    let b = Arc::clone(&a);

    let handle = thread::spawn(move || {
        println!("b = {}", b);
    });

    println!("a = {}", a);
    handle.join().unwrap();

    // Example E. Arc<T> 

   // use std::sync::{Arc, Mutex};
    //use std::thread;

    // fn main()..

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle  = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;

        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}