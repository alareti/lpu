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

TT Truth Table
| O |
|---|
| 1 |

T = 1

NT Truth Table
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
| 0           | NT    | ⊥      | ⊥ or NT            |
| 1           | TT    | ⊤      | ⊤ or TT            |
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

These two rules, as applied to AND, OR, and partially to NXOR
(there is no NXORN because NXORN == NXOR), make up the bulk of
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

| OpCode | Truth Table | Name      | Symbol    | Usage (for inputs A and B)    | Comments              |
| ------ | ----------- | --------- | --------- | ----------------------------- | --------------------- |
| 0000   | 0           | NT        | ⊥         | ⊥ = NT = 0                    | Ignore inputs         |
| 1000   | 0001        | NOR       | ⊽         | A ⊽ B = A NOR B = !(A \| B)   |                       |
| 0100   | 0010        | NORN      | ⩛         | A ⩛ B = A NORN B = !(A \| !B) |                       |
| 1100   | 01          | NOT       | ¬         | ¬ A = NOT A = !A              | Only evaluate input A |
| 0010   | 0100        | ANDN      | ⩑         | A ⩑ B = A ANDN B = A & !B     |                       |
| 1010   | Undefined   | Undefined | Undefined | Undefined                     | Reserved              |
| 0110   | 0110        | XOR       | ⊻         | A ⊻ B = A XOR B = A ^ B       |                       |
| 1110   | 0111        | NAND      | ⊼         | A ⊼ B = A NAND B = !(A & B)   |                       |
| 0001   | 1000        | AND       | ∧         | A ∧ B = A AND B = A & B       |                       |
| 1001   | 1001        | NXOR      | ⊙         | A ⊙ B = A NXOR B = !(A ^ B)   |                       |
| 0101   | Undefined   | Undefined | Undefined | Undefined                     | Reserved              |
| 1101   | 1011        | NANDN     | ⩚         | A ⩚ B = A NANDN B = !(A & !B) |                       |
| 0011   | 10          | OT        | ⊢         | ⊢ A = OT A = A                | Only evaluate input A |
| 1011   | 1101        | ORN       | ⩒         | A ⩒ B = A ORN B = A \| !B     |                       |
| 0111   | 1110        | OR        | ∨         | A ∨ B = A OR B = A \| B       |                       |
| 1111   | 1           | TT        | ⊤         | ⊤ = TT = 1                    | Ignore inputs         |

We no longer have the capacity to perform NOTB or OTB, since any NOT or OT
operation is guaranteed by protocol to be routed to input A. This also
gains us flexibility since 0101 and 1010 are not useful to us anymore.
However, what we gained instead is the capacity to perform arbitrary 0, 1,
or 2-arity logical operations given only a second-order lookup table that
naively would have only been capable of performing 2-arity operations. These
fourteen operations are more than enough to perform arbitrary combinational
computation, and is flexible enough to form the processing core of the nodes
on a directed acyclic graph, the data structure which will form the backbone
of the logic processing unit.

One will also notice that the OpCode representation mirrors the truth
table representation. That is so that the opcode has a dual function:
It not only specifies what operation to perform, but it also serves as
a look up table for all 2-arity functions. This allows one to implement
an efficient datapath in silicon as any 2-arity operation can be implemented
as a simple 4-to-1 multiplexer.

## Data Operation Code

Every node on the DAG which comprises a "sub-block" of a program is limited
to these 14 operations. Each node can process 2 incoming edges (potentially
ignoring one or two of them),
where each edge represents a single bit of information coming from some
upstream node. In order to allow for fanout, each node
defines two output edges which (potentially) connects itself to other
downstream child nodes
which depend on the result of this operation. Thus, to completely describe
a node (in memory), one needs 1 bit for the first input value,
another bit for the second possible input value, 4 bits for the operation to
be performed by the node itself, and however many bits to evaluate the location
of this node's dependant's inputs.

Any directed acyclic graph can be topologically sorted into layers where
each layer contains nodes which do not depend on each other directly,
though they may have common upstream dependencies which reside in previous
layers. All nodes with no upstream
dependencies whatsoever belong in the first layer, and all nodes with no
downstream dependents whatsoever belong in the last layer. Direct dependents
will always be found in the subsequent layer, assuming that they don't
depend on nodes which in turn depend on this node. In which case,
they will always reside in the layer subsequent the _intervening_ node.

To completely specify a node, we must know three things: the operation it
performs, the data it must process (i.e. its input), and where it must
store its output (i.e. the locations of its dependent nodes). We have
already defined a protocol for specifying a node's inputs and its operation
in memory. At some definite location in memory, we have so far 6 meaningful
bits. The first bit corresponds to the first input (IA). The second bit
corresponds to the second input (IB). The next four bits (OPCODE) correspond to
the operation to be performed (see chart above). For each OPCODE, there
is an associated truth table which defines the values of the two output
edges emanating from the node (OA and OB for Output A and Output B). For
all OPCODEs defined above, OA = OB. However, we have a couple
undefined OPCODEs, and those could be useful later for adding
functions where OA does not equal OB, or a function which outputs
nothing (essentially a no-op).

Lastly, we must define a protocol for where each node must store its output.
For each node, there are two outputs, OA and OB. And associated with each
output is a link (labeled LA and LB). We can specify that any OA must be
routed to some node's IA, and similarly, any OB must be routed to some
dependent node's IB. Thus, for some Node1 connected to some Node2 and Node3,
`OA_1` can connect to `IA_2` or `IA_3`, but cannot connect to `IB_2` or `IB_3`,
whereas `OB_1` can connect to `IB_2` or `IB_3`, but cannot connect to
`IA_2` or `IA_3`.

This, however, presents an issue as there exist non-symmetric OPCODEs where
the order in which one presents an input matters (namely all postfix-N
operations). Additionally, there is no way for us to call the NOT or OT
functions on any OB, since they only operate on input A.

This, however, is easily remedied by reintroducing the OTB and NOTB operations
discussed earlier, but reinterpreting them as _switches_. Their purpose
is not computation so much as to be able to take some IB and make it
available on OA, such that some other downstream node can take it in as
its IA. As such, we add them back to the OPCODE list as SW and NSW, where
SW stands for "switch." This introduces something of a bias for A into the
naming conventions, but the reality is that we now are perfectly
symmetric in operation and can route any OA to IB and vice-versa by
either introducing a buffer OT or SW between the two linked nodes.
Similarly, we can substitute these buffers with their prefix-N counterparts
should we wish to simulate some postfix-N operation. As such,
we alter the names of the codes themselves to reflect this - OT becomes
PA ("Pass through A"), NOT becomes NA ("Not A"), and similar mnemonics for
PB and NB.

