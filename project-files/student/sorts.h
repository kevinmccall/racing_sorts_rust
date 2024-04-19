#ifndef SORTS_H
#define SORTS_H

#define MAX_STRING_LENGTH 128

typedef struct SortMessage {
    int threadIndex;
    char data[MAX_STRING_LENGTH + 1];
    int length;
} SortMessage;

void *bubbleThread(void *arg);
void *insertionThread(void *arg);
void *selectionThread(void *arg);
void *shellThread(void *arg);
void *quickThread(void *arg);

typedef struct SortRoutineDescriptor {
    char *sortName;
    void *(*routine)(void *arg);
} SortRoutineDescriptor;

#endif
