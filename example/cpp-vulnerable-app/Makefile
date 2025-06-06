CXX=g++
CXXFLAGS=-std=c++14 -Wall -Wextra -g -O0 -fno-stack-protector -D_FORTIFY_SOURCE=0
TARGET=vulnerable_cpp_app
SOURCES=main.cpp

# Disable security features for demonstration purposes
CXXFLAGS += -fno-stack-protector -z execstack -no-pie -pthread

$(TARGET): $(SOURCES)
	$(CXX) $(CXXFLAGS) -o $(TARGET) $(SOURCES)

clean:
	rm -f $(TARGET)

run: $(TARGET)
	./$(TARGET)

.PHONY: clean run