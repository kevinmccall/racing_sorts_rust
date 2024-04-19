#include "sorts.h"
#include <pthread.h>
#include <stdio.h>
#include <string.h>

void snapfirst(int threadIndex, char string[], int length) {
    // TODO:  This function records the first "snapshot" of a thread's
    //        state.  Replace the printf() below with code to send the
    //        main thread the supplied information.
    printf("%3d: %s\n", threadIndex, string);
}

void snapshot(int threadIndex, char string[], int length) {
    // TODO:  This function records "progress" toward the completion of the
    //        sorting algorithm.  Replace the printf() below with code to
    //        send the progress information to the main thread.
    printf("%3d: %s\n", threadIndex, string);
}

void snaplast(int threadIndex, char string[], int length) {
    // TODO:  This function records the completion of the sorting algorithm.
    //        Replace the printf() below with code to send some sort of
    //        sentinal message to the main thread so that it will know that
    //        the sorting algorithm has completed.
    printf("%3d: %s\n", threadIndex, string);
}
