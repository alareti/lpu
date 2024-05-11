# Logic Processing Unit

An LPU is an experimental approach to creating a processor based off second
order look up tables organized as a stack of directed acyclic graphs that
will allow for branching conditionals and loops.

## Connective Functions

Propositional calculus defines various functions that are useful for crafting
boolean logic. They are usually expressed as truth tables for clarity. For
example, the following _connectives_ should be familiar to the logic designer:

| Connective | Function symbol(s) | Number of Arguments |
| ---------- | ------------------ | ------------------- |
| AND        | ∧ & ⋅              | 2                   |
| OR         | ∨ \| +             | 2                   |
| XOR        | ^ ⊕ ⊻              | 2                   |

A truth table describes precisely what relationship a connective establishes
among its inputs. For example, one can organize every possible truth table
in a standard form, such that it forms a look up table for to deduce the
output of a connective given some input. Below are examples for the AND and
OR connectives.

AND Truth Table
| A | B | O |
|---|---|---|
| 1 | 1 | 1 |
| 1 | 0 | 0 |
| 0 | 1 | 0 |
| 0 | 0 | 0 |

OR Truth Table
| A | B | O |
|---|---|---|
| 1 | 1 | 1 |
| 1 | 0 | 1 |
| 0 | 1 | 1 |
| 0 | 0 | 0 |

Notice that the columns A and B remain constant in this standard form, such
that we can express a truth table with a shorthand composed of only
the (O)utput column. So, for instance, one can say the truth table for AND
is "1000" whereas the truth table for OR is "1110."

For a single input connective function, such as NOT (otherwise known as
an inverter), there will be only two characters in its associated truth table
shorthand, as demonstrated below.

NOT Truth Table
| A | O |
|---|---|
| 1 | 0 |
| 0 | 1 |

NOT = 01

And if we consider a connective function with _no_ arguments, it would only
have a single output. Below are the truth tables for the boolean value of
"true" and the boolean value of "false." Their function names are T and N,
respectively.

T Truth Table
| O |
|---|
| 1 |

T = 1

N Truth Table
| O |
|---|
| 0 |

N = 0

Given this background in connective functions (such as T, N, NOT, AND, etc.),
we are ready to take a look at Second Order Look Up Tables.

## Second Order Look Up Tables

Let's tabulate all possible connective functions, along with the names this
project assigns them, from 0 arguments all the way to 2 arguments.
There are 2 possible zero argument functions (which we've already named T and
N), 4 possible one argument functions, and 16 possible 2 argument functions.

| Truth Table | Name  | Symbol | Usage              |
| ----------- | ----- | ------ | ------------------ |
| 0           | N     | ⊥      | ⊥ or N             |
| 1           | T     | ⊤      | ⊤ or T             |
| 00          | NOP   | ⊥      | ⊥ A or NOP A       |
| 01          | OT    | ⊢      | ⊢ A or OT A        |
| 10          | NOT   | ¬      | ¬ A or NOT A       |
| 00          | OP    | ⊤      | ⊤ A or OP A        |
| 0000        | NOPE  | ⊥      | A ⊥ B or A NOPE B  |
| 0001        | NOR   | ⊽      | A ⊽ B or A NOR B   |
| 0010        | NORN  | ⩛      | A ⩛ B or A NORN B  |
| 0011        | NOTA  | ⬎      | A ⬏ B or A NOTA B  |
| 0100        | ANDN  | ⩑      | A ⩑ B or A ANDN B  |
| 0101        | NOTB  | ⬐      | A ⬐ B or A NOTB B  |
| 0110        | XOR   | ⊻      | A ⊻ B or A XOR B   |
| 0111        | NAND  | ⊼      | A ⊼ B or A NAND B  |
| 1000        | AND   | ∧      | A ∧ B or A AND B   |
| 1001        | NXOR  | ⊙      | A ⊙ B or A NXOR B  |
| 1010        | OTB   | ⬑      | A ⬑ B or A OTB B   |
| 1011        | NANDN | ⩚      | A ⩚ B or A NANDN B |
| 1100        | OTA   | ⬏      | A ⬏ B or A OTA B   |
| 1101        | ORN   | ⩒      | A ⩒ B or A ORN B   |
| 1110        | OR    | ∨      | A ∨ B or A OR B    |
| 1111        | OPE   | ⊤      | A ⊤ B or A OPE B   |

A remark on the naming scheme of the two-arity functions. There are three
main "fundamental" functions - namely AND, OR, XOR - that most of the rest of
the names derive from. NAND is equivalent to NOT AND, such that the result
of AND is inverted (A NAND B == NOT (A AND B)). So, for any fundamental
function, the prefix N inverts the result. This causes the rather well known
logic gate "xnor" to be expressed as NXOR in our scheme. A postfix N on any
of those three fundamental gates indicates that the second argument is inverted
before evaluating the fundamental function. Thus, A ANDN B evaluates to
A AND (NOT B). Combining these two rules, for instance on NANDN, gives
A NANDN B == NOT (A AND (NOT B)).

