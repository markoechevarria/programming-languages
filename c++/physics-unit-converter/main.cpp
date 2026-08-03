#include <iostream>
#include "physics.h"

int main() {

    double original { 100.0 };

    double value_to_feets = convertDistance( original , "feet"); 
    double value_to_meters = convertDistance( value_to_feets , "meters");

    bool result_comparison = approximatelyEqualRel( original, value_to_meters, 0.001 );

    std::cout << "Original value " << original << "\n";
    std::cout << "To Feets " << value_to_feets << "\n";
    std::cout << "To Meters " << value_to_meters << "\n";
    std::cout << ( result_comparison ? "Equal" : "No equal" ) << "\n";

    return 0;

}
