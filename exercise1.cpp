#include <iostream>
#include <ctime>
#include <cstdlib>

using namespace std;

int main() {
    srand(time(NULL));

    int number = rand() % 10000 ;
    cout << "Random number between 1 and 1000: " << number << endl;

    if (number >= 0) {
        cout << "The number is positive." << endl;
    }
    else if (number < 0) {
        cout << "The number is negative." << endl;
    }
    else {
        cout << "The number is zero." << endl;
    }

    return 0;
}