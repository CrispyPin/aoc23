adventofcode 2023 day 4
input example:
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

out0 out1 score0 score1
255 t0 t1 t2 t3 t4
255 win_0 win_1 etc

>>>>- set marker before t0
>>>> >>- set marker before cmp
<<<< < t0
, read "C"
[ for each line
    ,,, ,, read "ard x"
    [read until ":" (58)
        ,
        >++++ ++++
        [-<---- --->] subtract 8*7=56
        <-- subtract 2
    ]
    : reached
    , read " "
    [ read winning numbers until "|" (124)
        +[-<+]-> goto t0
        , digit or " " or "|"
        >[-]++++ ++++
        [-<---->] subtract " " (32)
        < t0
        [digit or "|"
            >++++
            [-<---->] subtract 16 from t0 to get digit value
            t1
        ] now at t0 or t1
        +[-<+]-> goto t0
        t0 now has digit value or "|" minus 48 (76)
        space interpreted as zero
        [->+>+<<] move t0 to t1 and t2
        ++++ ++++ +
        [->---- ----<] subtract 72 from t1
        >---- subtract 4
        t1
        [ t2 is a digit
           >t2
           [->>+++++ +++++<<] t2*10 in t4
           , read second digit to t2
            >++++ ++++ t3
            [-<--- --->] reduce t2 to digit value
            < t2
            [->>+<<] add t2(ones place) into t4
            >>t4
            > marker 1
            [>] find empty win slot
            +mark slot
            [<]> return to marker t4 (t3 is empty and everything between is not)
            [-> marker 1
                [>]< find last populated win slot
                +
                +[-<+]-return to marker 1
                < t4
            ]
            > marker 1
            [>]< find last populated win slot
            - unmark
            <[<] return to t4 (everything between will be nonzero)
            << t2
            ,[-] read the trailing space
        ]
        +[-<+]->> return to t1
        loop on t1 since its the input="|" marker
    ]
    all winner numbers parsed for this line
    >[-] reset t2
    << t0
    , read " "
    [read candidate numbers until \n=LF(10) is reached
        there is another number
        ,  digit or " "
        >++++ ++++
        [-<---->] subtract " " (32)
        <
        [not space so subtract 16 to get digit value
            >++++
            [-<---->]
            now at t1
        ] t0 or t1
        +[-<+]-> goto t0
        [->>+++++ +++++<<] digit*10 in t2
        , guaranteed digit in t0
        >++++ ++++
        [-<------>] subtract 48 from t0
        < t0
        [->>+<<] add t0 to t2

        >>>>> goto marker
        [>]- mark end of array
        [<] goto t4

        check numbers while t3 is set
        <+ set t3
        [ COMPARISON for all winning numbers
            (copy t2 to t0):
            <<<[-] clear t0
            >>[-<+<+>>] move t2 to t0 and t1
            <[->+<] move t1 back to t2

            >>>> goto marker
            >+[->+]-< go to last element
            [ (subtract next number in win list to compare):
                - dec element
                <+[-<+]-< return to t4
                <<<< goto t0
                - subtract for comparison
                >>>>> goto marker
                >+[->+]- goto end marker
                >+ place copy of number after the marker
                << return to last element
            ]
            (at old pos of last element)
            t0 is now 0 if this was a winning number
            ->+ move end marker to the left
            <<+[-<+]-< return to t4
            <<<<<<+ mark score1
            >>>>>
            - unmark t3 (exit loop)
            <<< t0
            [ numbers are not equal
                not a winning number (yet)
                << - unmark score1
                >>> t1 is empty; use to exit
            ]
            now t0 or t1
            t0 might be 255
            > t1 or t2
            +[->+]-> first array element
            +[-
                array has remaining values
                looping will continue
                <<<+mark t3
                > exit on t4
            ] t4 or first array element
            <+[->+]- goto marker
            <<t3
            loop on t3
        ]
        array is empty and/or winning number confirmed
        > t4
        [
            -
            add to temp score
            <<<<t0
            <<+>>>t1 should be empty
        ] t4 or t1
        +[->+]- goto marker (t0 might be 255 from subtraction

        (reorder winner array)

        >[+] remove end marker (its still there if there was a winning number)
        >> first in the shifted array
        [>]- mark end of array
        < last element
        [
            -
            <+[-<+]- goto start marker
            >>+ move value to beginning of array
            +[->+]-< return to last element
        ]
        < new last element
        [
            -
            <+[-<+]- goto start marker
            >+ move value to beginning of array
            +[->+]-<< return to last element
        ]

        >>+ remove marker
        <<< last element
        [<] return to t4
        <<[-] clear t2
        << t0
        , ---- ---- -- subtract LF(10)
        NOTE: gets stuck if there is no trailing newline
        exit loop if newline reached
    ] t0
    (clear win array)
        >>>>> marker
        > first array element
        [[-]>] clear array
        +[-<+]- return to marker
        <<<<< t0
    TODO add score to total

    (score calculations)
    <<[->>+>+<<<] move score to t0 and t1
    check if 9 or 10 (256 or 512)
    <<<++ mark high byte as if score was 512
    >>>>> t0
    ----- -----
    [
        not equal to 10
        <<<<<- unmark high byte for 512
        >>>>>+
        [
            not equal to 9
            <<<<<- unmark high byte for 256
            >>>>> [+]reset t0
            >t1
            [-
                for each match
                <<< score1 low byte
                [->>+<<] move score to t0
                >>t0
                [double the score; it was not zero
                    [-<<+<+>>>]move t0 to score0 and score1
                    <<<[->+<] add score0 to score1
                    >>>t0 is empty now
                ]
                <<<+ add init to score0
                > score1
                [ if score is already set; remove init score
                    <-
                ]score0 or score1
                +[->+]- goto marker
                <<[->+<] move init score if any to score1
                >>>> t1
            ]
            add to total
            <<<score1
            [-
                <+ mark score0 that wrapping happened
                <+ add to total1
                [total1 did not wrap
                    >- unmark wrap (score0)
                    >>> t0 is empty
                ]total1 or t0
                total1 might be 255
                >>>score0 or t3
                +[-<+]- goto marker
                <score1
            ]
            < score0; wrap marker
            [ apply wrap
                <<+>>-
            ]
            >>>return to t0
        ]
    ]
    ,loop on first character of the line (exits if all input is consumed)
]
<+ remove first marker
>>>>>> + remove array marker
<<- marker at c8

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


