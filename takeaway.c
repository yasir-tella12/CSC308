#include <stdio.h>

int main() {
    char type;
    printf("Enter the type of input (c for character, f for float, i for integer): ");
    scanf(" %c", &type);

    if (type == 'c') {
        char ch;
        printf("Enter a character: ");
        scanf(" %c", &ch);
        printf("Input character: %c | ASCII: %d | Size: %lu bytes\n", ch, ch, sizeof(ch));
        for (int i = 1; i <= 4; i++) {
            char nextChar = ch + (i * 3);
            printf("Next character #%d: %c | ASCII: %d\n", i, nextChar, nextChar);
        }
    } 
    else if (type == 'f') {
        float num;
        printf("Enter a float number: ");
        scanf("%f", &num);
        printf("Input float: %.2f | Size: %lu bytes\n", num, sizeof(num));
        for (int i = 1; i <= 4; i++) {
            float nextFloat = num + (i * 3.0f);
            printf("Next float #%d: %.2f\n", i, nextFloat);
        }
    } 
    else if (type == 'i') {
        int num;
        printf("Enter an integer: ");
        scanf("%d", &num);
        printf("Input integer: %d | Size: %lu bytes\n", num, sizeof(num));
        for (int i = 1; i <= 4; i++) {
            int nextInt = num + (i * 3);
            printf("Next integer #%d: %d\n", i, nextInt);
        }
    } 
    else {
        printf("Invalid input type. Please enter 'c', 'f', or 'i'.\n");
    }

    return 0;
}
