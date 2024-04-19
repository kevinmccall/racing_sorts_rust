CC = gcc CFLAGS = -O - Wall - Wextra - DTHREADED LDFLAGS =
                      -lpthread - lncurses OBJS =
                          bubbleSort.o insertionSort.o selectionSort.o shellSort
                              .o quickSort.o snapshot.o screen.o racer
                              .o

                                  all : racer bubbleSort.o
    : bubbleSort.c sorts.h snapshot.h insertionSort.o
    : insertionSort.c sorts.h snapshot.h selectionSort.o
    : selectionSort.c sorts.h snapshot.h shellSort.o
    : shellSort.c sorts.h snapshot.h quickSort.o
    : quickSort.c sorts.h snapshot.h snapshot.o : snapshot.c screen.o
    : screen.c screen.h racer.o : racer.c sorts.h racer : $(OBJS) clean
    : $(RM) $(OBJS) racer
