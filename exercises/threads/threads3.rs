// threads3.rs
// Finish the `map_with_threads` implementation using message passing!
// Execute `rustlings hint threads3` for hints :)

// I AM NOT DONE

use std::sync::mpsc;
use std::thread;

/// Study the following code, it uses message passing to send
/// the elements in `input` to a few worker threads which then
/// process individual elements using `fun` and send the result
/// back to the main thread.
///
/// Unfortunately, this code has a critical bug which triggers our assertion 
/// in main: While the result in the returned vector contains all the right elements,
/// they now appear in random order :(.
/// Can you fix it?
fn map_with_threads<F>(mut input: Vec<usize>, fun: F) -> Vec<usize> 
where
    F: Copy + Send + 'static,
    F: Fn(usize) -> usize
{
    // We create one channel (main) to send results from threads 
    // back to this function:
    let (tx_main, rx_main) = mpsc::channel();

    // This vector stores the handles to send messages to the worker threads:
    let mut send_to: Vec<_> = Default::default();

    // Spawn a few workers that will execute `fun` on input elements for us:
    for _tid in 0..4 {
        let (tx, rx) = mpsc::channel();
        let to_main = tx_main.clone();
        send_to.push(tx);

        thread::spawn(move || {
            loop {
                let msg = rx.recv();
                match msg {
                    // We got an element, apply `fun` and send back the result to the main-thread
                    Ok(value) => to_main.send(fun(value)).unwrap(),
                    // End the thread (we get an error when we try to receive on a channel 
                    // which was destroyed when we exited `map_with_threads`)
                    Err(_) => break,
                }
            }
        });
    }

    // Send the individual elements to the worker threads for processing
    for (idx, element) in input.iter().enumerate() {
        // We make sure to use all worker threads in a round-robin fashion (idx % 4)
        send_to[idx % 4].send(*element).unwrap();
    }

    // Retrieve results and store them in the input
    for idx in 0..input.len() {
        let received = rx_main.recv().unwrap();
        input[idx] = received;
    }

    // We return the (now hopefully modified) input as our output
    input
}


fn main() {
    let elements = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20];
    // We use the (single-threaded) map() function from the standard library to 
    // generate the expected result for us:
    let expected_result = elements.iter().map(|x| x*2).collect::<Vec<usize>>();
    assert_eq!(map_with_threads(elements, |x| x*2), expected_result);
}