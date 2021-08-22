RM 				= rm -rf

CC 				= g++
CFLAGS 			= -w 
CFLAGS_DEBUG	= -g -w 

SOURCES			= src/*.cpp src/control/*.cpp src/data/*.cpp src/service/*.cpp
INCLUDES		= src/*.h src/control/*.h src/data/*.h src/service/*.h
LIB_INCLUDES	= 
MAIN_PROGRAM	= trello-cli

OUTPUT_DIR		= out

default: $(MAIN_PROGRAM)

$(MAIN_PROGRAM): $(SOURCES)
	$(CC) $(SOURCES) $(LIB_INCLUDES) $(CFLAGS) -o $(OUTPUT_DIR)/$(MAIN_PROGRAM) 

debug: $(SOURCES)
	$(CC) $(SOURCES) $(LIB_INCLUDES) $(CFLAGS_DEBUG) -o $(OUTPUT_DIR)/$(MAIN_PROGRAM) 

clean:
	$(RM) $(OUTPUT_DIR)/*
