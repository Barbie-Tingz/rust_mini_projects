/* Coffee Run Disaster Calculator

**The bit:** An employee was sent to get coffee for the entire office and returned with a
catastrophically wrong order — several drinks swapped, one completely missing, and
someone's name spelled in a way that started an argument.

**Your mission:** Write a function that takes three specific numbers about the coffee
run — wrong orders, minutes late, and people upset — and turns them into a single
"disaster score." Then print a different verdict depending on how the score comes out.*/

// 1. Create your main function.
// 2. In main, create three variables: wrong_orders (i32), minutes_late (i32), and
//    people_upset (i32), and give them example values.
// 3. Define three constants for how much each input should weigh toward the final score:
//    WRONG_ORDER_WEIGHT, MINUTE_LATE_WEIGHT, and PEOPLE_UPSET_WEIGHT (all i32).
// 4. Write a function named calculate_disaster_score that takes wrong_orders,
//    minutes_late, and people_upset as parameters and returns an i32 score, using the
//    three constants from step 3 in the math.
// 5. Call calculate_disaster_score from main and store the result in a variable.
// 6. Print the raw score first, to confirm the function is working.
// 7. Use if / else if / else to check the score against thresholds you choose, printing
//    one of three verdicts: "Everyone Survived," "Office Morale Damaged," or
//    "Someone Is Filing HR Paperwork."
// 8. Test it by changing the three input values in main and re-running, to confirm all
//    three verdicts are reachable.