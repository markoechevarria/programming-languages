#include <bitset>
#include <cassert>
#include <cstdint>
#include <iostream>
#include <random>

namespace Flags {

    constexpr std::uint8_t flagPoisoned = 1 << 0;
    constexpr std::uint8_t flagStunned = 1 << 1;
    constexpr std::uint8_t flagShielded = 1 << 2;
    constexpr std::uint8_t flagEnraged = 1 << 3;

}

void print_player ( int turn, int result, std::uint8_t player) {

    std::cout << "**************************\n";
    std::cout << "- Turn " << turn << " with Random " << result << "\n" ;
    std::cout << "- Player " << std::bitset<8>(player) << "\n";

    if ( player & Flags::flagEnraged ) {
        std::cout << "  [Active] Enraged\n";
    }
    if ( player & Flags::flagShielded ) {
        std::cout << "  [Active] Shielded\n";
    }
    if ( player & Flags::flagStunned ) {
        std::cout << "  [Active] Stunned\n";
    }
    if ( player & Flags::flagPoisoned ) {
        std::cout << "  [Active] Poisoned\n";
    }
    
    std::cout << "\n";

}

int main() {

    std::random_device rd;
    std::mt19937 gen( rd() );
    std::uniform_int_distribution<int> distrib(1, 4);

    std::uint8_t player = 0;

    for ( int i=0; i < 10; i++ ) {

        int random_number = distrib(gen);

        switch (random_number) {

            case 1:
                player |= Flags::flagPoisoned;
                player &= ~Flags::flagShielded;
                break;
            case 2: 
                player |= Flags::flagShielded;
                player &= ~Flags::flagPoisoned;
                break;
            case 3:
                player |= Flags::flagStunned;
                break;
            case 4:
                player ^= Flags::flagEnraged;
                break;
        }

        print_player(i, random_number, player);

        assert( !( (player & Flags::flagPoisoned ) && ( player & Flags::flagShielded )) );

    }

}
