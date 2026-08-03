#include "input_handler.h"
#include <ios>
#include <limits>

namespace UserInput {

    int getValidInteger( std::string_view prompt, int minVal, int maxVal) {

        static int totalPrompts {0} ;
        totalPrompts++;

        std::cout << "[System call count: " << totalPrompts << "]\n";
        int attempts { 0 };

        while ( totalPrompts <= UserInput::maxAttempts ) {
            int input;
            std::cout << prompt << " ";
            std::cin >> input;
            
            if ( std::cin.fail() || input > maxVal || input < minVal ) {

                std::cout << "Error while reading input\n";
                std::cin.clear();
                std::cin.ignore( std::numeric_limits<std::streamsize>::max(), '\n');
                ++attempts;

            } else {
                std::cin.ignore( std::numeric_limits<std::streamsize>::max(), '\n');
                return input;
            }
        }
        std::cout << "Max attempts reached! Returning fallback value.\n";
        return minVal;

    }
}
