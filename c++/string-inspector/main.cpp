#include <cctype>
#include <iostream>
#include <string_view>

constexpr bool isVowel( char character ) {

    char letter = static_cast<char>( std::tolower(character) );

    return letter == 'a' || letter == 'e' || letter == 'i' || letter == 'o' || letter == 'u';

}

constexpr std::string_view classifyWord( std::string_view sv ) {

    if ( sv.empty() ) {
        return "Empty";
    }

    if ( sv.length() == 1 ) {
        return "Single-Char";
    } 

    if ( sv.front() == sv.back() ) {
        return "Bookended";
    } 

    if ( isVowel( sv.front() ) && isVowel( sv.back() ) )  {
        return "Vowel-Bounded";
    }

    return "Standard";

}

void printWordDetails( std::string_view sv ) {

    std::string_view classification = classifyWord(sv);
    int sv_len = static_cast<int>( sv.length() );

    std::cout << "Lenght => " << sv_len << std::endl;
    std::cout << "Classification => " << classification << std::endl;
    
    if ( !sv.empty() ) {

        std::cout << "Front => " << static_cast<int>( sv.front() ) << std::endl;
        std::cout << "Back => " << static_cast<int> ( sv.back() ) << std::endl;

    }

}

int main () {

    std::string text { "marko" };
    std::cout << "\n\nText: '"<< text << "'" << std::endl;
    printWordDetails( text );

    constexpr std::string_view result = classifyWord( "scorpions" );
}
