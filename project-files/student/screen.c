/*
 * screen.c
 *
 * Interface to termcap screen manipulation routines.
 *
 * Dr. Edward Pekarek Jr.
 * Dr. Andrew R. Dalton
 * Dr. William Kreahling
 */
#include "screen.h"
#include <curses.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/ioctl.h>
#include <term.h>
#include <termios.h>
#include <unistd.h>

/* Global Constants */
#define AREA_SIZE 1024
#define BUFFER_SIZE 1024
#define NUM_SIGNALS (SIGUSR2 + 1)

/* Typedefs */
typedef void (*sighandler_t)(int);
typedef struct termios termios_t;

/* Global file-private data */
static int numLines = 0; /* li */
static int numCols = 0;  /* co */
static char area[AREA_SIZE];
static char *areaPointer = area; /* ap */
static char *clearToEOLString;   /* ce */
static char *clrscrString;       /* cl */
static char *cursorMotionString; /* cm */
static char *termInitString;     /* ti */
static termios_t backupTermios;
static sighandler_t backSignalHandlers[NUM_SIGNALS] = {0};
static const int interestingSignals[] = {
    SIGHUP,  /* Hangup */
    SIGINT,  /* Interrupt */
    SIGQUIT, /* Quit */
    SIGILL,  /* Invalid instruction */
    SIGTRAP, /* Trace trap */
    SIGABRT, /* End process */
    SIGFPE,  /* Arithmetic exception - divide by 0 */
    SIGBUS,  /* Specification exception */
    SIGSEGV, /* Segmentation violation */
    SIGSYS,  /* Invalid parameter to system call */
    SIGTERM, /* Software termination signal */
    SIGTSTP, /* Interactive stop */
    SIGCONT, /* Continue if stopped */
    -1       /* End-of-list flag */
};

/* Private function prototypes */
static void signalHandler(int signalValue);
static int outc(int ch);

/*
 * Queries the termcap database and extracts entries
 * associated with cursor movement and screen size.
 */
void getTerminalCapabilities(void) {
    char *terminalType = NULL;
    char buffer[BUFFER_SIZE];
    int result = 5;
    int errorFlag = 0;

    // TODO:  Get the type of the terminal and set terminalType to that
    //        value.  The terminal type is specified using the TERM environment
    //        variable.  Do not hard code this!  Find and use a function to
    //        get the value of an environment variable.

    if (NULL == terminalType) {
        fprintf(stderr, "No TERM environment variable defined.\n");
        exit(1);
    }

    // TODO:  Use the function tgetent() to read the termcap database into
    //        buffer.  Catch its return value in result.

    switch (result) {
    case 1:
        /* Ok */
        break;
    case 0:
        fprintf(stderr, "No termcap entry for '%s'\n", terminalType);
        exit(2);
    case -1:
        fprintf(stderr, "Can't open termcap file\n");
        exit(3);
    default:
        fprintf(stderr, "Unexpected value from tgetent(): %d\n", result);
        exit(4);
    }

    // TODO:  Use the function 'tgetnum()' to read the number of lines on the
    //        screen ("li"), the number of columns on the screen ("co") from
    //        the termcap database.  Save those values in the corresponding
    //        global static above.  If the function returns a negative value,
    //        an error has occured.  Check for this condition and print an
    //        appropriate error message.

    // TODO:  Use the function 'tgetstr()' to read the clear screen string
    //        ("cl"), the clear to end of line string ("ce"), and cursor
    //        motion string ("cm"), and the terminal init string ("ti")
    //        from the termcap database.  Save those values in the
    //        corresponding static variables variables above.  If the function
    //        returns a NULL value, an error has occured.  Check for this
    //        condition and print an appropriate error message
}

/*
 * Initialize the terminal driver and terminal
 */
void initScreen(void) {
    // TODO:  Use ioctl() to get terminal driver state and change it.
    //        (Save original for cleanup.)  Possibly send initialization
    //        strings.  Once terminal interface changes restoration is
    //        required; catching signals is now needed -- see the list
    //        of interesting signals above.
}

/*
 * Undo what initScreen() did.
 */
void cleanupScreen(void) {
    // TODO:  Undo what initScreen() did.  Possibly send terminal "cleanup"
    //        string(s).  Use ioctl() to restore terminal driver state.
    //        Possibly reset signal handlers to their previous values.
}

/*
 * Clears the screen
 */
void clearScreen(void) {
    tputs(clrscrString, numLines, outc);
    fflush(stdout);
}

/*
 * Places the cursor at the given row and column position
 */
void gotoXY(int row, int col) {
    char *where;

    where = tgoto(cursorMotionString, col, row);
    tputs(where, numLines, outc);
    fflush(stdout);
}

/*
 * Writes a character to the specified location
 */
void putCharacterXY(int row, int col, int ch) {
    // TODO:  Fill in the implementation of this function
}

/*
 * Function used to print a string
 */
void putString(char *string) {
    // TODO:  Fill in the implementation of this function.  Use
    //        the tputs() function to do so.
}

/*
 * Displays the specified string at the given row and
 * column position.
 */
void displayStringAt(int row, int col, char *buffer) {
    // TODO:  Fill in the implementation of this function.
}

/*
 * Gets the number of rows on the screen.
 */
int getNumRows() { return numLines; }

/*
 * Gets the number of cols on the screen.
 */
int getNumCols() { return numCols; }

/*
 * Signal handler that resets the terminal interface
 * and exits the program.
 */
static void signalHandler(int signalValue) {
    cleanupScreen();
    /* would like to make sure the cursor is back
       to the left margin, but it's unsafe to call
       such a function in the signal handler */
    exit(130);
}

/*
 * Function passed to tputs() to print a character
 * to the output stream.
 */
static int outc(int ch) { return putchar(ch); }
