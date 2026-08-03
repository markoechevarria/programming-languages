#ifndef ENCODERS_H
#define ENCODERS_H

#include <bitset>
#include <cstdint>

namespace Encoders {

    constexpr std::uint32_t encodeTelemetry( std::uint8_t id, std::uint8_t temp, std::uint8_t battery, std::uint8_t state) {

        std::uint32_t mask = static_cast<std::uint32_t>(0);
        std::uint32_t id_enc = static_cast<std::uint32_t>(id) << 0;
        std::uint32_t temp_enc = static_cast<std::uint32_t>(temp) << 8;
        std::uint32_t battery_enc = static_cast<std::uint32_t>(battery) << 16;
        std::uint32_t state_enc = static_cast<std::uint32_t>(state) << 24;

        return mask | id_enc | temp_enc | battery_enc | state_enc;

    }

    std::uint8_t extractID( std::uint32_t packet);

    std::uint8_t extractTemp( std::uint32_t packet);

    std::uint8_t extractBattery( std::uint32_t packet);

    std::uint8_t extractState( std::uint32_t packet);

    void processState( std::uint8_t state);

}

#endif
