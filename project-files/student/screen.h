#ifndef SCREEN_H
#define SCREEN_H

void getTerminalCapabilities(void);
void initScreen(void);
void cleanupScreen(void);

void clearScreen(void);
void gotoXY(int row, int col);
void putCharacterXY(int row, int col, int ch);
void putString(char *string);
void displayStringAt(int row, int col, char *buffer);
int getNumRows();
int getNumCols();

#endif