These two rules, as applied to AND, OR, and partially to XNOR
(there is no XNORN because XNORN == XNOR), make up the bulk of
the 2-arity functions. None of these functions are degenerate in the sense
that they can be expressed trivially be a 0 or 1-arity function.

There is another class of functions composed of OTA, OTB, NOTA, and NOTB.
As the names suggest, they either pass through the value of the first
argument A (or its complement) while ignoring B, or passes through B (or
its complement) while ignoring A. These essentially are functions which only
depend on 1 argument (e.g. NOTA's and OTA's outputs only depend on A), but
which happen to take in a second argument. These are pseudo-degenerate
functions, as they can fully be expressed by OT or NOT, so long as you
pass in the correct argument to that function.

The final class of the 2-arity functions consists of NOPE and OPE. NOPE,
as the name suggests, always outputs a 0, no matter what its inputs are.
This is a fully degenerate function, as it can be entirely replaced by the
0-arity function N. Likewise, OPE can be replaced by T. Similarly, NOP and OP,
from the 1-arity functions, also fully degenerate to T and N.

The goal of this section is to come up with a scheme such that these 22 rows
can fit within 16 rows, such that degenerate cases need not be taken account.
Essentially, we wish to be able to specify, effectively, which operation
in the above table we wish to compute given only 4 bits of information, as
opposed to the 5 that a naive attempt would suggest.

A few degenerate cases immediately grab our attention - namely, the equivalence
among N, NOP, and NOPE, as well as T, OP, and OPE. If we have four bits of
information, then we can essentially store this degenerate case as NOPE and OPE
while simply disregarding the other 4 functions, as their functionalities are
entirely encompassed by NOPE and OPE.

That leaves us with 18 functions. We've also noticed that OTA and OTB are
pseudo-degenerates of OT, and that NOTA and NOTB are pseudo-degenerates of NOT.
If, when we are performing our lookup functions on this table, and we need to
access NOT or OT functionality, we can set up a protocol such that we always
use input A instead of input B to perform the operation, since this effectively
is a single input function. That way, we don't need to support OTB / NOTB,
since the protocol ensures that it never will be needed. The means that we can
express any 0-arity, 1-arity, or 2-arity function using only a 2-arity (aka
second order) look up table. The new functionality looks like this:

| Truth Table | Name  | Symbol | Usage              | Comments |
| ----------- | ----- | ------ | ------------------ | -------- |
| 0000        | N     | ⊥      | ⊥                  |          |
| 0001        | NOR   | ⊽      | A ⊽ B or A NOR B   |          |
| 0010        | NORN  | ⩛      | A ⩛ B or A NORN B  |          |
| 0011        | NOT   | ¬      | ¬ A or NOT A       |          |
| 0100        | ANDN  | ⩑      | A ⩑ B or A ANDN B  |          |
| 0101        |       |        |                    | Reserved |
| 0110        | XOR   | ⊻      | A ⊻ B or A XOR B   |          |
| 0111        | NAND  | ⊼      | A ⊼ B or A NAND B  |          |
| 1000        | AND   | ∧      | A ∧ B or A AND B   |          |
| 1001        | NXOR  | ⊙      | A ⊙ B or A NXOR B  |          |
| 1010        |       |        |                    | Reserved |
| 1011        | NANDN | ⩚      | A ⩚ B or A NANDN B |          |
| 1100        | OT    | ⊢      | ⊢ A or OT A        |          |
| 1101        | ORN   | ⩒      | A ⩒ B or A ORN B   |          |
| 1110        | OR    | ∨      | A ∨ B or A OR B    |          |
| 1111        | T     | ⊤      | ⊤                  |          |

We no longer have the capacity to perform NOTB or OTB, since any NOT or OT
operation is guaranteed by protocol to be routed to input A. This also
imposes some overhead in that 0101 and 1010 are not useful to us anymore.
However, what we gained instead is the capacity to perform arbitrary 0, 1,
or 2-arity logical operations given only a second-order lookup table that
naively would have only been capable of performing 2-arity operations. These
fourteen operations are more than enough to perform arbitrary combinational
computation, and is flexible enough to form the processing core of the nodes
on a directed acyclic graph, the data structure which will form the backbone
of the logic processing unit.

## Data Operation Code

Every node on the DAG which comprises a "sub-block" of a program is limited
to these 14 operations. Each node can process either 0, 1, or 2 incoming edges,
where each edge represents a single bit of information coming from some
parent node. In order to allow for fanout, each node will be allowed to
have up to 2 output edges connecting itself to other downstream child nodes
which depend on the result of this operation. Thus, to completely describe
a node (in memory), one needs 1 bit for the first input value,
another bit for the second possible input value, 4 bits for the operation to
be performed by the node itself, and however many bits to evaluate the location
of this node's dependant's inputs.
