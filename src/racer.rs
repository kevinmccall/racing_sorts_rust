use std::alloc::System;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;
use std::u8;

/// I am borrowing some sorts from the internet: https://www.kirillvasiltsov.com/writing/sorting-algorithms-in-rust/


// pub trait Sort<T> {
//     fn sort(data: [T], sender: Sender<Arc<Mutex<[T]>>>) -> [T];
// }

// pub enum Sorts<T: Ord + Clone> {
//     RegularSort(fn([T]) -> [T])
// }



struct SortRunner<T: PartialOrd> {
    data: Arc<Mutex<Vec<T>>>,
    // TODO make this have a boolean in case of early wakeups
    condvar: Arc<Condvar>,
    id: u8,
}

impl<T: PartialOrd> SortRunner<T> {
    fn new(data: Vec<T>, id: u8) -> Self {
        SortRunner {
            data: Arc::new(Mutex::new(data)),
            condvar: Arc::new(Condvar::new()),
            id
        }
    }

    fn bubble_sort(&self, sender: Sender<SortMessage<T>>) {
        let data = self.data.lock().unwrap();
        let message = SortMessage {
            id: self.id,
            data: self.data.clone(),
            condvar: self.condvar.clone()
        };
        sender.send(message).unwrap();
        let mut data = self.condvar.wait(data).unwrap();


        for i in 0..data.len() {
            for j in i+1..data.len() {
                if data[i] > data[j] {
                    data.swap(i, j);
                    let message = SortMessage {
                        id: self.id,
                        data: self.data.clone(),
                        condvar: self.condvar.clone()
                    };
                    sender.send(message).unwrap();
                    data = self.condvar.wait(data).unwrap();
                }
            }
        }
    }
}


pub fn sort_manager() {
    let (sender, receiver) = mpsc::channel();

    let num_sorters = 3;

    for i in 0..num_sorters {
        let sender = sender.clone();
        thread::spawn(move || {
            let my_data = "When you shave and get that #extrainch".bytes().collect();
            let sort_runner = SortRunner::new(my_data, i);
            sort_runner.bubble_sort(sender);
        });
    }
    drop(sender);

    while let Ok(message) = receiver.recv() {
        let data = message.data.lock().unwrap();
        let display = std::str::from_utf8(&data[..]).unwrap_or("error");
        println!("{}: {:?}", message.id, display);
        message.condvar.notify_one();
    }
}

struct SortMessage<T> {
    id: u8,
    data: Arc<Mutex<Vec<T>>>,
    condvar: Arc<Condvar>
}

// // TODO:  You will need some sort of global buffer that is shared by the
// //        main thread and your sorting threads.  This buffer will enable
// //        the sorting threads to send messages to the main thread, and allow
// //        the main thread to receive and process those messages.
// //
// //        For this you must use a bounded (i.e., fixed-size) buffer.  You
// //        may use/modify the examples we've discussed in class for this task.





// int main(int argc, char* argv[]) {
//     char*        filename  = parseArgs(argc, argv, "f:");
//     char         string[MAX_STRING_LENGTH + 1];
//     int          stringLength;
//     SortMessage* threadArgs;
//     pthread_t*   threads;
//     int          i;

//     readString(filename, string);
//     stringLength = strlen(string);

//     // Adjust away any of the arguments consumed by parseArgs()
//     argc -= optind;
//     argv += optind;

//     // TODO:  Make a call to your screen library to load information about
//     //        the current terminal type.



//     // TODO:  Check to make sure that the number of requested sorts doesn't
//     //        exceed the number of rows on the screen.


//     // TODO:  Check to make sure that the string your are to sort isn't longer
//     //        than the screen is wide.  Remember to include an offset for
//     //        the sort names.

//     // TODO:  Dynamically allocate two arrays, one for the pthread_t values
//     //        and another for SortMessages.
//     threads    = NULL;
//     threadArgs = NULL;


//     // TODO:  Loop over the remaining command line arguments -- these should
//     //        be names of sorting algorithms.  For each command line argument
//     //        compare the string to the sorting table declared above.  If
//     //        you find a match, start up a new thread using the function
//     //        pointer in the sorting table.  Be sure to populate the
//     //        threadArgs array entry with appropriate information and pass
//     //        that as an argument to the newly created thread's main function.
//     //
//     //        If the user enters an invalid sorting algorithm name, you can
//     //        simply terminate the program here.
//     //



//     // TODO:  initialize and clear the screen



//     // TODO:  Display the sort strings on their correspondign lines.
//     //        Note that this should happen only once.



//     // TODO:  Receive messages from the threads and update the display
//     //        accordingly.



//     // TODO:  Cleanup the screen



//     // TODO:  Make sure that you release any dynamically allocated resources
//     //        you still hold.
//     free(threads);
//     free(threadArgs);

//     return 0;
// }

// static void readString(char* filename, char* string) {
//     FILE* inputFile = fopenWrapper(filename, "r");
//     bool  okay      = true;
//     int   i         = 0;
//     char  c;

//     while ( okay && ((c = getc(inputFile)) != EOF) && (c != '\n') ) {
//         if (i < MAX_STRING_LENGTH) {
//             string[i++] = c;
//         } else {
//             okay = false;
//         }
//     }
//     string[i] = '\0';

//     if (!okay) {
//         fprintf(stderr, "Input too big.  (Buffer holds %d characters)\n",
//                          MAX_STRING_LENGTH);
//         exit(5);
//     }
//     fclose(inputFile);
// }

// static char* parseArgs(int argc, char* argv[], char* opts) {
//     char* filename = NULL;
//     char  option;

//     while ( (option = getopt(argc, argv, opts)) != -1) {
//         switch (option) {
//         case 'f':
//             filename = optarg;
//             break;

//         case '?':
//             usage(argv[0]);
//             exit(3);
//             break;
//         }
//     }

//     if (filename == NULL) {
//         usage(argv[0]);
//         exit(4);
//     }

//     return filename;
// }

// static FILE* fopenWrapper(char* filename, char* mode) {
//     FILE* file = fopen(filename, mode);

//     if (file == NULL) {
//         perror(filename);
//         exit(5);
//     }

//     return file;
// }

// static void usage(char* command) {
//     int i;

//     fprintf(stderr, "usage: %s -f <filename> <sortAlgorithm> "
//                     "[<sortAlgorithm> ...]\n", basename(command));
//     fprintf(stderr, "Where <sortAlgorithm> is one of:\n");

//     for (i = 0; knownSorts[i].sortName != NULL; ++i) {
//         fprintf(stderr, "    %s\n", knownSorts[i].sortName);
//     }
// }
