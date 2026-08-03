#ifndef INPUT_HANDLER_H
#define INPUT_HANDLER_H

#include <iostream>
#include <string_view>

namespace UserInput {

    inline constexpr int maxAttempts { 3 };
    static_assert( maxAttempts > 0 );

    int getValidInteger( std::string_view prompt, int minVal, int maxVal);

}

#endif
