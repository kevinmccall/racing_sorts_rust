/*
 * A simple insertion sort of a string.
 */
#include "snapshot.h"
#include "sorts.h"
#include <stdio.h>

void *insertionThread(void *arg) {
    SortMessage *sortArgs = (SortMessage *)arg;
    int nsorted;

    snapfirst(sortArgs->threadIndex, sortArgs->data, sortArgs->length);

    for (nsorted = 1; nsorted < sortArgs->length; nsorted++) {
        // Now elements sortArgs->data[0]...sortArgs->data[nsorted-1] form
        // an ordered list. Insert sortArgs->data[nsorted] into it.
        char c = sortArgs->data[nsorted];
        int hole = nsorted;

        while ((hole > 0) && (sortArgs->data[hole - 1] > c)) {
            sortArgs->data[hole] = sortArgs->data[hole - 1];
            --hole;
            snapshot(sortArgs->threadIndex, sortArgs->data, sortArgs->length);
        }

        // Hole is now where the thing goes: drop it in.
        sortArgs->data[hole] = c;
        snapshot(sortArgs->threadIndex, sortArgs->data, sortArgs->length);
    }
    snaplast(sortArgs->threadIndex, sortArgs->data, sortArgs->length);

    return NULL;
}
