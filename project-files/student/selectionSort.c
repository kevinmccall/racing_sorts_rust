/*
 * A simple selection sort of an array of characters into non-decreasing
 * order.
 */
#include "snapshot.h"
#include "sorts.h"
#include <stdio.h>

void *selectionThread(void *arg) {
    SortMessage *sortArgs = (SortMessage *)arg;
    int i;

    snapfirst(sortArgs->threadIndex, sortArgs->data, sortArgs->length);

    for (i = 0; i < sortArgs->length - 1; i++) {
        int j;
        int k = i;
        char temp;

        for (j = i + 1; j < sortArgs->length; j++) {
            if (sortArgs->data[k] > sortArgs->data[j]) {
                k = j;
            }
        }
        temp = sortArgs->data[i];
        sortArgs->data[i] = sortArgs->data[k];
        sortArgs->data[k] = temp;

        snapshot(sortArgs->threadIndex, sortArgs->data, sortArgs->length);
    }

    snaplast(sortArgs->threadIndex, sortArgs->data, sortArgs->length);

    return NULL;
}
