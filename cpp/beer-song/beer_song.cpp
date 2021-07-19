#include "beer_song.h"
#include <sstream>

namespace beer_song {

std::string verse(int i)
{
    std::stringstream ss;

    if (i == 0) {
        ss << "No more bottles of beer on the wall, no more bottles of beer.\n"
              "Go to the store and buy some more, 99 bottles of beer on the wall.\n";
    } else if (i == 1) {
        ss << "1 bottle of beer on the wall, 1 bottle of beer.\n"
              "Take it down and pass it around, no more bottles of beer on the wall.\n";
    } else if (i == 2) {
        ss << "2 bottles of beer on the wall, 2 bottles of beer.\n"
              "Take one down and pass it around, 1 bottle of beer on the wall.\n";
    } else if (i >= 3 && i <= 99) {
        ss << i << " bottles of beer on the wall, " << i << " bottles of beer.\n"
              "Take one down and pass it around, " << i - 1 << " bottles of beer on the wall.\n";
    }
    return ss.str();
}

std::string sing(int from, int to)
{
    std::string song;
    for (int i = from; i >= to; --i) {
        song.append(verse(i));
        if (i != to) {
            song.push_back('\n');
        }
    }
    return song;
}

}  // namespace beer_song
