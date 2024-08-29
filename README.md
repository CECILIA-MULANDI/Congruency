
#Congruency and Modular Arithmetic in Rust

##Congruency Check

The is_congruent function determines if two integers are congruent* modulo a* given modulus. It calculates whether the *difference between the two integers is divisible by the modulus*. 
If the *difference is divisible*, the function returns *true*, indicating that the numbers are congruent modulo the specified modulus.

##Find Congruent Numbers

The find_congruent_numbers function identifies all integers within *a specified range* that are congruent to a given value modulo a specified modulus. It iterates through the range, checking if each number has the same remainder when divided by the modulus as the given value. It collects and returns all such numbers.

##Generate Congruent Numbers

The generate_positive_congruent_numbers and generate_negative_congruent_numbers functions generate sequences of numbers that are congruent to a specified value modulo a given modulus.

    generate_positive_congruent_numbers: Produces the first count positive numbers that satisfy the congruence.
    generate_negative_congruent_numbers: Produces the first count negative numbers that satisfy the congruence.