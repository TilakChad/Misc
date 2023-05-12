#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>

#define G_PUSHPOP_DIR_ENV "G_PUSHPOP_DIR"
#define G_PUSHPOP_FILE "/.gpushpop"

int create_intermediate_directories();

int main(int argc, char* argv[]) {
        if (argc < 2)
        {
                fprintf(stderr, "Error usage : %s <dir>\n", argv[0]);
                return -2;
        }

        const char* push_dir = argv[1];
        const char* pushpop_dir = getenv(G_PUSHPOP_DIR_ENV);
        if (!pushpop_dir) {
                // Environment variable not set, so default to using home directory
                pushpop_dir = getenv("HOME");
                if (!pushpop_dir) {
                        fprintf(stderr, "Invalid user home directoy");
                        return -3;
                }
                fprintf(stderr, "using home_directory : %s.\n", pushpop_dir);
        }

        char file_name[256];// improve this, probably use string with length
        char dir_full_path[256] = {0};

        if (realpath(push_dir, dir_full_path) == NULL) {
                fprintf(stderr, "folder doesn't exist");
                return -5;
        }
        strcpy(file_name, pushpop_dir);
        strcat(file_name, G_PUSHPOP_FILE);
        // Search for the dir/.gpushpop file inside the directory if not create it

        int dir_len = strlen(dir_full_path);
        dir_full_path[dir_len] = '\n';

        int fd = open(file_name, O_WRONLY | O_CREAT | O_APPEND /*, Permissions */, S_IRUSR | S_IRGRP | S_IWUSR | S_IWGRP);
        if (fd == -1) {
                perror("open() failed : ");
                return -2;
        }

        // obtain the full path of the file
        write(fd,dir_full_path, strlen(dir_full_path));
        // append a new line to it
        close(fd);
        return 0;
}
