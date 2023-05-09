#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <fcntl.h>
#include <string.h>

typedef enum EPopType { Remove, Keep } EPopType;

typedef struct PopEntryInfo {
        int offset;
        int length;
} PopEntryInfo;

typedef struct PopInfo {
        EPopType type;
        PopEntryInfo prev;
        PopEntryInfo curr;
} PopInfo;


#define G_PUSHPOP_DIR_ENV "G_PUSHPOP_DIR"
#define G_PUSHPOP_FILE "/.gpushpop"

#define Bytes(x) (x)
#define KiloBytes(x) (1024 * Bytes(x))
#define MegaBytes(x) (1024 * KiloBytes(x))
#define GigaBytes(x) (1024 * MegaBytes(x))

#define MAX_FILE_SIZE Bytes(1024)

int main(int argc, char *argv[]) {
        const char *pushpop_dir = getenv(G_PUSHPOP_DIR_ENV);
        if (!pushpop_dir) {
                // Environment variable not set, so default to using home directory
                pushpop_dir = getenv("HOME");
                if (!pushpop_dir) {
                        fprintf(stderr, "Invalid user home directoy");
                        return -3;
                }
                fprintf(stderr, "using home_directory : %s.\n", pushpop_dir);
        }

        // The truncate() function could be used here instead of rewriting everything again
        // Read the whole file at once and start scanning

        char buffer[MAX_FILE_SIZE];

        char file_name[512] = {0};
        strcpy(file_name, pushpop_dir);
        strcat(file_name, G_PUSHPOP_FILE);

        int fd = open(file_name, O_RDWR | O_CREAT);
        if (fd == -1) {
                perror("open() failed : ");
                return -1;
        }

        int read_bytes = read(fd,buffer,MAX_FILE_SIZE);
        if (read_bytes == MAX_FILE_SIZE) {
                fprintf(stderr, "File buffer too short, current length : %d.", read_bytes);
                return -5;
        }

        buffer[read_bytes] = '\0';

        // scan the file fully while terminating on the new lines
        PopInfo pop_action;
        pop_action.type = Remove;

        pop_action.curr.offset = 0;
        pop_action.curr.length = 0;

        pop_action.prev.offset = 0;
        pop_action.prev.length = 0;

        int offset = 0;
        while (offset < read_bytes) {
                if (buffer[offset] == '\n') { // TODO :: Fix for windows
                        int nlength     = offset - pop_action.prev.offset;
                        pop_action.prev = pop_action.curr;
                        pop_action.prev.length = nlength;

                        pop_action.curr.offset = offset;
                        pop_action.curr.length = 0; // not determined yet
                }
                offset = offset + 1;
        }

        // Handle the last offset by marking the current offset
        pop_action.curr.length = offset - pop_action.curr.offset - 1;

        fprintf(stderr, "Displaying the last pushed stack entry of length %d : \n", pop_action.curr.length);
        fprintf(stderr, "%.*s", pop_action.curr.length, &buffer[pop_action.curr.offset]);


        fprintf(stderr, "Displaying the last pushed stack entry of length %d : \n", pop_action.prev.length);
        fprintf(stderr, "%.*s", pop_action.prev.length, &buffer[pop_action.prev.offset]);

        // TODO :: The below case doesn't apply for new file
        // FIX  ::

        ftruncate(fd,pop_action.prev.offset + 1); // Since the offset is calculated just where there is new line
        close(fd);

        return 0;
}