As such, we remove the postfix-N operations, as they no longer add much
value to our set of OPCODEs, and we want space for more useful operations
like addition or no-op. The updated chart is shown below.

| OpCode | Name      | Sensitivity | OA Table  | OB Table  |
| ------ | --------- | ----------- | --------- | --------- |
| 0000   | NT        | None        | 0         | 0         |
| 1000   | NOR       | IA and IB   | 0001      | 0001      |
| 0100   | Undefined | Undefined   | Undefined | Undefined |
| 1100   | NA        | IA          | 01        | 01        |
| 0010   | Undefined | Undefined   | Undefined | Undefined |
| 1010   | NB        | IB          | 01        | 01        |
| 0110   | XOR       | IA and IB   | 0110      | 0110      |
| 1110   | NAND      | IA and IB   | 0111      | 0111      |
| 0001   | AND       | IA and IB   | 1000      | 1000      |
| 1001   | NXOR      | IA and IB   | 1001      | 1001      |
| 0101   | PB        | IB          | 10        | 10        |
| 1101   | Undefined | Undefined   | Undefined | Undefined |
| 0011   | PA        | IA          | 10        | 10        |
| 1011   | Undefined | Undefined   | Undefined | Undefined |
| 0111   | OR        | IA and IB   | 1110      | 1110      |
| 1111   | TT        | None        | 1         | 1         |

Let's return to defining LA and LB (the output linkage data). So far,
we've decided that all OA gets routed to some IA, and all OB gets routed
to some IB, so nothing needs to be encoded there. Also, there is no need
to route to some node on the current or previous layer of a DAG, and so at
a minimum the lateral movement starts at the subsequent layer of the DAG.
Similarly, we want to exploit longitudinal locality, meaning that most often
a node is going to be connected to some node nearby itself.

When I speak of lateral or longitudinal movement, think of the directed graph
as a tabular grid with slots in them of where the nodes will be placed.
A node, then, can be located by a layer index and a depth index, where the
origin is the top-left most corner of the grid, the layer index determines
how far to the right (lateral) the node is, and the depth index determines
how far down (longitudinal) the node is. Not every cell of the grid has
to be filled (in fact, when viewing it like this, most will probably be
empty for any real combinational circuit). This theoretical empty space
need not affect the actual performance of our computation, but it
still serves as a good mental model for this discussion.

LA and LB should be thought of as relative offsets on this grid that
point from the current node to IA of the destination node specified by LA
and IB of the destination node specified by LB. We have two dimensions of
movement - lateral (moving left or right) and longitudinal (moving
either up or down). Now, since this is a sorted directed acyclic graph,
any link _must_ point to the right, starting at the layer directly
following the layer the node resides. Also, given our ability to buffer
data using PA, PB, NA, and NB, we are not too constrained in our overall
lateral movement. As such, we could assign one unsigned bit to specifying
lateral movement, which pretty much states that the dependent node resides
on either the next layer or the layer directly after that one. All the
rest of the available bits should be assigned to signed longitudinal movement.
It is signed, since it is possible for a node to connect to a node that
is longitudinally above or below it (assuming, of course, that it is
not on the same or previous layer).

Since the absolute depth index starts at 0 and only increases, we want to have
slightly more options of increasing as opposed to decreasing the
longitudinal relative offset. Unfortunately, two's complement representation
has more options for negative numbers than for positive numbers (e.g. 5 bits
allows you to express the range \[-16, 15\]). As such, we'll store the
negative of the actual offset, and have the depth index start at 0 and
have it _decrease_ into the negative numbers to indicate _increasing_
depth.

Since we're going for a compact representation of a node, we should aim to
fit all 5 components (IA, IB, OPCODE, LA, LB) into at most 16 bits. As
we've said before, IA and IB each take up one bit, OPCODE takes up 4 bits,
and we already have one bit each for LA and LB assigned to lateral offset.
That means that we have 8 bits left for longitudinal offset, split evenly
for LA and LB. However, we need to reserve two bits in the 16 bit word for
other control purposes. That leaves only 3 longitudinal bits each for LA and
LB. However, that will have to do, letting them each specify a node either
4 down or 3 up from their current position. This may be insufficient
and we may want to redefine this protocol to have _no_
latitudinal bits and only longitudinal bits. That would mean that every node
can only connect to a node on the next layer, no longer able to do so
on the layer after, needing intermediaries to meet those nodes.

Right now, we will take the latter approach, and dedicate all link
data for longitudinal movement and neglect latitudinal movement. That means
we get 4 bits for each link, giving each link a range from -7 to 8.

This is the breakdown of the 16 bits which constitute a data op.

| CTL    | OPCODE  | IA    | IB    | LA       | LB        |
| ------ | ------- | ----- | ----- | -------- | --------- |
| `2b00` | `[2:5]` | `[6]` | `[7]` | `[8:11]` | `[12:15]` |

CTL has a width of 2, OPCODE a width of 4, IA a width of 1, IB a width of 1,
LA a width of 4, and LB a width of 4. LA and LB essentially act as pointers
to its dependent nodes' IA and IB, respectively. A CTL value of `00` is
what indicates to the algorithm that what it is dealing with is, in fact,
a data op.

## The Jumper Modifier

There are times when a link offset is not expressible with only 4 bits.
The link offset included in the data op is designed to take advantage of
locality, but when that is not an option for a particular link, there
should be a mechanism for extending the range of that link. The jumper
modifier aims to fill that gap.

Though we previously imagined the DAG to be a two dimensional grid structure,
in memory it will be stored as a contiguous piece of memory, topologically
sorted such that a process can simply cycle through the ops sequentially.
As such, we can place a _modifier_ op before a data op
to indicate that the data op needs to be treated in a different manner
than usual. This allows these modifiers to remain somewhat transparent
to the data processing, and essentially gives each node more capabilities
by simply tagging it with one of these modifier instructions.

The jumper modifier can conceivably come in either a
one-dimensional or two-dimensional flavor (i.e. 1D and 2D). A 1D
jumper means that there is a bit which indicates whether the jumper
is making either a longitudinal or a lateral link. The rest of the bits
of that 16 word indicates how big the offset is. Lateral is unsigned
with an implicit start of 1, and lateral is signed with an implicit
negative bias (reasoning given above when describing LA and LB).

