#ifndef PHYSICS_H
#define PHYSICS_H

#include <string_view>

constexpr float metersToFeet = 3.28084;
constexpr float celsiusToFarenheitMultiplier = 1.8;
constexpr float celsiusToFarenheitOffset = 32.0;

bool approximatelyEqualRel( double a, double b, double relEps = 1e-5 );

double convertDistance(double value, std::string_view targetUnit);

#endif
