// these Code Snippets are from a Medium article (from Black Hat Rust by S Kerkour and Rust Doc'tion)

// Smart pointers prevent common memory safety issues by enforcing strict ownership and borrowing rules
// Ensures resources are properly cleaned up when they are no longer needed
// It also offers thread-safe mechanism for sharing between threads

// Example A - Box<T>:

// - Box<T> allocates values on the heap
// - Provides heap allocation for values - it is used when you need to store data on the heap rather than a stack
// It's useful for recursive data structures and when you need to enforce size contraints.

use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;


fn main() {
    
    let b = Box::new(5);
    println!("b = {}", b);
    println!("Hello, world!");


// Example B Rc<T> (Reference Counting)

// IN the book "Black Hat Rust", the author suggested smart pointers provide an effective solution for managing long-lived references, 
// whether they are shared or exclusive, 
// and whether they are are mutable or immutable.


    // let pointer = Rc::new(2);
    // {
    //     let second_pointer = pointer.clone(); // or Rc::clone(&pointer)
    //     println!("{}", *second_pointer);
    // }
    // println!("{}", *pointer);

    // Example C. Rc<T>(Reference Counting) - a reference counting type that enables multiple ownership
   
    // It keeps track of the number of references to the data
    // Suitable for a single-threaded scenarios where multiple parts of the program need to read from the same data.
    // This example is to obtain a mutable, shared pointer (S Kerkour, 2002)

    
    //fn main()..

    // let shared_string = Rc::new(RefCell::new("Hello ".to_string()));

    // {
    //     let mut hello_world: RefMut<String> = shared_string.borrow_mut();
    //     hello_world.push_str("World");
    // }
    // println!("{}", shared_string.take());

    // Example D.

    //Arc<T>(Atomic Reference Counting)

    // It is similar to Rc<T>, but safe to use in multi-threaded contexts (A.K.A concurrent situations) due to atomic reference counting
    // It's used when data needs to be shared across multiple threads

    // let a = Arc::new(5);
    // let b = Arc::clone(&a);

    // let handle = thread::spawn(move || {
    //     println!("b = {}", b);
    // });

    // println!("a = {}", a);
    // handle.join().unwrap();

    // Example E. Arc<T> 

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


    // DANGLING Raw Pointer Example: - this is unsafe code, dangling pointers occur when raw pointers are used incorrectly.
    // Use safe Rust where compile time checks with (ownership, borrowing, and lifetimes)

    fn create_dangling_pointer() -> *const i32 {
        let x = 42;
        &x as *const i32  // returns a pointer to 'x' which is destroyed when the function exits
    }

    //fn main() {
        let ptr = create_dangling_pointer();
        unsafe {
            println!("Dereferencing dangling pointer: {}", *ptr) // Undefined behaviour
        }
    //}

    // UNSAFE RUST (Unsafe SUPER Powers) 
    
    // -> A Dereference a Rawmpointer , B Call an unsafe func/method , C Acces or modify a mutable static variable,
    //  D Implement an unsafe trait, E Access fields of a union


    // 1. Creating an IMMUTABLE & a MUTABLE raw pointer
     
        let mut num = 6;

        let r1 = &raw const num;
        let r2 = &raw mut num;

    //Raw pointer with unsafe keyword
    
        let mut num = 7;
        
        let r3 = &raw const num;
        let r4 = &raw mut num;

        unsafe {
            println!("r3 is: {}", *r3);
            println!("r4 is: {}", *r4); 
        } 
        // BOTH raw pointers are pointing at the same memory spot (data "data races can occur")
        // above, *const i32 and *mut i32 was created = data race! = data in the same memory location 

        // When interacting with C code - you may need raw pointers OR when building a Safe ABSTRACTIONS
        // that the borrow checker does not understand.

    // 2. Calling an unsafe Function or Method - they look like regular functions / methods with keyword 'únsafe'

        unsafe fn dangerous() {}

        unsafe {
            dangerous();
        }
        

    // Creating a safe abstraction over unsafe code - using mutable slices

        // fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        //     let len = values.len();
        //     let prt = values.as_mut_ptr();

        //     assert!(mid <= len);

        //     unsafe {
        //         (
        //             slice::from_raw_parts_mut(ptr, mid),
        //             slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        //         )
        //     }


        // Using unsafe in REAL life:

        // fn main()..

        //     let mut data = [1, 2, 3, 4, 5, 6];

        //     let (left, right) = split_at_mut(&mut data, 3);

        // // modify the slices to show they are mutable and separate:

        //     left[0] = 20;
        //     right[0] = 50;

        //     println!("Modified data: {:?}", data);            

        // }

    // fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //     let len = values.len();
    //     let ptr = values.as_mut_ptr();

    //     assert!(mid <= len);

    //     unsafe {
    //         (
    //             slice::from_raw_parts_mut(ptr, mid),
    //             slice::from_raw_parts_mut(ptr.add(mid), len - mid),
    //         )
    //     }


        // Using extern Functions to Call external Code - (interacting with C code)

        unsafe extern "C" {
            fn abs(input: i32) -> i32;
        }

       // fn main() {
            unsafe {
                println!("Absolute value of -3 according to C: {}", abs(-3));
            }
       // }

       // SLICES - The Slice Type:

       // Problem - write a function that takes a string of words separated by spaces and returns the first word it finds in that string.
       // If the function does not find a space in the string, the whole string must be one word, so the entire string should be returned.

       // FIRST - lets see how we wld write the signature of this function WITHOUT using slices, to understand the problem that slices will solve:
       
      //      fn first_word(s: &String) -> ?

        // the first_word function has a '&String as a parameter. We dont need ownership, so this is fine. But, what should we return?
       // We dont reallyhave a way to talk about part of a string. However, we could return the index of the end of the word, indicated by a space:
       
       // EXAMPLE:

            fn first_word(s: &String) -> usize {
                let bytes = s.as_bytes();

                for (i, &item) in bytes.iter().enumerate() {
                    if item == b' ' {
                        return i;
                    }
                }

                s.len()

                  // ..because we need to go through the string, element by element and check whether a value is a space,
                // we'll convert our string to an array of bytes using the 'as_bytes' method
        
                //    let bytes = s.as_bytes(); - NEXT , we create an iterator over the array of bytes using the 'iter' method:
                //    for (i, &item) in bytes.iter().enumerate() {}
    

        }       

    
// EXPLORING INTERIOR mutability with RefCell<T>

// RefCell<T> is a runtime-checked mutable memory location that enables INTERIOR mutability - a patter where you can mutate data even when you have an 
// immutable reference to it. It enforces Rust's borrowing rules (either one mutable reference OR multiple references) at runtime instead of compile time.
// RefCell<T> complements Rc<T> - when both are combined - you get shared ownership WITH mutability.

// - How it works is use borrow() to get immutable reference (Ref<T>)
//- Use borrow_mut() to get a mutable reference (RefMut<T>) - if you violate the borrowing rules - it panics at runtime.


// // A use CASE - Testing

    struct DatabaseClient {
        cache: RefCell<HashMap<String, String>>,
    }

    impl DatabaseClient {
        fn get(&self, key: &str) -> String {
            let mut cache = self.cache.borrow_mut(); // Mutate cache immutably
            cache.entry(key.to_string())
            .or_insert("default".into())
            .clone()
    }
}

    

fn example_refcell() {
    //use std::cell::RefCell;

    let x = RefCell::new(42);
    {
        let mut y = x.borrow_mut(); // mutable borrow 
        *y += 1; // borrow ends here
     }
    let z = x.borrow(); // immutable borrow now allowed.
    println!("RefCell value: {}", z);
}

// Combining RefCell<T> and Rc<T>

    let shared_data = Rc::new(RefCell::new(0));

    let clone1 = Rc::clone(&shared_data);
    let clone2 = Rc::clone(&shared_data);
    let clone3 = Rc::clone(&shared_data);
    let clone4 = Rc::clone(&shared_data);

    *clone1.borrow_mut() += 1; // Mutate through Rc
    *clone2.borrow_mut() += 2;
    *clone3.borrow_mut() += 5;
    *clone4.borrow_mut() -= 4;

    println!("Final value: {}", shared_data.borrow()); 



}



    


    
        