A 2D jumper is similar except you specify both a lateral and a longitudinal
offset. Half the available bits are dedicated to the lateral offset, and
the other half to the longitudinal offset.

In addition, a jumper can specify which output it wishes to forward. A 1 output
jumper only forwards a specified output to the target. A 2 output
jumper forwards both outputs to the target. An independent jumper
specifies two different links - one for each output. Each of these links
are specified as either a 1D or 2D link. Below is a chart to sort out
what different options are available. There are twelve options,
which will take 4 bits to sort out, plus the required two control bits.
That leaves 10 bits available for defining links

| Link Type | Capabilities                                               | Bits per link |
| --------- | ---------------------------------------------------------- | ------------- |
| OALAT     | Forwards only OA latitudinally only                        | 10            |
| OALON     | Forwards only OA longitudinally only                       | 10            |
| OA2D      | Forwards only OA in both dimensions                        | 5             |
| OBLAT     | Forwards only OB latitudinally only                        | 10            |
| OBLON     | Forwards only OB longitudinally only                       | 10            |
| OB2D      | Forwards only OB in both dimensions                        | 5             |
| ABLAT     | Forwards both OA and OB to same target latitudinally only  | 10            |
| ABLON     | Forwards both OA and OB to same target longitudinally only | 10            |
| AB2D      | Forwards both OA and OB to same target in both dimensions  | 5             |
| OOLAT     | Forwards both OA and OB independently latitudinally only   | 5             |
| OOLON     | Forwards both OA and OB independently longitudinally only  | 5             |
| OO2D      | Forwards both OA and OB independently in both dimensions   | 2             |

Given that normal links already have available 4 bits, it seems that these
jumper do not give much advantage unless we restrict ourselves
to LAT or LON link types and ignore the OO link types. Given that,
we now only have six options, meaning we also gain an additional bit we can
devote to specifying the link offset. Below is the updated table.

| OPCODE | Link Type | Capabilities                                               | Bits per link |
| ------ | --------- | ---------------------------------------------------------- | ------------- |
| 100    | OALAT     | Forwards only OA latitudinally only                        | 11            |
| 101    | OALON     | Forwards only OA longitudinally only                       | 11            |
| 010    | OBLAT     | Forwards only OB latitudinally only                        | 11            |
| 011    | OBLON     | Forwards only OB longitudinally only                       | 11            |
| 110    | ABLAT     | Forwards both OA and OB to same target latitudinally only  | 11            |
| 111    | ABLON     | Forwards both OA and OB to same target longitudinally only | 11            |

We will also specify that any one data node will have, at most, one
modifier preceding it. That might mean that it takes multiple
intermediary jumper in order to reach a target, but it will help
simplify processing logic. A jumper modifier is specified with a CTL
sequence of `01`.

| CTL    | OPCODE  | OFFSET   |
| ------ | ------- | -------- |
| `2b01` | `[2:4]` | `[5:15]` |

Now, there is one more additional concern about these jumper. We'd
like to make its operation as independent as possible from the
data processing that the actual data op is going to make. In other words,
the modifier should be _transparent_ to the processing of a data op.
The only difference that this modifying instruction makes is that,
_in addition_ to whatever the data op does, it also indicates that
the output be forwarded, as well, to the specified location. This implies
that it does not prevent the data op from also establishing its own links
as specified in that section.

## CAP, ADD, and SUB

However, there are times when we _do_ want the data op to _effectively_
terminate its local updates, and cease influencing that region of the graph.
In that case, what we need is a data op which takes in input but
does not output anything. It is like a no-op, but which blocks until
its input has arrived, yet does nothing beyond that. We will call it CAP
(like a bottle cap) to indicate that it caps or stops that link. Below
is the updated data op table to show its addition.

| OpCode | Name      | Sensitivity | OA Table  | OB Table  |
| ------ | --------- | ----------- | --------- | --------- |
| 0000   | NT        | None        | 0         | 0         |
| 1000   | NOR       | IA and IB   | 0001      | 0001      |
| 0100   | CAP       | IA or IB    | None      | None      |
| 1100   | NA        | IA          | 01        | 01        |
| 0010   | Undefined | Undefined   | Undefined | Undefined |
| 1010   | NB        | IB          | 01        | 01        |
| 0110   | XOR       | IA and IB   | 0110      | 0110      |
| 1110   | NAND      | IA and IB   | 0111      | 0111      |
| 0001   | AND       | IA and IB   | 1000      | 1000      |
| 1001   | NXOR      | IA and IB   | 1001      | 1001      |
| 0101   | PB        | IB          | 10        | 10        |
| 1101   | Undefined | Undefined   | Undefined | Undefined |
| 0011   | PA        | IA          | 10        | 10        |
| 1011   | Undefined | Undefined   | Undefined | Undefined |
| 0111   | OR        | IA and IB   | 1110      | 1110      |
| 1111   | TT        | None        | 1         | 1         |

We've defined it such that it is sensitive to either IA _or_ IB. This means
that it only ever expects one input, and that feeding it two inputs is
undefined behavior, but that it could be either IA or IB.
We can apply this same logic to NA, NB, PA, and PB,
and consolidate them back into NOT and OT, while specifying their
sensitivity lists to be either IA _or_ IB, indicating that this function
only expects one input. This frees up two more op codes.

| OpCode | Name      | Sensitivity | OA Table  | OB Table  |
| ------ | --------- | ----------- | --------- | --------- |
| 0000   | NT        | None        | 0         | 0         |
| 1000   | NOR       | IA and IB   | 0001      | 0001      |
| 0100   | CAP       | IA or IB    | None      | None      |
| 1100   | NOT       | IA or IB    | 01        | 01        |
| 0010   | Undefined | Undefined   | Undefined | Undefined |
| 1010   | Undefined | Undefined   | Undefined | Undefined |
| 0110   | XOR       | IA and IB   | 0110      | 0110      |
| 1110   | NAND      | IA and IB   | 0111      | 0111      |
| 0001   | AND       | IA and IB   | 1000      | 1000      |
| 1001   | NXOR      | IA and IB   | 1001      | 1001      |
| 0101   | Undefined | Undefined   | Undefined | Undefined |
| 1101   | Undefined | Undefined   | Undefined | Undefined |
| 0011   | OT        | IA or IB    | 10        | 10        |
| 1011   | Undefined | Undefined   | Undefined | Undefined |
| 0111   | OR        | IA and IB   | 1110      | 1110      |
| 1111   | TT        | None        | 1         | 1         |

