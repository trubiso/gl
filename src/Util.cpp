#include "Util.h"
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

std::string read_file(const char *path) {
	std::string content;
	std::ifstream file;
	file.exceptions(std::ifstream::failbit | std::ifstream::badbit);

	try {
		file.open(path);
		std::stringstream str;
		str << file.rdbuf();
		file.close();
		content = str.str();
	} catch (std::ifstream::failure err) {
		std::cout << "[ERROR] Couldn't read file " << path << "\n"
		          << err.what() << std::endl;
	}

	return content;
}
