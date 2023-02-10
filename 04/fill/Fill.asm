// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// To write to write pixel values (ones or zeros, white or black) to the screen, you address memory locations
// 0x4000 to 0x5FFF, which correspond to the pixels of the screen.
// Reading any input from the keyboard is done by reading the memory location 0x6000.

// First, we check if a key is pressed. If so, we write "black" (-1) to the screen.
// If not, we write "white" (0) to the screen.

(START)
	@24576 // 0x6000 keyboard input
	D=M // D=keyboard input

	// Set `color` to 0 if a key is pressed, and to 1 otherwise
	@KEY_PRESSED
		D;JNE // if D!=0 (key pressed), jump to @color

		@0
		D=0 // D=0

		// Jump to the end of the if statement
		@END_KEY_PRESSED
		0;JMP
		
		(KEY_PRESSED)
		D=-1 // D!=0

	(END_KEY_PRESSED)

	@0
	M=D // color=-1 if a key is pressed, 0 otherwise

	@16384
	D=A // D=0x4000
	@0 
	M=D // counter=0x4000

	(LOOP)
		@0 // Get the color
		D=M // D=color

		@1
		A=M // Set memory to location described by counter
		M=D // Write `color` to the screen

		// Increment counter
		@1
		M=M+1

		// Check if we reached the end of the screen
		@24576
		D=A // D=0x6000
		@1
		D=M-D // D=counter-0x6000

		@LOOP
		D;JGT // if D > 0 (counter-0x6000>0), jump to LOOP

	// Jump back to the start
	@START
	0;JMP