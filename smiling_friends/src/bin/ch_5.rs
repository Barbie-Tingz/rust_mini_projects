/* Meet the Employees & Clients

**The bit:** The agency's paperwork has caught up with reality: management finally demands
actual employee records after discovering nobody knows how many people currently work
there. Your Employee is Debra, whose sanity has been in freefall since the swamp-landlord
case. Your Client is a man whose case description reads: "insists his own shadow filed a
restraining order against him."

**Your mission:** Define structs for your core character types, so later chapters have
something to build on.*/

// 1. Create your main function.
// 2. Define an Employee struct with exactly these three fields: name (String),
//    job_title (String), and sanity_level (i32).
// 3. Define a Client struct with exactly these three fields: name (String),
//    case_description (String), and weirdness_rating (i32).
// 4. Inside main, create one Employee instance and one Client instance using made-up
//    example data.
// 5. Write a function named introduce_employee that takes a reference to an Employee
//    and prints its name and job_title in a sentence.
// 6. Write a function named introduce_client that takes a reference to a Client and
//    prints its name and case_description in a sentence.
// 7. Call both functions from main with your sample instances.