Since we now have five extra opcodes, it is high time to add
addition and subtraction to our list of functionality. Also,
we'll reorganize our table such that it is by ascending opcode.

| OpCode | Name      | Sensitivity | OA Table  | OB Table  |
| ------ | --------- | ----------- | --------- | --------- |
| 0000   | NT        | None        | 0         | 0         |
| 0001   | AND       | IA and IB   | 1000      | 1000      |
| 0010   | Undefined | Undefined   | Undefined | Undefined |
| 0011   | OT        | IA or IB    | 10        | 10        |
| 0100   | CAP       | IA or IB    | None      | None      |
| 0101   | Undefined | Undefined   | Undefined | Undefined |
| 0110   | XOR       | IA and IB   | 0110      | 0110      |
| 0111   | OR        | IA and IB   | 1110      | 1110      |
| 1000   | NOR       | IA and IB   | 0001      | 0001      |
| 1001   | NXOR      | IA and IB   | 1001      | 1001      |
| 1010   | ADD       | IA and IB   | 0110      | 1000      |
| 1011   | SUB       | IA and IB   | 0110      | 0010      |
| 1100   | NOT       | IA or IB    | 01        | 01        |
| 1101   | Undefined | Undefined   | Undefined | Undefined |
| 1110   | NAND      | IA and IB   | 0111      | 0111      |
| 1111   | TT        | None        | 1         | 1         |

ADD and SUB implement the standard half adder and half subtracter
circuits, where OA is sum / diff and OB is carry / borrow, respectively.
ADD and SUB are unique in this table since only they have outputs
such that OA is not equivalent to OB. CAP is unique, of course, in that
it has _no_ output.

## The Pass Gate

A transmission gate allows for tri-state logic. Essentially, it defines
whether a particular edge in a graph is either driven high, low,
or intentionally left un-driven. This is called a high impedance state,
and is usually labeled X on a truth table. This leads us to discussing
the life-cycle of a graph. A graph is a series of topologically sorted
nodes that are connected by edges. For our purposes, we only
allow ourselves to define a certain type of graph called a
directed acyclic graph (DAG), and further, that each node, generally speaking,
takes in up to two edges and outputs two edges. This is enough for
_describing_ a DAG - simply specify the nodes and how their
associated edges.

However, we wish to do more than that. We wish to _evaluate_ a DAG.
Luckily, a DAG implies that there are a set of root nodes which
take in no incoming edges and a set of leaf nodes which take in no
incoming edges. These are analogous to the inputs and outputs of a graph,
and usually topological sorting puts root nodes as close to the front
of the list, and leaf nodes at the back of the list.

Regardless, there exists an easy method for evaluating a DAG - iterate
through a topologically sorted list of nodes and evaluate them one
at a time. This ensures that a node's input(s) will always be available,
as its dependencies outputs have already been evaluated, and are
available for processing by the current working node.

This model falls apart, however, when we start to add more cores
evaluating the DAG independently. Ideally, we would want each N cores
to process N sequential data ops independently (we're ignoring
control ops for now). One must remember that each data op should
have enough information contained within itself to allow itself
to completely be evaluated by a core running in parallel with other cores
which are also simultaneously evaluating data ops in the close vicinity
of the current data op.

A core theoretically, then, only has the 16 bits presented to it to
determine the output value of that node. However, it could very well
be possible that some other core is busy processing one if its
_direct descendants_. This would be disastrous, since that core
had not yet finished writing its output to the input of the current
core's data input (IA and IB) fields of the current op. That constitutes
a data hazard, since it implies that we have somehow incorrectly
processed a data op, and were operating on garbage data which had
nothing to do with what we cared about doing.

The actual implementation of the mechanism which resolves this hazard
is left for another section, but I wish to give the general idea of
the approach taken, as it relates directly to how we evaluate the
transmission gate. The basic idea is that we must encode the op in such
a way that we are informed that its input has been provided and is
ready to be evaluated, as opposed to simply just providing the data
levels (high or low) which constitutes the meaning of that data.
This is an example of _event_ triggered logic (the event being
the input going from an unready to a ready state, similar to
triggering on clock edges) as opposed to _level_ driven logic, which implies
that we only are sensitive to what the _value_ of the data is
at our inputs.

This is why for each opcode, there is an associated sensitivity list:
what _event_ causes an operation to be evaluated? For some, both
inputs need to be available for to evaluate the op (like OR and XOR)
For others, they specify no incoming edges - they must be left
(like TT and NT).

The last group of sensitivity lists is somewhat special, in that
they require one _activated_ incoming graph edge in order to
allow them to process, yet can host either one or two incoming edges,
simultaneously (so long as no more than one is ever active at any point).

This creates a conundrum for the two-input case, since _all_ ops
defined so far necessarily activate their output links once evaluated.
There is no way for any of the nodes to conditionally activate or
deactivate their output, and so if any such nodes connect to two of
these "one or the other" (think of the OT op), this leads to
undefined behavior as there exist two active inputs on what should
intuitively be single input nodes.

However, it is very useful to be able to specify "one or the other" logic.
One could do so with a multiplexer, and to good effect, with pure
logic gates, bypassing the need for tri-state logic. However, OT is
also useful for "path-switching." Right now we've defined our
protocol such that a OA always routes to some IA, and an OB always
routes to some IB. It would be impossible to pipe some B path to
an A path using only the dual-input-triggered logic ops defined above.
We could define two sets of OT, like a "fan-out A" and "fan-out B" ops
(which if you remember, was what the old NA, NB, PA, and PB ops did),
but this feels rather wasteful, and also limits us to never
being able to specify "one or the other" logic in the future - it only
acts as a stop-gap in order to give us path switching at the cost
of two extra ops (in an already very limited set of 16!).

So, we include the transmission gate, which is the only op capable
of conditionally activating its output. It will be named PG (short
for "pass gate"), and its truth table is as follows.

PG Truth Table
| A | B | O |
|---|---|---|
| 1 | 1 | 1 |
| 1 | 0 | X |
| 0 | 1 | 0 |
| 0 | 0 | X |

X is the "high impedance" state. It essentially means that the edge
is intentionally being un-driven. All other gates, once evaluated,
drive their respective outputs either high (logic 1) or low (logic 0).
Thus, only PG ops can safely be attached to OT nodes that have more
than one input edge specified, as they expect that at any point in
time at most only one input will ever be activated (a.k.a. "driven").

