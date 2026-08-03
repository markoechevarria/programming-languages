#include <algorithm>
#include <cmath>
#include "physics.h"

bool approximatelyEqualRel( double a, double b, double relEps ) {

    double left_side = std::abs( a - b );
    double right_side = std::max( std::abs(a) , std::abs(b) );

    return left_side <= right_side * relEps;

}

double convertDistance(double value, std::string_view targetUnit) {

    if ( targetUnit == "feet" ) {

        return value * metersToFeet;

    } else if ( targetUnit == "meters" ) {

        return value / metersToFeet;

    } else {

        return 0.0;

    }

}
