
first_digit last_digit t0 t1 t2 t3 t4 out0 out1 t7

>>
marker at t7
>>>> >>>+<<<< <<
, read first char into t1
[ main char loop (loop until zero/end)
	move char to t0 and t4
	[-<+>>>>+<<<]
	goto t0
	subtract 48=8x6 from char(t0) using t1 temporarily
	++++ ++++[<------>-]<
	move char to t1 and t2
	[->+>+<<]
	++++ ++++ ++ 10 in t0
	subtract 10 from t1 and store the difference in t4
	if t4 is not zero then this is a digit
	[- loop on t0(10)
		>- dec t1
		assume t1 reached zero
		move loop counter t0 to t3
		<[->>>+<<<]>
		[t1 still not zero
			undo loop counter reset from t3
			>>[-<<<+>>>]<<
			>>> >>> goto t7 to escape
		]
		>>> >>> goto t7
		-[+<-]+ go left until a cell with 1 (should be t7)
		<<< <<< return to t1

	return to t0 <
	]
	>>> goto t3
	if t3 is 0 then the character was not a digit so return (it never got reset by the previous loop)
	[ character is a digit (value is in t2)
		[-] reset t3
		<<<<
		[-]reset last_digit
		<
		if first_digit is set
		[
			>>>> goto t2
			move t2 to last_digit
			[-<<<+>>>]
			>>>>>> goto t8 to escape
		]
		>>>>>>>>> goto t7 or higher
		-[+<-]+ go left until a cell with 1 ( t7)
		<<<<< go back to t2
		copy t2 to last_digit and first_digit x10
		[-<<<+<++++ ++++ ++>>>>]
		> goto t3
	]
	<<[-] reset t1
	>>> goto t4
	---- ---- -- subtract 10 (value of LF)
	<+> mark t3 that LF is reached
	if there is something here still then its not a newline
	[
		<-> cancel mark on t3
		dump contents of t4 to exit this scope
		[-]
	]
	<goto t3
	[ LF is reached
		[-] remove mark on t3
		<<<< goto last_digit
		combine digits into last_digit
		<[->+<]>
		add to output:
		[while sum
			-
			>>>>>>>+ inc lower (second) byte
			<+> do carry
			[lower byte not zero so cancel the carry
				<->
				escape to t8
				>>
			]
			>goto t7 or t9
			-[+<-]+ go left until t7 (=1)
			<<<<<<<< goto sum(last_digit)
		]
		>>>> goto t3
	]
	reset t2 and t1
	<[-]<[-]
	, next char
]


>>>> high byte of output
>>[-]>> >>>>- new c8

Convert to decimal and print output
<c7
+[loop: every digit; c7
    <+
    [loop: div by 10; c6
        <<<<c2
        +++++ +++++ 10
        [-loop: subtract 10
            <- subtract low byte
            <-> underflow carry from high byte
            [c1
                did not underflow
                <+ undo underflow carry on c0
                >c1
                >> exit on c3
            ] c1 or c3
            >>>> c5 or c7
            +[->+]- marker c8

            <<<< << c2
            loop c2
        ] (subtract 10 complete)


        stop division (c6) if trying to high byte underflowed
        >>>>- unmark c6
        <<<< << c0
        +[-c0 is not  255
            >>>> >>+ mark c6; subtractions can continue
            <<< exit on c3
        ] c0 or c3
        >>>> >>>> c8 or c11
        +[-<+]- marker c8


        increase quotient
        <<<+ addition carry c5
        <+ add c4
        [did not overflow
            >- undo carry c5
            << exit on c3
        ] c4 or c3
        >>+[->+]- marker c8
        <<
        loop c6
    ] divided by 10
    store remainder:
    <<<< < c1
    ++++ ++++ ++ add 10 to compensate overcounting

    >>>> >>>> c9
    [>]find next available spot
    >++++++++[-<++++++>]< mark the end with a 48 (base for ascii digits)
    <+[-<+]- marker c8
    <<<< <<< c1
    [- move c1 to end
        >>>> >>>> c9
        [>]< end of array
        + place value
        <+[-<+]- c8
        <<<< <<< c1
    ]
    >>>> >>>>
    [>] find end of array
    <
    <+[-<+]- marker c8

    move quotient to input:
    <<<<- c4
    [-<<<+>>>] move c4 to c1
    >[-<<<< <+> >>>>] move c5 to c0

    clear c7 if input is empty:
    >>- mark quit
    <<<< << c1
    [
        >>>> >>+ mark that more decimals exist
        <<<< exit on c3
    ]c1 or c3
    >>>>> c6 or c8
    +[->+]- c8
    <<<< <<<< c0
    [
        >>>> >>>+ mark that more decimals exist
        <<<< exit on c3
    ]c0 or c3
    >>>>> c5 or c8
    +[->+]- c8
    <c7
    [if over 1 then set to 1
        [-]+
        <<<< c3
    ]c7 or c3
    >+[->+]- c8
    < c7
    loop c7
]
>c8
+ set marker to 0
>[>]< go to end of array
[.<] print in reverse until start


