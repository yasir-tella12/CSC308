#include <stdio.h>

int main() {
    int age;

    // Prompt user for age
    printf("Enter your age: ");
    scanf("%d", &age);

    // Check voting eligibility
    if (age > 18) {
        printf("You can vote.\n");
    } else {
        printf("You cannot vote.\n");
    }

    return 0;
}
