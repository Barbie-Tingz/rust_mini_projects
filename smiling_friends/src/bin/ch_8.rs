/* The Case Files & Staff Roster

**The bit:** The agency needs an actual staff roster and a filing cabinet for open cases,
instead of just remembering things.

**Your mission:** Store your employees in a Vec and your case files in a HashMap.*/

// 1. Create a Vec<Employee> to hold your staff roster, and push a few Employee instances into it.
// 2. Create a HashMap<String, Client> (or a Case struct) keyed by client name or case ID,
//    representing open case files.
// 3. Insert a few sample cases into the HashMap.
// 4. Write a function that loops over the roster Vec and prints each employee's name.
// 5. Write a function that looks up a specific case in the HashMap by key and prints its
//    details — handle the case where that key doesn't exist.
// 6. From main, add a new employee to the roster and a new case to the HashMap, then reprint
//    both to confirm they updated.
// 7. (Optional) Remove an employee from the roster by index, or a case from the HashMap by key.