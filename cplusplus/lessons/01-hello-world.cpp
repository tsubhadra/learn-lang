// 01-hello-world.cpp
// A simple C++ program to print "Hello world!"
//g++ 01-hello-world.cpp -o 01-hello-world
//./01-hello-world
//g++ 01-hello-world.cpp -o 01-hello-world && ./01-hello-world

#include <iostream>
//#include <iostream>: Includes the Input/Output stream library for console output

int main() {
    //int main(): The entry point of every C++ program
    std::cout << "Hello world!" << std::endl;
    //std::cout << "Hello world!" << std::endl;: Prints "Hello world!" to the console
    return 0;
    //return 0;: Returns 0 to indicate successful program execution
}

