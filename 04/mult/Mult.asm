// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

// Essentially, we repeatedly add R0 to R2, R1 times. In order to track how many times 
// we added, we store that value in `counter`

// First, we initialize R2 to 0, and counter to R1.
@R2
M=0

@R1
D=M
@counter
M=D // We set the counter to R1 because comparison operations check against 0, not some arbitrary value.

// Then, we start a loop that will run R1 times.
(LOOP)
	// We check if we've added R0 R1 times.
	@counter
	D = M
	@END
	D; JLE // counter <= 0? (Jump comparisons check against 0)

	// We add R0 to R2.
	@R0
	D = M
	@R2
	M = M + D
	
	// We decrement the counter.
	@counter
	M = M - 1

	@LOOP
	D; JMP

// Finally, we end the program.
(END)
	// Infinte loop
	@END
	0; JMP
