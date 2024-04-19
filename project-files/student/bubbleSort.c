/*
 * A bubble sort of an array of characters into non-decreasing order.
 * Many variations are called bubble-sort.  This is but one of them.
 */
#include "snapshot.h"
#include "sorts.h"
#include <stdbool.h>
#include <stdio.h>

void *bubbleThread(void *arg) {
    SortMessage *sortArgs = (SortMessage *)arg;
    int last = sortArgs->length;
    bool sorted = false;

    snapfirst(sortArgs->threadIndex, sortArgs->data, sortArgs->length);

    while (!sorted) {
        int i;

        sorted = true;
        last--;

        for (i = 0; i < last; i++) {
            if (sortArgs->data[i] > sortArgs->data[i + 1]) {
                char temp = sortArgs->data[i];
                sortArgs->data[i] = sortArgs->data[i + 1];
                sortArgs->data[i + 1] = temp;
                sorted = false;

                snapshot(sortArgs->threadIndex, sortArgs->data,
                         sortArgs->length);
            }
        }
    }

    snaplast(sortArgs->threadIndex, sortArgs->data, sortArgs->length);

    return NULL;
}
