A balanced implementation of a file system which can contain small changes 
efficiently and can be read without great difficulty.

This library is supposed to be used for data management which accumulates small changes,
like auto-save or version control.

If we accumulate changes linearly, it's basically the most space-efficient strategy.

>(A:0, B:0)

(diff A→1)

>(A:1, B:0)

(diff B→1)

>(A:1, B:1)

To get the last state, at first we read the file (A:0, B:0), 
then read the first diff file (A→1) and apply(A:1, B:0),
then read the second diff file (B→1) and apply(A:1, B:1),

However, obviously it's not a sustainable way.
If we create 1,000,000 diff files, we need to open and apply them 1,000,000 times, 
and no files can be removed to get the latest state.

On the contrary, We can derive everything from the initial state.

>Initial State (A:0, B:0)
> 
>First State (A:1, B:0) diff (A→1)
> 
>Second State (A:1, B:1) diff (A→1, B→1) from Initial State

To get the second state, open the inial state file then 
open the second diff file and apply. 
We can ignore the first diff file, and we can remove it if we'd like.

Even if we have 1,000,000 files, we can open only 2 files to get any state.

But diffs can be very big.

>Initial State (100 KB)
>
>First State (10 MB diff from Initial State)
>
>Second State (10 MB diff from Initial State)

It requires 20 MB + 100 KB storage space to store. 
But the diff from the first state to the second state may be 2 KB.
If so, if we use the diff, required storage space will be 10 MB + 102 KB, 
so We can almost save the storage space of the entire state.

So we need a balanced strategy. Basically we use a 4-Phase strategy.

>Initial
> 
>A1 (1 MB from Initial)
> 
>B1 (1.1 MB from A1)
> 
>C1 (0.9 MB from B1)
> 
>D1 (0.8 MB from C1)

Phase-D is the last phase. The next diff file is D2

>Initial
> 
>A1 (1 MB from Initial)
> 
>B1 (1.1 MB from A1)
> 
>C1 (0.9 MB from B1)
> 
>D1 (0.8 MB from C1) D2(1.5 MB from C1)

It's 4-Phase, so we only need 4 files to get any state. 
Maybe 1.5 MB is acceptable, but it will be unacceptable sometime.

>D1 (0.8 MB from C1) D2 (1.5 MB from C1)...D15 (10 MB from C1)

If D15(10 MB) is unacceptable, we can make C2

>Initial
> 
>A1 (1 MB from Initial)
> 
>B1 (1.1 MB from A1)
> 
>C1 (0.9 MB from B1) C2 (11 MB from B1)

After that, We can derive from C2

>D2-1 (1 MB from C2) D2-2 (1.8 MB from C2)...

if D2-X is unacceptable, we can make C3.
If CX is unacceptable, we can make B2,
if BX is unacceptable, we can make A2,
if AX is unacceptable, we can't help it.

The line we think it's unacceptable is important. we'll talk about it later.

We can make "Cumulative-Phase" the last phase. 

>C1 (1 MB from B1)
> 
>D1 (1 MB from C1) D2 (1.1 MB from D1) D3 (0.7 MB from D2)...

In the Cumulative-Phase, we use linear deriving strategy
to store small changes efficiently.
We need to open more files to get a state instead.

When is the file size regarded as unacceptable?
Our strategy is to reduce total storage space. 
When we shift phases, if the total storage space is reduced, shift them.

However, we can't know future file sizes precisely. 
So we assume "the last phase will be repeated."
It's not true, but I think it's a reasonable compromise.

For example, 

>C3 (1 MB)
> 
>D1 (300 KB)

We have 1 MB of C3 and 300 KB of D1. The average is 1.3 / 2 = 0.75.

And we assume the last one (300 KB) will be repeated.

>C3 (1 MB)
> 
>D1 (300 KB) D2(300 KB)
> 
>1.6 / 3 = 0.66..
 
The average is reduced from 0.75 to 0.66. So we continue Phase-D.

>C3 (1 MB)
> 
>D1 (300 KB) D2 (500 KB)
> 
>(1 + 0.3 + 0.5) / 3= 0.6
> 
>(1 + 0.3 + 0.5 + 0.5) / 4 = 0.575

The real D2 was 500 KB. If 500 KB was repeated, the average would be 0.575 from 0.6. So we continue. 

>C3 (1 MB)
> 
>D1 (300 KB) D2 (500 KB) D3 (700 KB)
> 
>(1 + 0.3 + 0.5 + 0.7) / 4 = 0.625
> 
>(1 + 0.3 + 0.5 + 0.7 + 0.7) / 5 = 0.64
 
The real D3 was 700 KB. If D3 was repeated, the average would increase from 0.625 to 0.64. 
So we shift to Phase-C.

And the Phase-C's situation was below.

>B4 (3MB)
> 
>C1 and descendants (1 MB)
> 
>C2 and descendants (2 MB)
> 
>C3 and descendants (2.5 MB)
 
We assume if we continue Phase-C, C3 will be repeated. The average will be

>3 + 1 + 2 + 2.5 + 2.5 / 5 = 2.2
 
The current average is

>3 + 1 + 2 + 2.5 / 4 = 2.125
 
So if we continue Phase-C, the average would increase. So we shift to Phase-B.

That's the calculation. When we use Cumulative-Phase, the calculation doesn't change.

Cumulative-Phase's file sizes may not vary largely. In that case, phase-shift doesn't occur often.

It would result too many files to get a state, so customize the conditions to shift phases if you need.
 


