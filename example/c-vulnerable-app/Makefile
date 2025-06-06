CC=gcc
CFLAGS=-Wall -Wextra -g -O0 -fno-stack-protector -D_FORTIFY_SOURCE=0
TARGET=vulnerable_app
SOURCES=main.c

# Disable security features for demonstration purposes
CFLAGS += -fno-stack-protector -z execstack -no-pie

$(TARGET): $(SOURCES)
	$(CC) $(CFLAGS) -o $(TARGET) $(SOURCES)

clean:
	rm -f $(TARGET)

run: $(TARGET)
	./$(TARGET)

.PHONY: clean run