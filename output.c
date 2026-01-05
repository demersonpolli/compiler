#include <stdio.h>

int main() {
    long long A, I, X, Y, Z;

    X = 10;
    Y = 5;
    Z = (X + (Y * 2));
    printf("%lld\n", (long long)Z);
    for (I = 1; I <= 5; I++) {
        A = (I * 2);
        printf("%lld\n", (long long)A);
    }
    printf("%lld\n", (long long)(X - Y));

    return 0;
}