So, when processing a DAG, each edge has four possible states: unevaluated,
driven low, driven high, and high impedance (i.e. "un-driven"). To
return back to our mental model of the multicore machine processing
this DAG, this information must be encoded in either the protocol or the
data itself in order for them to evaluate ops properly and atomically (i.e.
without needing to look at the graph as a whole in order to figure out
whether the current op is ready to be processed). We update our table
of ops accordingly. We also add another column to show the relationship
between the Sensitivity list and the minimum / maximum amount of incoming
edges a node supports.

We've also changed the definition of CAP such
that it supports any combination of input triggers (i.e. if no inputs
are driven, if no inputs are even evaluated, if some inputs are driven
and some are high impedance and some are not evaluated, and if all
inputs are evaluated). In other words, once a processor comes across
a CAP op, it can immediately evaluate it regardless of its inputs. This
is different behavior from NT and TT which have sensitivities of "None"
since they don't expect _any_ input whatsoever. They are not allowed
to have incoming edges, whereas CAP can have 0, 1, or 2 incoming edges
at whatever one of the four states of evaluation mentioned previously.

Another point to mention is that when a processor comes across let's
say an AND node which expects two active driven (i.e. in one of either
"driven high" or "driven low" of the four states), it is undefined
behavior to have the state of its incoming edge be in the "high impedance"
state. This implies that PG is only really useful for connecting to
either OT or NOT nodes (or potentially CAP). Another thing to realize
is that, for such "dual-sensitive" ops (which cover those ops with
sensitivity lists that say "IA and IB"), it is _not_ undefined
behavior for the two incoming edges to be in the "unevaluated" state,
since that could mean (assuming a well-behaved graph) that its
dependencies simply have not been evaluated yet. The core would then
be free to either wait until the state turns into some sort of active
driven, or it can go on and evaluate other nodes in the process.
Eventually some core will return back to it (not necessarily the same one)
in order to further along the processing of that particular graph.

I'm also going to reorder some of the ops around (CAP, PG, ADD, and SUB)
to align better with some underlying patterns.

| OpCode | Name      | Min / Max Inputs | Sensitivity | OA Table  | OB Table  |
| ------ | --------- | ---------------- | ----------- | --------- | --------- |
| 0000   | NT        | 0 / 0            | None        | 0         | 0         |
| 0001   | AND       | 2 / 2            | IA and IB   | 1000      | 1000      |
| 0010   | CAP       | 0 / 2            | Any         | None      | None      |
| 0011   | OT        | 1 / 2            | IA or IB    | 10        | 10        |
| 0100   | SUB       | 2 / 2            | IA and IB   | 0110      | 0010      |
| 0101   | Undefined | Undefined        | Undefined   | Undefined | Undefined |
| 0110   | XOR       | 2 / 2            | IA and IB   | 0110      | 0110      |
| 0111   | OR        | 2 / 2            | IA and IB   | 1110      | 1110      |
| 1000   | NOR       | 2 / 2            | IA and IB   | 0001      | 0001      |
| 1001   | NXOR      | 2 / 2            | IA and IB   | 1001      | 1001      |
| 1010   | PG        | 2 / 2            | IA and IB   | 1X0X      | 1X0X      |
| 1011   | ADD       | 2 / 2            | IA and IB   | 0110      | 1000      |
| 1100   | NOT       | 1 / 2            | IA or IB    | 01        | 01        |
| 1101   | Undefined | Undefined        | Undefined   | Undefined | Undefined |
| 1110   | NAND      | 2 / 2            | IA and IB   | 0111      | 0111      |
| 1111   | TT        | 0 / 0            | None        | 1         | 1         |

A final note. Great care must be taken to ensure that, for every input to
an OT or a NOT, one and _only_ one of its input edges are to be activated.
The PG gate is perhaps the most powerful of the dual-sensitive ops, as
it can implement all of the others, yet none of the others can implement it.
However, it is the most difficult to check whether it causes undefined
behavior or not. For all other logic ops, it is easy to check if they
are improperly driving a single-sensitivity op (OT and NOT) since
they can have at most one edge linking to it. Such static analysis
is much harder to do with PGs, since their correct functioning cannot
be determined by such simple static analysis.

## Differential Signals and DP

Most every data op is _single-ended_, meaning that they really only compute
one value of meaning, and drive both of their outputs with that value.
Two, however, are _differential_, in that the difference between OA and
OT actually carries value - in other words, their outputs can be different,
and aren't always simple copies of each other. Those, of course, are ADD
and SUB, as they represent the standard half-adder and half-subtracter
truth tables, respectively. As such, their OAs are not the same as their OBs.
Such nodes, if they wish to buffer their outputs would have to resort
to using two OT buffers instead of just the one that single-ended
nodes need.

To address this issue, we'll define another op named DP (which stands for
"diff pair") that will allow us to carry such differential signals.
All it will do is copy its IA to its OA and its IB to its OB. It is
shown in the following updated data op table.

| OpCode | Name      | Sensitivity | OA Table  | OB Table  |
| ------ | --------- | ----------- | --------- | --------- |
| 0000   | NT        | None        | 0         | 0         |
| 0001   | AND       | IA and IB   | 1000      | 1000      |
| 0010   | CAP       | Any         | None      | None      |
| 0011   | OT        | IA or IB    | 10        | 10        |
| 0100   | SUB       | IA and IB   | 0110      | 0010      |
| 0101   | PG        | IA and IB   | 1X0X      | 1X0X      |
| 0110   | XOR       | IA and IB   | 0110      | 0110      |
| 0111   | OR        | IA and IB   | 1110      | 1110      |
| 1000   | NOR       | IA and IB   | 0001      | 0001      |
| 1001   | NXOR      | IA and IB   | 1001      | 1001      |
| 1010   | DP        | IA and IB   | 1100      | 1010      |
| 1011   | ADD       | IA and IB   | 0110      | 1000      |
| 1100   | NOT       | IA or IB    | 01        | 01        |
| 1101   | Undefined | Undefined   | Undefined | Undefined |
| 1110   | NAND      | IA and IB   | 0111      | 0111      |
| 1111   | TT        | None        | 1         | 1         |

