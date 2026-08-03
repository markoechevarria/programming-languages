#include "encoders.h"
#include <bitset>
#include <iostream>

namespace Encoders {

    std::uint8_t extractID( std::uint32_t packet) { 

        std::uint8_t unpacked = static_cast<std::uint8_t>(packet >> 0);
        return unpacked & 0xFF; 

    }

    std::uint8_t extractTemp( std::uint32_t packet) { 

        std::uint8_t unpacked = static_cast<std::uint8_t>(packet >> 8);
        return unpacked & 0xFF; 

    }

    std::uint8_t extractBattery( std::uint32_t packet) { 

        std::uint8_t unpacked = static_cast<std::uint8_t>(packet >> 16);
        return unpacked & 0xFF; 

    }

    std::uint8_t extractState( std::uint32_t packet) { 

        std::uint8_t unpacked = static_cast<std::uint8_t>(packet >> 24);
        return unpacked & 0xFF; 

    }

    void processState( std::uint8_t state) {

        switch (state) {

            case 0:
                std::cout << "Status: System Normal | Idle" << std::endl;
                break;
            case 1:
                std::cout << "Status: Transmiting Data" << std::endl;
                break;
            case 2:
                std::cout << "WARNING: Temperature threshold exceeded" << std::endl;
                [[fallthrough]];
            case 3:
                std::cout << "CRITICAL: Executing emergency shutdown procedure" << std::endl;
                break;

        }

    }

}
