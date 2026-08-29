#include <iostream>
#include <cstdlib>
#include <ctime>
using namespace std;

int main()
{
    srand(time(0));

    int number;
    number = rand() % 200 - 100;

    int last_digit = abs(number % 10);//chose 10 because we're trying to find the last digit, and 10 is the base of the last decimal number system

    cout << "Last digit of " << number << " is " << last_digit;

    if (last_digit > 5)
        cout << " and is greater than 5";
    else if (last_digit == 0)
        cout << " and is 0";
    else
        cout << " and is less than 6 and not 0";

    cout << endl;

    return 0;
}
