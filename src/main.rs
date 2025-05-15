use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;



// these Code Snippets are from (from Black Hat Rust by S Kerkour and Rust Doc'tion) 

// Smart pointers prevent common memory safety issues by enforcing strict ownership and borrowing rules
// Ensures resources are properly cleaned up when they are no longer needed
// Also offers thread-safe mechanism for sharing data between threads

// Example A - Box<T>:

// - Box<T> allocates values on the heap
// - Provides heap allocation for values - it is used when you need to store data on the heap rather than a stack
// It's useful for recursive data structures and when you need to enforce size contraints.

// fn main() {
    
//     let b = Box::new(5);
//     println!("b = {}", b);
    

// // Example B Rc<T> (Reference Counting)

// // IN the book "Black Hat Rust", the author suggested smart pointers provide an effective solution for managing long-lived references, 
// // whether they are shared or exclusive, 
// // and whether they are are mutable or immutable.


//     let pointer = Rc::new(2);
//     {
//         let second_pointer = pointer.clone(); // or Rc::clone(&pointer)
//         println!("{}", *second_pointer);
//     }
//     println!("{}", *pointer);
    

// //     // Example C. Rc<RefCell<T>>(Reference Counting) - a reference counting type that enables multiple ownership with interior mutation.
   
// //     // It keeps track of the number of references to the data
// //     // Suitable for a single-threaded scenarios where multiple parts of the program need to read from the same data.
// //     // This example is to obtain a mutable, shared pointer (S Kerkour, 2002)

    
//     //fn main()..

//     let shared_string = Rc::new(RefCell::new("Hello ".to_string()));

//     {
//         let mut hello_world: RefMut<String> = shared_string.borrow_mut();
//         hello_world.push_str("World");
//     }
//     println!("{}", shared_string.take());

// //    // Example D.

// //     //Arc<T>(Atomic Reference Counting)

// //     // It is similar to Rc<T>, but safe to use in multi-threaded contexts (A.K.A concurrent situations) due to atomic reference counting
// //     // It's used when data needs to be shared across multiple threads

//     let a = Arc::new(50);
//     let b = Arc::clone(&a);

//     let handle = thread::spawn( move || {
//         println!("b = {}", b);
//     });

//     println!("a = {}", a);
//     handle.join().unwrap();

// //     // // Example E. Arc<T> 

//     let counter = Arc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter = Arc::clone(&counter);
//         let handle  = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;

//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());


// //     // DANGLING Raw Pointer Example: - this is unsafe code, dangling pointers occur when raw pointers are used incorrectly.
// //     // Use safe Rust where compile time checks with (ownership, borrowing, and lifetimes)

    
//     fn create_dangling_pointer() -> *const i32 {
//         let x = 42;
//         &x as *const i32  // returns a pointer to 'x' which is destroyed when the function exits
//     }

//     //fn main() {
//         let ptr = create_dangling_pointer();

//         unsafe {
//             println!("Dereferencing dangling pointer: {}", *ptr) // Undefined behaviour
//         }
//     //}

// //     //  UNSAFE RUST (Unsafe SUPER Powers) 
    
// //     //  -> A Dereference a Rawpointer , B Call an unsafe func/method , C Acces or modify a mutable static variable,
// //     //  D Implement an unsafe trait, E Access fields of a union


// //     // 1. Creating an IMMUTABLE & a MUTABLE raw pointer
     
//         let mut num = 6;

//         let r1 = &raw const num;
//         let r2 = &raw mut num;

//     // Raw pointer with unsafe keyword
    
//         let mut num = 7;
        
//         let r3 = &raw const num;
//         let r4 = &raw mut num;
//         let r5 = &raw mut num;

//         unsafe {
//             println!("r3 is: {}", *r3);
//             println!("r4 is: {}", *r4); 
//          } 
// //     //      BOTH raw pointers are pointing at the same memory spot (data "data races can occur")
// //     //      above, *const i32 and *mut i32 was created = data race! = data in the same memory location 