Now, there is more to DP than just more efficient buffering for
ADD and SUB. It can be used to carry _any_ signal where the
difference between its elements carry meaning, not just for the
ADD and SUB outputs. This allows, for example, for a compact way to jumper
multiple pieces of data more efficiently, needing only one control
op for to handle effectively two jumper of single-ended data
by first buffering them into a DP. It also allows for more efficient
local routing - by first compressing data into a bus of DPs, more logic
nodes can be reachable by the rather limited range of +/- 8 longitudinal
offsets.

## The Localizer Modifier

Let's return to the notion of a control modifier. What follows is a couple of
charts briefly summarizing the jumper modifier:

| OPCODE | Link Type | Capabilities                                               | Bits per link |
| ------ | --------- | ---------------------------------------------------------- | ------------- |
| 100    | OALAT     | Forwards only OA latitudinally only                        | 11            |
| 101    | OALON     | Forwards only OA longitudinally only                       | 11            |
| 010    | OBLAT     | Forwards only OB latitudinally only                        | 11            |
| 011    | OBLON     | Forwards only OB longitudinally only                       | 11            |
| 110    | ABLAT     | Forwards both OA and OB to same target latitudinally only  | 11            |
| 111    | ABLON     | Forwards both OA and OB to same target longitudinally only | 11            |

| CTL    | OPCODE  | OFFSET   |
| ------ | ------- | -------- |
| `2b01` | `[2:4]` | `[5:15]` |

