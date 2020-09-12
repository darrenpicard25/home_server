#include <stdio.h>
#include "sum.h"

int main(void)
{
    printf("Running program\n");
    int value = sum(3, 5);
    printf("Total sum is %d\n", value);
    return 0;
}