// //     //      When interacting with C code - you may need raw pointers OR when building a Safe ABSTRACTIONS
// //     //      that the borrow checker does not understand.

   
    
// //    PRESENTATION ENDS!

//      }

// use std::slice;

// // External C function
// extern "C" {
//     fn abs(input: i32) -> i32;
// }

// // A safe abstraction over unsafe code: splitting a mutable slice
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
// }

fn main() {
    // Example 1: Unsafe function
    // unsafe fn dangerous() {
    //     println!("Called an unsafe function.");
    // }

    // unsafe {
    //     dangerous();
    // }

    // // Example 2: Using split_at_mut
    // let mut data = [1, 2, 3, 4, 5, 6];
    // let (left, right) = split_at_mut(&mut data, 3);

    // // Modify slices to show they're mutable and separate
    // left[0] = 20;
    // right[0] = 50;

    // println!("Modified data: {:?}", data);

    // // Example 3: Call to external C function
    // unsafe {
    //     println!("Absolute value of -3 according to C: {}", abs(-3));
    // }

//   //      SLICES - The Slice Type:

//  //      Problem - write a function that takes a string of words separated by spaces and returns the first word it finds in that string.
// //    If the function does not find a space in the string, the whole string must be one word, so the entire string should be returned.

//        // FIRST - lets see how we wld write the signature of this function WITHOUT using slices, to understand the problem that slices will solve:
       
//            fn first_word(s: &String) -> ?

//         the first_word function has a '&String as a parameter. We dont need ownership, so this is fine. But, what should we return?
//        We dont reallyhave a way to talk about part of a string. However, we could return the index of the end of the word, indicated by a space:
       
//        EXAMPLE:

//             fn first_word(s: &String) -> usize {
//                 let bytes = s.as_bytes();

//                 for (i, &item) in bytes.iter().enumerate() {
//                     if item == b' ' {
//                         return i;
//                     }
//                 }

//                 s.len()

//                 ..because we need to go through the string, element by element and check whether a value is a space,
//                 we'll convert our string to an array of bytes using the 'as_bytes' method
        
//                    let bytes = s.as_bytes(); - NEXT , we create an iterator over the array of bytes using the 'iter' method:
//                    for (i, &item) in bytes.iter().enumerate() {}
    

// }       

    
// // EXPLORING INTERIOR mutability with RefCell<T>

// // RefCell<T> is a runtime-checked mutable memory location that enables INTERIOR mutability - a patter where you can mutate data even when you have an 
// // immutable reference to it. It enforces Rust's borrowing rules (either one mutable reference OR multiple references) at runtime instead of compile time.
// // RefCell<T> complements Rc<T> - when both are combined - you get shared ownership WITH mutability.

// // - How it works is use borrow() to get immutable reference (Ref<T>)
// //- Use borrow_mut() to get a mutable reference (RefMut<T>) - if you violate the borrowing rules - it panics at runtime.


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

    

// fn example_refcell() {
    use std::cell::RefCell;

    let x = RefCell::new(42);
    {
        let mut y = x.borrow_mut(); // mutable borrow 
        *y += 1; // borrow ends here
     }
    let z = x.borrow(); // immutable borrow now allowed.
    println!("RefCell value: {}", z);
// }

// // Combining RefCell<T> and Rc<T>

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


//     }
// Treating Smart Pointers like Regular References with the Deref Trait
// - Implementing the Deref trait allows you to customise the behaviour of the 'dereference operator *' operator. By implementing Deref in such a way that a smart pointer can be treated like a regular reference, 
// you can write code that operates on references and use that code with smart ointers too.
//  Let’s first look at how the dereference operator works with regular references. Then we’ll try to define a custom type that behaves like Box<T>, and see why the dereference operator doesn’t work like a reference on our newly defined type. 
// We’ll explore how implementing the Deref trait makes it possible for smart pointers to work in ways similar to references. 
// Then we’ll look at Rust’s deref coercion feature and how it lets us work with either references or smart pointers.

    fn main() {
        let x = 5;
        let y = &x;

        assert_eq!(6, x);
        assert_eq!(6, *y);
    } // using a dereference operator to follow a reference to an i32 value

    // NB - comparing a number and a reference to a number isn't allowed bcs they're 
    // different types, We must use the derefence operator to follow the reference to the value it's 
    // pointing to.

// Using a Box<T> Like a reference

    fn main() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

// Defining our smart pointer using Box<T>, then add ability to use a dereference operator

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    } // Defining a MyBox<T> type

    // we define a struct named 'MyBox' and declare a generic parameter T, bcs we wnat a our type to hold values of any type.
    // The MyBox type is a tuple struct with one element of type T, the MyBox::new function takes one parameter of type T and returns a MyBox instance that holds the value passed in.
    // Adding main to the code above ?

    fn main() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    // Attempting to use MyBox<T> in the same way we usd references and Box<T>
    // The above code wont compile - our MyBox<T> can't be dereferenced bcs we have not implemented that ability on our type. 
    // To enable dereferencing with the '*' operator, we implement the Deref trait.

    // Using Deref trait, this borrows self and returns a reference to the inner data:

    use std::ops::Deref;

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    // Implicit Deref and Coersion on Fn's and Methods
    // Deref Coersion was added to Rust so that programmers writing function & method calls don't
    // dont need to add as many explicit references and dereferences with '&' and '*'
    // The deref coersion feature also lets us write more code that can work for either references or smart pointers.

    fn main() {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    } // Calling hello with a reference to a MyBox<String> value
    // which works bcs of deref coersion



    // COMPARING Lifetimes with Rc<T> used_with RefCell<T> at the sameTime

    // Lifetimes in Rust, Rc<T> and RefCell<T> serve distinct but complementary roles in managing memory safety and data access.

    // 1. Lifetimes's PURPOSE:

    // Lifetimes ensure references are VALID for the duration of use.
    
    // FUNCTIONALITY:
    // - Annotate relationships between references to guarantee they outlive their use
    // - Checked at COMPILE TIME by the borrow checker
    // - Prevent dangling references by enforcing scoping rules
 
    // // EXAMPLE:

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    } // here a' ensures that returned reference lives as the inputs 

    // 2. Rc<T> (Reference Counting)

    // PURPOSE:
    
    // - Enable shared ownership of data (multiple 'owners' of the same value).

    // FUNCTIONALITY:

    // - Tracks reference counts at RUNTIME to deallocate data when all owners are gone.
    // - Provides immutable access to the data it wraps.
    // - Useful for dynamically determining lifetimes (e.g, shared state).

    // EXAMPLE:

    // use std::rc::Rc;
    // let data = Rc::new(42);
    // let clone1 = Rc::clone(&data); // Shared ownership    

    
    // 3. RefCell<T> (INterior Mutability).
    
    // PURPOSE:
    
    // - Allow safe mutation of data even when it appears immutable.

    // FUNCTIONALITY:

    // - Enforces borrowing rules (&T or &mut T) at RUNTIME (panic on violation).
    // - Enables patterns like mutable aliasing, which the borrow checker normally rejects.
    // - Used when mutation needs to occur through an immutable reference.

    // // EXAMPLE:

    // use std::cell::RefCell;
    // let cell = RefCell::new(42);
    // *cell.borrow_mut() += 1; // mutable borrow checked at runtime


    // 4. Combining Rc<T> and RefCell<T> ...Rc<RefCell<T>> allows:

    // - Shared ownership (via Rc<T>) and mutable access (via RefCell<T>).
    // - RUNTIME checks for both reference counting and borrowing rules. 

//     // EXAMPLE:

    // use std::rc::Rc;
    // use std::cell::RefCell;

    // let shared_datta = Rc::new(RefCell)::new(42);
    // let clone = Rc::clone(&shared_datta);
    // *clone.borrow_mut() += 1; // Mutating shared ownership
      


    
}
