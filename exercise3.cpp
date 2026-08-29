#include <iostream>

int main()
{
    for (int i = 97; i <= 122; i++)
    {
        std::cout << static_cast<char>(i);
    }

    return 0;
}