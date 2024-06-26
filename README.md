## Commands:
- ### Directory control:
  - ```cd \path\goes\here``` -> change directory to path passed as an argument
  - ```pwd``` -> prints the current path, also visible behind the newline symbol
  - ```ls``` -> prints out all files and directories in the current directory
  - ```create``` -> makes an empty file called ```emptyFile``` in the current directory (WIP)
  - ```rm``` -> deletes the file called ```emptyFile``` in the current directory (WIP)

- ### Math tools:
  - ```sum a1 a2 a3... an``` -> Sum: prints the sum of all arguments
  - ```prod a1 a2 a3... an``` -> Product: prints the product of all arguments
  - ```exp a b``` -> Exponent: prints a^b
  - ```mod a b``` -> Mod: prints a % b
  - ```fib n``` -> Fibonacci: prints the $n^{\text{th}}$ fibonacci number
  - ```pi``` -> Pi: prints $\pi$ to a 100 digits
  - ```bin n``` -> Binary: prints n in base 2
  - ```hex n``` -> Hexadecimal: prints n in base 16
  - ```oct n``` -> Octanary: prints n in base 8
  - ```bitshift n m``` -> Bitshifts to the left m times: returns $n \times 2^m$

- ### Terminal flow:
  - ```exit``` -> exit the shell
  - ```clear``` -> doesn't actually work but is supposed to clear the screen (WIP)
  - ```cmd1 || cmd2``` -> chain commands together, executing left to right
