RM 				= rm -rf

CC 				= g++
CFLAGS 			= -w 
CFLAGS_DEBUG	= -g -w 

SOURCES			= src/*
INCLUDES		= src/*.h
MAIN_PROGRAM	= trello-cli

OUTPUT_DIR		= out

default: $(MAIN_PROGRAM)

$(MAIN_PROGRAM): $(SOURCES)
	$(CC) $(SOURCES) -o $(OUTPUT_DIR)/$(MAIN_PROGRAM) $(CFLAGS) 

debug: $(SOURCES)
	$(CC) $(SOURCES) -o $(OUTPUT_DIR)/$(MAIN_PROGRAM) $(CFLAGS_DEBUG)

clean:
	$(RM) $(OUTPUT_DIR)/*
