#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <fcntl.h>
#include <string.h>
#include <stdbool.h>

#define MAX_ENTRY_COUNT 32

#define G_PUSHPOP_DIR_ENV "G_PUSHPOP_DIR"
#define G_PUSHPOP_FILE "/.gpushpop"

#define Bytes(x) (x)
#define KiloBytes(x) (1024 * Bytes(x))
#define MegaBytes(x) (1024 * KiloBytes(x))
#define GigaBytes(x) (1024 * MegaBytes(x))

#define MAX_FILE_SIZE Bytes(1024)


typedef struct PopEntryInfo {
        int offset;
        int length;
} PopEntryInfo;

#define BITS(x) (1 << (x)) 

typedef enum Flags {
	Keep    = BITS(0),
	Remove  = BITS(1),
	List    = BITS(2),
	Empty   = BITS(3),
	ShowNth = BITS(4)
} Flags;

typedef struct PopInfo {
	Flags        flags; 
        int          entry_count;
	PopEntryInfo pop_entries[32]; 
} PopInfo;

// Function declarations 
Flags parse_flags(const char** argv); 
bool  index_files(PopInfo* pop_data, int fp); 
bool  list_files(PopInfo* pop_data);
void  show_last_entry(PopInfo* pop_data);
void  pop_last_entry(PopInfo* pop_data, int fd); 
void  show_nth_entry(PopInfo* pop_data, int n);  // show nth entry from the last 

static char buffer[MAX_FILE_SIZE];

int main(int argc, char *argv[]) {
        const char *pushpop_dir = getenv(G_PUSHPOP_DIR_ENV);
        if (!pushpop_dir) {
                // Environment variable not set, so default to using home directory
                pushpop_dir = getenv("HOME");
                if (!pushpop_dir) {
                        fprintf(stderr, "Invalid user home directoy");
                        return -3;
                }
                // fprintf(stderr, "using home directory as the base directory : %s.\n", pushpop_dir);
        }

        // The truncate() function could be used here instead of rewriting everything again
        // Read the whole file at once and start scanning

        char file_name[512];
        strcpy(file_name, pushpop_dir);
        strcat(file_name, G_PUSHPOP_FILE);

        int fd = open(file_name, O_RDWR | O_CREAT);
        if (fd == -1) {
                perror("open() failed : ");
                return -1;
        }
	
        // scan the file fully while terminating on the new lines
        PopInfo pop_data;
	pop_data.flags = parse_flags((const char**)argv);
	
	if (pop_data.flags & Empty) {
		ftruncate(fd, 0);
		return 0;
	}

	index_files(&pop_data,fd);

	// arrays of functions
	// handler_fn_t funcs[] = ; 
	
	if (pop_data.flags & List) 
		list_files(&pop_data); 
	if (pop_data.flags & Keep) 
		show_last_entry(&pop_data);
	if (pop_data.flags & ShowNth)
		show_nth_entry(&pop_data, 3);
	if (pop_data.flags & Remove && !(pop_data.flags & List)) {
	        show_last_entry(&pop_data); 
		pop_last_entry(&pop_data, fd);
	}
        close(fd);
	return 0;
}

Flags parse_flags(const char** argv) {
	Flags flag = Remove;

	// TODO :: Fix the files orderings :
	for (const char** ptr = argv + 1; *ptr; ptr = ptr + 1) {
		if (!strcmp(*ptr, "-e"))
			flag |= Empty;
		else if (!strcmp(*ptr,"-k")) {
			flag |= Keep;
			flag &= ~Remove; 
		}
		else if (!strcmp(*ptr,"-l")) {
			flag |= List;
		}
		else {
			fprintf(stderr, "Invalid arguments passed : Accepted flags [-k] [-e] [-l] \n");
			fprintf(stderr, "\n-k : keep the content of the stack");
			fprintf(stderr, "\n-l : list the content of the stack");
			fprintf(stderr, "\n-e : make the stack empty");  
			exit(-2); 
		}
	}
	return flag; 
}

bool  index_files(PopInfo* pop_data, int fd) {
	
	int read_bytes = read(fd,buffer,MAX_FILE_SIZE);

	if (read_bytes == MAX_FILE_SIZE) {
                fprintf(stderr, "File buffer too short, current length : %d.", read_bytes);
                exit(-5);
        }
	
        buffer[read_bytes] = '\0';

	int offset                      = 0;
	pop_data->pop_entries[0].offset = 0; 
	pop_data->entry_count           = 0;
	
	while(offset < read_bytes) {
	    	if (buffer[offset] == '\n') { // TODO :: Handle for win32 which contains carriage returns
			pop_data->entry_count = pop_data->entry_count + 1;
			pop_data->pop_entries[pop_data->entry_count - 1].length = offset - pop_data->pop_entries[pop_data->entry_count - 1].offset;
			// we could've just count the occurence of new lines and stored them into array directly
			pop_data->pop_entries[pop_data->entry_count].offset = offset + 1; 
			pop_data->pop_entries[pop_data->entry_count].length = 0; 

		}
		offset = offset + 1; 
	}

	// Index the last entry
	// \n is left as trailing newline with no practical purpose
	// Discard the last entry 
}

bool  list_files(PopInfo* pop_data) {
	printf("Listing the stack : \n");
	for (int i = pop_data->entry_count - 1; i >=0 ; --i) {
		printf("%d. %.*s\n",pop_data->entry_count - i, pop_data->pop_entries[i].length, buffer + pop_data->pop_entries[i].offset);  
	}
}

void show_last_entry(PopInfo* pop_data) {
	if (!pop_data->entry_count)
		return;
	PopEntryInfo* info = &pop_data->pop_entries[pop_data->entry_count - 1]; 
	printf("%.*s\n", info->length, &buffer[info->offset]);
}

void pop_last_entry(PopInfo* pop_data, int fd) {
	if(!pop_data->entry_count)
		return;
	PopEntryInfo* info = &pop_data->pop_entries[pop_data->entry_count-1];
	ftruncate(fd, info->offset); 
}

void show_nth_entry(PopInfo* pop_data, int n) {
	if (!pop_data->entry_count)
		return;
	int total_items = pop_data->entry_count - 1;
	if (n > total_items)
	{
		fprintf(stderr, "Invalid entry\n"); 
		exit(-1); 
	}
	int req_entry = total_items + 1 - n;
	PopEntryInfo* entry = &pop_data->pop_entries[req_entry];
	printf("%.*s\n", entry->length, &buffer[entry->offset]); 
}
		
		
