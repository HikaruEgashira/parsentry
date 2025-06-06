CXX=g++
CXXFLAGS=-Wall -g -std=c++17
TARGET=vulnerable_cpp_app
SOURCES=main.cpp

all: $(TARGET)

$(TARGET): $(SOURCES)
	$(CXX) $(CXXFLAGS) -o $(TARGET) $(SOURCES)

clean:
	rm -f $(TARGET)

.PHONY: all clean