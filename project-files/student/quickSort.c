/*
 * A quicksort implementation downloaded from:
 *
 *     http://www.comp.dit.ie/rlawlor/Alg_DS/sorting/quickSort.c
 *
 */
#include "snapshot.h"
#include "sorts.h"
#include <stdbool.h>
#include <stdio.h>

static int partition(char a[], int l, int r, SortMessage *sortArgs);
static void quickSort(char a[], int l, int r, SortMessage *sortArgs);

void *quickThread(void *arg) {
    SortMessage *sortArgs = (SortMessage *)arg;

    snapfirst(sortArgs->threadIndex, sortArgs->data, sortArgs->length);
    quickSort(sortArgs->data, 0, sortArgs->length - 1, sortArgs);
    snaplast(sortArgs->threadIndex, sortArgs->data, sortArgs->length);

    return NULL;
}

static void quickSort(char a[], int l, int r, SortMessage *sortArgs) {
    if (l < r) {
        // divide and conquer
        int j = partition(a, l, r, sortArgs);

        quickSort(a, l, j - 1, sortArgs);
        quickSort(a, j + 1, r, sortArgs);
    }
}

static int partition(char a[], int l, int r, SortMessage *sortArgs) {
    char t;
    char pivot = a[l];
    int i = l;
    int j = r + 1;

    while (true) {
        // Fixed 3/17/14 by Tyler Allen
        // do { ++i; } while (a[i]<= pivot && i <= r);
        do {
            ++i;
        } while (i <= r && a[i] <= pivot);
        do {
            --j;
        } while (a[j] > pivot);

        if (i >= j) {
            break;
        } /* Dr. Dalton did not write this code */

        t = a[i];
        a[i] = a[j];
        a[j] = t;
        snapshot(sortArgs->threadIndex, sortArgs->data, sortArgs->length);
    }

    t = a[l];
    a[l] = a[j];
    a[j] = t;
    snapshot(sortArgs->threadIndex, sortArgs->data, sortArgs->length);

    return j;
}
