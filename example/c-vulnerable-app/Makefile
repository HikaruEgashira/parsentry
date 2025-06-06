CC=gcc
CFLAGS=-Wall -g
TARGET=vulnerable_app
SOURCES=main.c

all: $(TARGET)

$(TARGET): $(SOURCES)
	$(CC) $(CFLAGS) -o $(TARGET) $(SOURCES)

clean:
	rm -f $(TARGET)

.PHONY: all clean