#include <iostream>
#include <iterator>
#include <string>

int main() {
	std::istreambuf_iterator<char> begin(std::cin), end;
	std::string s(begin, end);
	std::cout << s;
}
