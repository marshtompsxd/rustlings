// threads2.rs
// Finish the `map_with_threads` implementation using Mutex and Arc! 
// Execute `rustlings hint threads2` for hints :)

// I AM NOT DONE

use std::thread;
use std::sync::Mutex;
use std::sync::Arc;

/// A concurrent Vec::map function:
/// https://doc.rust-lang.org/std/iter/struct.Map.html 
///
/// Your task is to finish this function so it applies `fun`
/// on `input` in-place and in-parallel.
///
/// The easiest way to achieve this is if you change the type for argument `input`,
/// and also the return type of the function `map_with_threads`.
/// Remember that you need to "prove" these two things to the rust type-system:
/// a) Your `input` needs to be alive (at least) as long as all threads that have access to it.
/// b) Accessing the invidivual elements of input should ensure mutual exclusion.
///
/// You'll probably also have to rewrite the
/// `input[i] = fun(input[i]);` part, to account for your new
/// types. 
fn map_with_threads<F>(input: Vec<usize>, fun: F) -> Vec<usize> 
where
    F: Copy + Send + 'static,
    F: Fn(usize) -> usize
{
    assert!(input.len() % 4 == 0, "Input length is divisible by four");
    let per_thread_chunk = input.len() / 4;
    let mut handles: Vec<_> = Default::default();

    // We spawn 4 threads to work on our `input` in parallel:
    for tid in 0..4 {
        // We give each thread read access to input by cloning it:
        // (mini hint: If the compiler tells you that this should be `mut`, go for it -- but careful
        // it is unfortunately misleading you... The code will compile, but fail the assert in main:
        // that's because it ends up copying the vector 4 times and each thread would work
        // on it's own (private) copy and the original `input` would never reflect
        // the changes -- that's not what we want for our `map_with_threads`)
        let input = input.clone();

        // We spawn the actual thread (which returns a handle we later use to
        // wait until it has completed)
        let handle = thread::spawn(move || {
            // The thread decides on which part of `input` it needs to work
            // on based on its thread id (`tid`)
            let begin = tid*per_thread_chunk;
            let end = (tid+1)*per_thread_chunk;
            for i in begin..end {
                // Each thread goes and applies `fun` on it's sub-region
                // of the input vector. It saves the result by overriding
                // the old element `input`:
                input[i] = fun(input[i]);
            }
        });
        handles.push(handle);
    }

    // We wait until all threads have completed by calling
    // `join` on each handle:
    for handle in handles {
        handle.join().unwrap();
    }

    // We return the (now hopefully modified) input as our output
    input
}

fn main() {
    // If you change the type signature of `map_with_threads`,
    // you'll also have to initialize the `elements` variable differently:
    let elements = vec![1,2,3,4];
    assert_eq!(map_with_threads(elements, |x| x*2), vec![2, 4, 6, 8]);
}