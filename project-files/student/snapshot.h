#ifndef SNAPSHOT_H
#define SNAPSHOT_H

#include "sorts.h"

void snapfirst(int threadIndex, char string[], int length);
void snapshot(int threadIndex, char string[], int length);
void snaplast(int threadIndex, char string[], int length);

#endif
