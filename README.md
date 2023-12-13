[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-24ddc0f5d75046c5622901739e7c5dd533143b0c8e959d652212380cedb1ea36.svg)](https://classroom.github.com/a/1qNXzPQT)
# Rust Programming Assignment: Pico Data Analysis Tool

## Motivation
Rust is becoming a popular language. It was created to have high performance, reliability and productivity. The code is compiled and it claims to have advanced optimizations that produce stable and efficient programs. It has concurrent abilities, it provides memory safety without a runtime garbage collector.

This project consists in the development of the front end of a compiler. By programming the Lexical Analyzer (Scanner) and Syntax Analyzer (Parser) for the Data-Analysis (DA) grammar you will gain further understanding of the lexical analysis and the production of tokens needed for the Syntax Analyzer (Parser), and how to consume those tokens by the Parser to verify that the syntax is correct.


## Description
Write a program in Rust that takes a program written in DA, and outputs:
1. If the program has lexical or syntax errors, the error that was found. Use "hide the head in the sand, like an ostrich" version of error handling.
1. If the program is OK, depending on a command line flag the program will produce:
   1.	If the flag is `-s` the program will output a code in Scheme that is going to be called by a program in Scheme that will execute the operations specified in the program.
   1. If the flag is `-p` the program will output a series of queries based in the operations specified in the program.

The program should run like this:
```
prompt>cargo run input.da -s
; Processing Input File input.da
; Lexical and Syntax analysis passed
(define xvalues (read-csv "./file.csv" #f 0))
(define yvalues (read-csv "./file.csv" #f 1))
(define a (regressiona xvalues yvalues))
(define b (regressionb xvalues yvalues))
(define r (correlation xvalues yvalues))
(display "value of a = ")
(newline)
(display a)
(newline)
(display "value of b = ")
(newline)
(display b)
(newline)
(display "value of r = ")
(newline)
(display r)
(newline)
prompt>
```

## Grammar

```
PROGRAM     -->   data:
                     DATADEFS
                  input:
                     INPUTOPS
                  process:
                     PROCESSOPS
                  output:
                     OUTPUTOPS
                  end.
DATADEFS    -->   DATADEF |
                  DATADEF, DATADEFS
DATADEF     -->   ID : TYPE
INPUTOPS    -->   INPUTOP |
                  INPUTOP, INPUTOPS
INPUTOP     -->   ID = read(STRING, BOOL, NUM)
PROCESSOPS  -->   PROCESSOP |
                  PROCESSOP, PROCESSOPS
PROCESSOP   -->   ID = regressiona(ID, ID) |
                  ID = regressionb(ID, ID) |
                  ID = mean(ID) |
                  ID = stddev(ID) |
                  ID = correlation(ID, ID)
OUTPUTOPS   -->   OUTPUTOP |
                  OUTPUTOP, | OUTPUTOPS
OUTPUTOP    -->   STRING |
                  ID
ID          -->   LETTER+
TYPE        -->   vector | number
BOOL        -->   true | false
STRING      -->   "LETTER+"
NUM         -->   DIGIT+
LETTER      -->   a | b | c | d | e | f | g | ... | z
DIGIT       -->   0 | 1 | 2 | 3 | 4 | 5 | 6 | ... | 9
```

The tokens of this grammar are (some lexemes shown as examples):
| Token | Lexeme |
| ----- | ------ |
| `DATA` | `data` |
| `INPUT` | `input` |
| `PROCESS` | `process` |
| `OUTPUT` | `output` |
| `END` | `end` |
| `ID` | `alpha` |
| `NUM` |  `256` |
| `true` | `true` |
| `false` | `false` |
| `READ` | `read` |
| `COLON` | `:` |
| `COMMA` | `,` |
| `PERIOD` | `.` |
| `LPAREN` | `(` |
| `RPAREN` | `)` |
| `ASSIGN` | `=` |
| `VECTOR` | `vector` |
| `NUMBER` | `number` |
| `REGRESSIONA` | `regressiona` |
| `REGRESSIONB` | `regressionb` |
| `MEAN` | `mean` |
| `STDDEV` | `stddev` |
| `CORRELATION` | `correlation` |
| `STRING` | `"the value"` |

Given the following program written in this language:
```
data:
   xvalues = vector,
   yvalues = vector,
   a = number,
   b = number,
   r = number
input:
   xvalues = read("file.csv", false, 0),
   yvalues = read("file.csv", false, 1)
process:
   a = regressiona(xvalues, yvalues),
   b = regressionb(xvalues, yvalues),
   r = correlation(xvalues, yvalues)
output:
   "value of a = ",
   a,
   "value of b = ",
   b,
   "value of r = ",
   r
end.
```
The tokens that it would generate are:
1. DATA
2. COLON
3. ID xvalues
4. ASSIGN
5. VECTOR 
6. COMMA
7. ID yvalues
8. ASSIGN
9. VECTOR
10. COMMA
11. ID a
12. ASSIGN
13. NUMBER
14. COMMA
15. ID b
16. ASSIGN
17. NUMBER
18. COMMA
19. ID r
20. ASSIGN
21. NUMBER
22. INPUT
23. COLON
24. ID xvalues
25. ASSIGN
26. READ
27. LPAREN
28. STRING "file.csv"
29. COMMA
30. FALSE
31. COMMA
32. NUM 0
33. RPAREN
34. COMMA 
35. ID yvalues
36. ASSIGN
37. READ
38. LPAREN
39. STRING "file.csv"
40. COMMA
41. FALSE
42. COMMA
43. NUM 1
44. RPAREN
45. PROCESS
46. COLON
47. ID a
48. ASSIGN
49. REGRESSIONA
50. LPAREN
51. ID xvalues
52. COMMA
53. ID yvalues
54. RPAREN
55. COMMA
56. ID b
57. ASSIGN
58. REGRESSIONB
59. LPAREN
60. ID xvalues
61. COMMA
62. ID yvalues
63. RPAREN
64. COMMA
65. ID r
66. ASSIGN
67. CORRELATION
68. LPAREN
69. ID xvalues
70. COMMA
71. ID yvalues
72. RPAREN
73. OUTPUT
74. COLON
75. STRING "value of a = "
76. COMMA
77. ID a
78. COMMA
79. STRING "value of b = "
80. COMMA
81. ID b
82. COMMA 
83. STRING "value of r = " 
84. COMMA
85. ID r
86. END
87. PERIOD

Notice that the ID, NUM, and STRING tokens have their lexeme associated. Also notice that in the language the elements do not need to be separated by space, but they could.

## How to run the program

### Scheme Output
To generate scheme output you will add the `-s` flag at the end of the command:
```
prompt> cargo run input.da -s
; processing input file input.da
; Lexical and Syntax analysis passed
(define xvalues (read-csv "./file.csv" #f 0))
(define yvalues (read-csv "./file.csv" #f 1))
(define a (regressiona xvalues yvalues))
(define b (regressionb xvalues yvalues))
(define r (correlation xvalues yvalues))
(display "value of a = ")
(newline)
(display a)
(newline)
(display "value of b = ")
(newline)
(display b)
(newline)
(display "value of r = ")
(newline)
(display r)
(newline)
```


### Prolog Output
To generate prolog output you will add the `-p` flag at the end of the command:
```
prompt> cargo run input.sc -p
/* processing input file input.sc
   Lexical and Syntax analysis passed */

main :-
   load_data_column('file.csv', false, 0, Data0),
   load_data_column('file.csv', false, 1, Data1),
   regressiona(Data0, Data1, A),
   regressionb(Data0, Data1, B),
   correlation(Data0, Data1, R),
   writeln("value of a = "),
   writeln(A),
   writeln("value of b = "),
   writeln(B),
   writeln("value of r = "),
   writeln(R).

```

### Note about the Output
You are not expected to output the list of tokens. You can do it to check your work, but remember to remove them from the output before submitting your final version.

Later we will redirect the output to Scheme and Prolog programs respectively.

## Assignment Requirements
- Good programming practices
  - Indentation
  - Meaningful identifier naming
  - Consistent variable naming convention
  - Clean code, This means remove commented code that you are not using, and make sure that you are using appropriate and professional vocabulary in your variables and comments.
  - Commented code
- This activity is strictly individual

## Delivery
You will use this repository and commit and push to it. Remember to push your last version before the deadline.
What files should be in your repository:
- `src/main.rs` Source code in Rust for your lexical and syntax analysis
- `test0.da`, `test1.da`, `test2.da`, `test3.da`, `test4.da`, `test5.da` the test files provided for you to test. Notice that `test4.da` has a lexical error and `test5.sdac` has a syntax error.

You may use more source code files. Actually, you are encouraged to create modules for your assignment.

## Assessment and Grading
Assessment will consider the following factors in the grading of this assignment:
-	Good programming practices
-	Your program will be tested with the five test programs that were provided and some others that will be made to test, some with lexical errors, some with syntax errors, some without any errors.
-	Adherence to instructions
-	Correct function of the program, that is, the program behaves correctly.
-	No runtime errors (on any input!) *Remember the command line parameters!*
-	Late deliveries will have a zero mark
-	Plagiarism will have a double zero mark (in addition to losing 10% of your final grade, the person that plagiarizes will lose an additional 10% of their final grade), and there will be a report filed in the students’ academic record.
- Remember that you will be assessed about this assignment in the course tests, so work on this conscientiously.

## Extra Challenge

Create an additional file with a decorated (Tokens with Lexemes on the Leafs) parse tree. The file should be named like the input file but with the extension `.pt` (stands for parse tree), for instance if the input file is `test1.da` the parse tree should be in file `test1.pt`. You are allowed to create an HTML file, or any **standard** format that can be visualized by a Firefox browser.


**NOTE**
You just need to report the first error that you find (that is what "ostrich" version means), if you find a lexical error, report it and stop the program, if the program passes the lexical analysis, if it finds a syntax error, report it and stop the program. If the program passes both the lexical and syntax analyzers then proceed to generate the code.

> Hiding the head in the Sand. [Here](https://www.phrases.org.uk/meanings/bury-your-head-in-the-sand.html) you may find information about the expression that says that an "ostrich hides their head in the sand." Basically, in this context it means that if you are executing an operation that has many steps, and one of those steps fails, the whole thing comes to a stop. It is also called "panic resolution," however, I decided not to use it because students think that I mean that they should call the `panic`  function.


## Academic Integrity
This programming assignment is to be done on an individual basis. At the same time, it is understood that learning from your peers is valid and you are encouraged to talk among yourselves about programming in general and current assignments in particular.  Keep in mind, however, that each individual student must do the work in order to learn.  Hence, the following guidelines are established:
- Feel free to discuss any and all programming assignments but do not allow other students to look at or copy your code. Do not give any student an electronic or printed copy of any program you write for this class.
- Gaining the ability to properly analyze common programming errors is an important experience. Do not deprive a fellow student of his/her opportunity to practice problem solving: control the urge to show them what to do by writing the code for them.
- If you’ve given the assignment a fair effort and still need help, see the instructor or a lab assistant.
- **If there is any evidence that a program or other written assignment was copied from another student, neither student will receive any credit for it. This rule will be enforced.**
- **If there is any evidence that a program or other written assignment was copied from any source, the student will not receive any credit for it. This rule will be enforced.**
- Protect yourself: Handle throw-away program listings carefully, keep your repository private.

Refer to the ECS Department Policy on Academic Integrity that is included in the class syllabus.