Let's also recall that all nodes define their relationship to each
other by means of _offset_. This is helpful for determining their
relative position to one another, but does not let us determine their
absolute positions. To illustrate this further, let's return to our
mental model of a DAG as a two dimensional grid where cells are occupied
by data ops, and each data op (ignoring those that are jumped) can
at most be connected to another data op which directly succeeds it
latitudinally (i.e. must be in the next layer) and cannot be
more than an absolute distance of 8 cells away latitudinally (i.e. must
be within 8 cells up or down of the current cell's location).

The issue with this model is that one can easily imagine graphs whose
very structure would prohibit us from placing a data op in every cell
no matter how hard we try. However, in order to reconcile this mental
model with the other model of the data ops as occupying contiguous
pieces of memory, there must be some mechanism which allows one to
specify on the sparse 2D graph the spacing between two successive nodes
in the packed array. This mechanism is the localizer modifier.

Since for any two subsequent nodes the absolute latitudinal distance between
them can either be 0 or 1, this requires only a single bit of information.
The rest of the data can be devoted to absolute longitudinal offsets
in order to encode how far separated a data node is from its preceding
neighbor on the same layer.

We will define, also, that if the latitudinal distance between two nodes
is 1, then the longitudinal offset no longer describes the distance from
the preceding node to the new node, but rather the distance from
the origin of that layer to whatever cell index the data op effectively
resides in. Additionally, this data op _must_ be the very first
node in that layer, starting from cell index 0.

Luckily, there is enough space in what has already been defined as the
jumper modifier to hold this localizer modifier. The full spec for
this is shown below:

| OPCODE | Link Type | Capabilities                                       |
| ------ | --------- | -------------------------------------------------- |
| 000    | LOCLON    | Localizer that offsets long locale                 |
| 001    | LOCLATLON | Localizer that offsets lat and long locale         |
| 010    | OALAT     | Jumper for only OA lat only                        |
| 011    | OALON     | Jumper for only OA long only                       |
| 100    | OBLAT     | Jumper for only OB lat only                        |
| 101    | OBLON     | Jumper for only OB long only                       |
| 110    | ABLAT     | Jumper for both OA and OB to same target lat only  |
| 111    | ABLON     | Jumper for both OA and OB to same target long only |

| CTL    | OPCODE  | OFFSET   |
| ------ | ------- | -------- |
| `2b01` | `[2:4]` | `[5:15]` |

This table now completely describes the "offset" op (much like the other table
of ops corresponding to CTL = `2b00` defines the "data" op).

Also, as a note, that these offset ops only are a subset of the broader
class of control ops, and for any one data op, we've already defined
that there can at most be only one control op which serves to modify
(a.k.a. extend) a data op.

## Input, Output, and FS

Offset ops and data ops are enough to allow us to process the internals
of a DAG. When we think of some generic DAG, we think of it having
a set of input nodes, a set of output nodes, and various intermediary
nodes which fully connect the inputs to the outputs. This is a special
case, however, of all possible DAGs. For example, a DAG can have several
independent trees, where each tree is disjoint from the others
(such a DAG is called a "forest"). Or, a DAG can be constructed such that
any of its inputs eventually propagates to all of its outputs, such that
any one input can trigger an output.

These structures present problems, because we must know whether
a DAG is _done_ being evaluated or not so that we can inform
other processes which drive this DAG whether it is ready to accept
input or not. To bring this down to a less abstract level, let's
consider a basic AND gate. Usually, we think of an AND gate atomically.
That is, it computes its input instantaneously, such that once there is
a change in inputs, there immediately exists the updated value
at its output. However, if we were to say that there exists some form
of unknown _delay_ associated with the computation of an AND gate,
then we would somehow have to inform those who drive this AND gate
when it is ready to accept more input.

The reason why we haven't had to worry about this _propagation delay_
thus far is because we have constrained our attention to a form of
_linear_ logic where we start at some already existing input nodes and
compute, linearly, all downstream descendants, because we assumed that,
since we _could_ evaluate the DAG (i.e. all edges started in the
"unevaluated" state), then we _would_ evaluate it by first evaluating
all the input nodes, then all the intermediary nodes (which have
nicely been sorted in topological order already, so it amounts to a
simple iteration loop), and then evaluate all the output nodes.
Once this was done, we simply assumed that the processor will know
through some sort of address lookup where the result of this computation
is supposed to go, and so will push the input nodes of that _downstream_
DAG. Once that is done, then it will now be able to evaluate _that_
DAG, and update _its_ descendant's inputs, and so the cycle continues.

However, an issue arises. If there exists some input to a DAG that
does not cause all output nodes to update, how will the processor
know that the DAG has completed evaluating? There could be some upstream
DAG wishing to drive our input, but not all of our outputs have been
driven, nor will they ever be driven, given the very nature and structure
of the way we've defined our DAG. If we'd defined our "definition of done"
to simply be when all output nodes have been driven, then we would
deadlock, which is a form of _structural_ hazard.

We _could_ specify that every combination of inputs into a DAG _must_
cause all outputs to be driven. However, this would severely limit
the performance of our system of interconnected DAGs, since for
_any_ change of input into the system, _all_ DAGs must be evaluated
until the system can accept another change of inputs. This is a non-starter.
We need to be able to _conditionally_ drive downstream DAGs, so that
only those DAGs necessary for processing the new input are triggered.

At this point, it may be useful to think of every DAG as a _module_ with
a set of binary inputs and a set of binary outputs. Each of these
modules can be connected such that a graph of modules is formed, connecting
various modules together. Much like how we interpreted each data op
as a node connected to other data op nodes in a DAG, so too we can
think of the _system_ as a graph of modules, whose inputs and outputs
are also connected via edges to each other.

And I say _graph_ instead of _DAG_ with great intention - we shall allow
_feedback_ at the module level (which is _disallowed_ at the DAG level).
We do not now have the luxury of simply assuming that a module is
ready to be evaluated like each data op was at the DAG level. We must
specify a "definition of done" in order to indicate to any upstream
modules that we are ready to take in more input, since we have not
only finished evaluating the model, but have also finished _driving_
all downstream modules. With this we have definitively moved from
the realm of _linear_ logic to _non-linear_ logic, and so different
constraints must be met.

What we _will_ specify, however, is that any output at the module
level must be connected to some module's input. This remains the same
at both the DAG level and at the module level, indicating that at both levels
there exist no dangling edges. This specification ensures that when there
exists some edge, the processor is confident simply by its presence, that
if it _can_ be evaluated, then it will be _useful_ to evaluate it, since it
_cannot_ be left dangling.

What this also means is that a single output _cannot_ drive more than one
downstream module. If we want to drive two modules with the same data,
then at the DAG level we must buffer that data (perhaps with some
OT ops) to two separate sets of nodes, specify both sets of nodes
as output nodes, and at the module level dictate that one set of outputs goes
to one downstream module, and that the other set of outputs goes to the other
module. In this way, any number of modules can be driven by one module
while maintaining the singly-terminating edge rule already established.

This is all fine, but we still need to define our "definition of done"
for a DAG, such that at the module level, we can conditionally activate
output edges so that only the modules who _ought_ to process data
_will_ process data. At the DAG level, we already have the mechanism
for this - the PG op. The PG op can conditionally activate or leave
unevaluated an OT node which, when specified as an output, would at the
module level also leave an inter-module edge un-evaluated. At the DAG
level, each edge could be in one of four states: un-evaluated,
driven high, driven low, or high-impedance (a.k.a. floating). At the
module level, however, since we are now dealing with non-linear logic,
we will specify that any edge can only be in one of three states:
un-evaluated, driven high, or driven low. This will simplify evaluation
since we'll know that if some edge _can_ be evaluated, then we
_should_ evaluate it, since it will definitely have some impact downstream
since it must, at that point, be driven high or low.

Also, I'd like to clarify what an inter-module edge actually connects at
the DAG level. An inter-module edge connects a module's output node to another
module's input node. That means that the upstream node's OA and OB are
routed directly to the downstream node's IA and IB. Care must be taken
to connect single-ended outputs to single-ended inputs, and
differential outputs to differential inputs, though there may be circumstances
where that is not appropriate, like when the IO interface intentionally
converts single-ended signals to differential signals or vice-versa.

Additionally, at the DAG level, we run into a problem _actually_ using
DP or OT, or any other "outputting" data op (a.k.a. all ops so far
except for CAP) for the purpose of assigning them to be module level outputs.
The reason being that they all implicitly assume that there exist
link targets (specified by OA and OB) that need updating _within_ the
DAG itself, though the only entity receiving them are other modules.
This causes issues since then an output node would have to CAP
an "outputting" node's outputs, implying that there can exist further
data ops to process beyond the last output node, which is not ideal.

Thus, we define another sort of data op called FS ("full stop") that
is sensitive to IA and IB (as opposed to CAP which is sensitive to anything)
but that has no output capability (similar to CAP). The now complete
data op chart looks like this. Also, since CAP has evolved to be more
of a no-op than anything else, I'll change its name to NOP.

| OpCode | Name | Sensitivity | OA Table | OB Table |
| ------ | ---- | ----------- | -------- | -------- |
| 0000   | NT   | None        | 0        | 0        |
| 0001   | AND  | IA and IB   | 1000     | 1000     |
| 0010   | NOP  | Any         | None     | None     |
| 0011   | OT   | IA or IB    | 10       | 10       |
| 0100   | SUB  | IA and IB   | 0110     | 0010     |
| 0101   | PG   | IA and IB   | 1X0X     | 1X0X     |
| 0110   | XOR  | IA and IB   | 0110     | 0110     |
| 0111   | OR   | IA and IB   | 1110     | 1110     |
| 1000   | NOR  | IA and IB   | 0001     | 0001     |
| 1001   | NXOR | IA and IB   | 1001     | 1001     |
| 1010   | DP   | IA and IB   | 1100     | 1010     |
| 1011   | ADD  | IA and IB   | 0110     | 1000     |
| 1100   | NOT  | IA or IB    | 01       | 01       |
| 1101   | FS   | IA and IB   | None     | None     |
| 1110   | NAND | IA and IB   | 0111     | 0111     |
| 1111   | TT   | None        | 1        | 1        |

Note that when assigning an FS as an output, the processor
will take its inputs and forward them to other modules, similar
to how DP simply takes its inputs and forwards them as is to other
nodes. In a lot of cases, I expect modules to implement their input
nodes as DPs, since any node which has meaningful input sensitivity
would function well, and DP can carry two bits of information as opposed
to just one for OT.

How can an output node reference a specific input node of another module?
The first thing we can do is label each input node as such so
the processor can recognize it as an input node. We'll specify
another type of modifier control op for IO to help facilitate this. This
is defined for a CTL pattern of `2b10`. It comes in two flavors,
Input and Output, and of the 16 available bits, 2 or devoted to CTL
and the rest to specifying INDEX.

For a module input, the Input flavor of the IO modifier specifies its ID,
like a pin number on an IC. For Output, it needs to define both
the "relative module offset" and also the input index of the target node.
The Output modifier specifies the target input node's index within
the graph that it is embedded in. 14 bits implies that there is a maximum
of 2 ^ 14 inputs this protocol supports on any given DAG. Below is
what the internals of an FS op looks like.

| CTL    | OPCODE   | IA    | IB    | OFFSET   |
| ------ | -------- | ----- | ----- | -------- |
| `2b00` | `4b1101` | `[6]` | `[7]` | `[8:15]` |

We have 8 remaining bits to specify the relative module offset which
is given by the remaining 8 least significant bits of the FS data op.
Notice that this is a permanent interpretation of FS - it can only
be an output node in a DAG. It is also the only node which _requires_
a CTL op to precede it. Similarly, any other type of data op
which is modified by an IO modifier is implicitly assumed to be an input
node. This is what allows us to devote 14 bits of a 16 bit instruction
towards defining an input node's index - the type of op it modifies
is what defines whether the modifier is the input or output flavor.

The OFFSET field in the FS op specifies a relative offset between
its containing module and its neighboring modules in memory. We can
conceptually imagine a processor, once finished loading the process
in its entirety, assigns an index to each module according to its place
in memory. It can do this since input nodes are specified to always
be at the start of the graph (since the constitute the first layer) and
output nodes are specified to always be at the end of the graph (since
they invariably constitute the last layer). A processor can thus
efficiently search through memory, assigning indices to memory locations
as it comes across output nodes abutting input nodes. Or the processor
can enforce alignment such that it degenerates into a simple index reference.
However the method, it is assumed that the processor knows the indices
of the various modules and where they reside in memory.

The OFFSET field specifies the relative module index offset from the output
node's containing module. An OFFSET of 0 means that the output node feeds
an input node of its own module. OFFSET is signed, meaning it can feed
modules that are located before or after itself in memory.

This two additions, the FS op and the IO modifier allows for inter-module
edges. However, we still need to define the "definition of done" for
a module. We'll proceed to that in the next section.

## Definition of Done

When we think of a module, we need to transition from thinking about
individual edges, like at the DAG level, to thinking more about buses. A
bus is merely a set of edges moving in the same direction. Though a
module can have any (up to 2^14) amount of inputs and outputs,
they can conceptually all be lumped into two buses - the module's input
bus and its output bus. The bus may be split and partitioned once it
needs to be routed to various other modules, but right at the boundary
of a module, its inputs and outputs can be thought of as monolithic buses.

A naive implementation of a module's definition of done is to
have it wait for the input bus as a whole to become ready
before allowing the module to begin evaluating its inputs.
Similarly, we would say that once all output nodes of a module have
been evaluated, then the module is ready to take new
inputs.

However, this implementation implies that on every evaluation of a module's
input, _all_ of its downstream descendants would be updated - a module
can't conditionally control who gets what message. This is not very flexible,
as it would imply that on any input to the process, all modules would need
to be evaluated in order to produce any sort of output, which is quite
inefficient if only a subset of those modules are needed in order to
process any particular input.

Another implementation of a module's definition of done is to say that
for any input in its input bus that has been driven, the module would
progress as far as possible in its DAG until, after a sufficient
quantity of inputs, an output node is evaluated, upon which the DAG is
considered done.

The issue with this is that if a module needs to update more than
one output node (which can correspond to only a maximum of two bits),
then it opens itself up to structural hazards, and it would be a lot
of overhead to keep on switching between various nodes, making tiny
incremental progress as each of their inputs come in piecemeal as
input-output dependencies scale in complexity.

As such, a combination of the two approaches will suffice - the
module may be evaluated only when all of its inputs are driven,
and it is said to have been evaluated when no further progress can
be made in its DAG. It is in the hands of the developer to ensure
that a DAG reaches its termination conditions properly under all
relevant input combinations.

## Input Ports

Much how through PGs a module can drive various downstream modules,
we'd like various modules to be able to drive a single module.
Right now it is possible to reference a module's input as a whole,
but we want an upstream module to be able to specify a subset
of a module's input bus to drive, allowing for other modules to
fill the gaps.

As such, any module's input bus can be divided into subsets
called "ports." Each and every output node of a module must
specify the port number that it connects to of its
downstream module. The various input ports of a module
need not be the same width as one another, and
as such, they need to be labelled as well.

We will also aim to consolidate all modifiers thus far mentioned
(Input, Output, Jumper, Localizer) into a single
op. Additionally, we will remove IA and IB from
the data ops, since the goal of each op
is _description_ as opposed to acting as _targets_. FS shall
remain as the only data op which can act as an output node,
as its LA and LB fields are combined into one OFFSET field.

## Summary

### Normal Data Op Format

| CTL    | OPCODE  | LA       | LB        |
| ------ | ------- | -------- | --------- |
| `2b00` | `[2:5]` | `[6:10]` | `[11:15]` |

### FS Data Op Format

| CTL    | OPCODE  | OFFSET   |
| ------ | ------- | -------- |
| `2b00` | `[2:5]` | `[6:15]` |

### Data Ops

| OpCode | Name | Sensitivity | OA Table | OB Table |
| ------ | ---- | ----------- | -------- | -------- |
| 0000   | NT   | None        | 0        | 0        |
| 0001   | AND  | IA and IB   | 1000     | 1000     |
| 0010   | NOP  | Any         | None     | None     |
| 0011   | OT   | IA or IB    | 10       | 10       |
| 0100   | ADD  | IA and IB   | 0110     | 1000     |
| 0101   | IN   | IA          | 10       | 10       |
| 0110   | XOR  | IA and IB   | 0110     | 0110     |
| 0111   | OR   | IA and IB   | 1110     | 1110     |
| 1000   | NOR  | IA and IB   | 0001     | 0001     |
| 1001   | NXOR | IA and IB   | 1001     | 1001     |
| 1010   | OUT  | IA and IB   | 1X0X     | 1X0X     |
| 1011   | SUB  | IA and IB   | 0110     | 0010     |
| 1100   | NOT  | IA or IB    | 01       | 01       |
| 1101   | PG   | IA and IB   | 1X0X     | 1X0X     |
| 1110   | NAND | IA and IB   | 0111     | 0111     |
| 1111   | TT   | None        | 1        | 1        |

### Modifier Ops

| OPCODE | Type      | Capabilities                                       |
| ------ | --------- | -------------------------------------------------- |
| 0010   | LOCLON    | Localizer that offsets long locale                 |
| 0011   | LOCLATLON | Localizer that offsets lat and long locale         |
| 0100   | JUMPLON   | Jumper for both OA and OB to same target long only |
| 0101   | JUMPLAT   | Jumper for both OA and OB to same target lat only  |
| 1000   | IN        | Input modifier (specifies port number)             |
| 1001   | OUT       | Output modifier (specifies downstream port number) |

### Modifier Op Format

| CTL    | OPCODE  | MOD      |
| ------ | ------- | -------- |
| `2b01` | `[2:5]` | `[6:15]` |
