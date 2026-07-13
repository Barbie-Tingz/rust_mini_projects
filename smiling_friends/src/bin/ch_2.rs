/* Guess the Client's Mood 

**The bit:** A client legally changed his name to "Sandwich" and is convinced everyone's jealous of it. 
Nobody knows what mood he'll be in today — could be anywhere from "at peace with it" to "feral."

**Your mission:** Build a program that picks a secret mood number and lets the player keep guessing until they land on it, 
giving a hint each time they're off. **Twist:** Sandwich is impatient — the longer it takes to guess, the worse his mood gets. 
Every so often (you decide how often), the secret number itself creeps upward, so a guess that would've been right two turns ago might be wrong now.*/

// 1. Import your crates. (Standard Input/Output & Random Generator)
// 2. Create your main function. 
// 3. Generate the secret mood number (1-100). 
// 4. Set up guess counter. 
// 5. Start main loop. 
// 6. Print a prompt to the terminal. 
// 7. Read player's input. 
// 8. Convert the user's input from a string to a number. 
// 9. Increment the guess counter by 1.
// 10. Check if it's time to shift the mood. Hint: Can be every 3 guesses!
// 11. If shifting, increase the secret number, but cap it so it never goes above 100.
// 12. Check if the secret number has hit 100 — if so, print an "EXTREME FERAL RAMPAGE" loss message and break.
// 13. Compare the guess to the secret number. Hint use match statements. 
// 14. Print hint aka. Too calm or Too feral.
// 15. Break the loop on a correct guess.