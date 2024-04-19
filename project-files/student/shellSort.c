/*
 * A simple selection sort of an array of characters into non-decreasing
 * order.
 * A shell sort of an array of characters into non-decreasing order.
 *
 * This is actually Knuth's "diminishing increment sort".  See D. Knuth,
 * "The Art of Computer Programming", Volume 3, Sorting and Searching,
 * Addison-Wesley.
 */
#include "snapshot.h"
#include "sorts.h"
#include <stdio.h>

// Knuth's recommended increments
static int h[8] = {3280, 1093, 364, 121, 40, 13, 4, 1};
static int smax = 7;

void *shellThread(void *arg) {
    SortMessage *sortArgs = (SortMessage *)arg;
    int s;

    snapfirst(sortArgs->threadIndex, sortArgs->data, sortArgs->length);

    for (s = 0; s <= smax; s++) {
        int step = h[s];
        int j;

        for (j = step; j < sortArgs->length; j++) {
            int i = j - step;
            char k = sortArgs->data[j];

            while ((i >= 0) && (sortArgs->data[i] > k)) {
                sortArgs->data[i + step] = sortArgs->data[i];
                i -= step;
                snapshot(sortArgs->threadIndex, sortArgs->data,
                         sortArgs->length);
            }
            sortArgs->data[i + step] = k;

            snapshot(sortArgs->threadIndex, sortArgs->data, sortArgs->length);
        }
    }
    snaplast(sortArgs->threadIndex, sortArgs->data, sortArgs->length);

    return NULL;
}
