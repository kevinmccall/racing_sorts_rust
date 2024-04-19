#include "screen.h"
#include "sorts.h"
#include <libgen.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define LABEL_PADDING 11

static char *parseArgs(int argc, char *argv[], char *opts);
static void readString(char *filename, char *string);
static FILE *fopenWrapper(char *filename, char *mode);
static void usage(char *command);

static SortRoutineDescriptor knownSorts[] = {
    // TODO:  Fill in this table with a mapping of sort name to the funciton
    //        implementing the sorting algorithm.  The { NULL, NULL } entry
    //        must remain in the table and it must be the last entry in the
    //        table.
    {NULL, NULL} // MUST be last entry
};

// TODO:  You will need some sort of global buffer that is shared by the
//        main thread and your sorting threads.  This buffer will enable
//        the sorting threads to send messages to the main thread, and allow
//        the main thread to receive and process those messages.
//
//        For this you must use a bounded (i.e., fixed-size) buffer.  You
//        may use/modify the examples we've discussed in class for this task.

int main(int argc, char *argv[]) {
    char *filename = parseArgs(argc, argv, "f:");
    char string[MAX_STRING_LENGTH + 1];
    int stringLength;
    SortMessage *threadArgs;
    pthread_t *threads;
    int i;

    readString(filename, string);
    stringLength = strlen(string);

    // Adjust away any of the arguments consumed by parseArgs()
    argc -= optind;
    argv += optind;

    // TODO:  Make a call to your screen library to load information about
    //        the current terminal type.

    // TODO:  Check to make sure that the number of requested sorts doesn't
    //        exceed the number of rows on the screen.

    // TODO:  Check to make sure that the string your are to sort isn't longer
    //        than the screen is wide.  Remember to include an offset for
    //        the sort names.

    // TODO:  Dynamically allocate two arrays, one for the pthread_t values
    //        and another for SortMessages.
    threads = NULL;
    threadArgs = NULL;

    // TODO:  Loop over the remaining command line arguments -- these should
    //        be names of sorting algorithms.  For each command line argument
    //        compare the string to the sorting table declared above.  If
    //        you find a match, start up a new thread using the function
    //        pointer in the sorting table.  Be sure to populate the
    //        threadArgs array entry with appropriate information and pass
    //        that as an argument to the newly created thread's main function.
    //
    //        If the user enters an invalid sorting algorithm name, you can
    //        simply terminate the program here.
    //

    // TODO:  initialize and clear the screen

    // TODO:  Display the sort strings on their correspondign lines.
    //        Note that this should happen only once.

    // TODO:  Receive messages from the threads and update the display
    //        accordingly.

    // TODO:  Cleanup the screen

    // TODO:  Make sure that you release any dynamically allocated resources
    //        you still hold.
    free(threads);
    free(threadArgs);

    return 0;
}

static void readString(char *filename, char *string) {
    FILE *inputFile = fopenWrapper(filename, "r");
    bool okay = true;
    int i = 0;
    char c;

    while (okay && ((c = getc(inputFile)) != EOF) && (c != '\n')) {
        if (i < MAX_STRING_LENGTH) {
            string[i++] = c;
        } else {
            okay = false;
        }
    }
    string[i] = '\0';

    if (!okay) {
        fprintf(stderr, "Input too big.  (Buffer holds %d characters)\n",
                MAX_STRING_LENGTH);
        exit(5);
    }
    fclose(inputFile);
}

static char *parseArgs(int argc, char *argv[], char *opts) {
    char *filename = NULL;
    char option;

    while ((option = getopt(argc, argv, opts)) != -1) {
        switch (option) {
        case 'f':
            filename = optarg;
            break;

        case '?':
            usage(argv[0]);
            exit(3);
            break;
        }
    }

    if (filename == NULL) {
        usage(argv[0]);
        exit(4);
    }

    return filename;
}

static FILE *fopenWrapper(char *filename, char *mode) {
    FILE *file = fopen(filename, mode);

    if (file == NULL) {
        perror(filename);
        exit(5);
    }

    return file;
}

static void usage(char *command) {
    int i;

    fprintf(stderr,
            "usage: %s -f <filename> <sortAlgorithm> "
            "[<sortAlgorithm> ...]\n",
            basename(command));
    fprintf(stderr, "Where <sortAlgorithm> is one of:\n");

    for (i = 0; knownSorts[i].sortName != NULL; ++i) {
        fprintf(stderr, "    %s\n", knownSorts[i].sortName);
    }
}
