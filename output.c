#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <time.h>

int main() {
    srand(time(NULL));
    double I, X;

    X = 10.0;
    printf("%s", "X IS ");
    printf("%g ", X);
    printf("\n");
    printf("%s", "COUNTING: ");
    for (I = 1.0; I <= 3.0; I += 1.0) {
        printf("%g ", I);
        printf("%s", " ");
    }
    printf("%s", "DONE");
    printf("\n");

    return 0;
}
