#include <iostream>
#include "encoders.h"

int main() {

    std::uint8_t id { 10 };
    std::uint8_t temp { 255 };
    std::uint8_t battery { 120 };
    std::uint8_t state { 50 };

    std::uint32_t result = Encoders::encodeTelemetry( id, temp, battery, state );

    std::cout << std::bitset<32>(result) << std::endl;

    std::uint8_t id_result = Encoders::extractID(result);
    std::uint8_t temp_result = Encoders::extractTemp(result);
    std::uint8_t battery_result = Encoders::extractBattery(result);
    std::uint8_t state_result = Encoders::extractState(result);

    std::cout << std::bitset<8>(id_result) << "\t" << static_cast<int>(id_result) << std::endl;
    std::cout << std::bitset<8>(temp_result) << "\t" << static_cast<int>(temp_result) << std::endl;
    std::cout << std::bitset<8>(battery_result) << "\t" << static_cast<int>(battery_result) << std::endl;
    std::cout << std::bitset<8>(state_result) << "\t" << static_cast<int>(state_result) << std::endl;

    Encoders::processState(state_result);

    return 0;

}
