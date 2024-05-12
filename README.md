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

## Hyperlink Modifier

There are times when a link offset is not expressible with only 4 bits.
The link offset included in the data op is designed to take advantage of
locality, but when that is not an option for a particular link, there
should be a mechanism for extending the range of that link. The hyperlink
modifier aims to fill that gap.

Though we previously imagined the DAG to be a two dimensional grid structure,
in memory it will be stored as a contiguous piece of memory, topologically
sorted such that a process can simply cycle through the ops sequentially.
As such, we can place a _modifier_ op before a data op
to indicate that the data op needs to be treated in a different manner
than usual. This allows these modifiers to remain somewhat transparent
to the data processing, and essentially gives each node more capabilities
by simply tagging it with these modifier instructions.

For now, the only modifier instruction that gets "attached" to a data
node itself is the hyperlink modifier. It can come in either a
one-dimensional or two-dimensional flavor (i.e. 1D and 2D). A 1D
hyperlink means that there is a bit which indicates whether the hyperlink
is making either a longitudinal or a lateral link. The rest of the bits
of that 16 word indicates how big the offset is. Lateral is unsigned
with an implicit start of 1, and lateral is signed with an implicit
negative bias (reasoning given above when describing LA and LB).

A 2D hyperlink is similar except you specify both a lateral and a longitudinal
offset. Half the available bits are dedicated to the lateral offset, and
the other half to the longitudinal offset.

In addition, a hyperlink can specify which output it can forward. A 1 output
hyperlink only forwards a specified output to the target. A 2 output
hyperlink forwards both outputs to the target. An independent hyperlink
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
hyperlinks do not give much advantage unless we restrict ourselves
to LAT or LON link types and ignore the OO link types. Given that,
we now only have six options, meaning we also gain an additional bit we can
devote to specifying the link offset. Below is the updated table.

| OPCODE | Link Type | Capabilities                                               | Bits per link |
| ------ | --------- | ---------------------------------------------------------- | ------------- |
| 100    | OALAT     | Forwards only OA latitudinally only                        | 11            |
| 101    | OALON     | Forwards only OA longitudinally only                       | 11            |
| 010    | OBLAT     | Forwards only OB latitudinally only                        | 11            |
| 011    | OBLON     | Forwards only OB longitudinally only                       | 11            |
| 000    | ABLAT     | Forwards both OA and OB to same target latitudinally only  | 11            |
| 111    | ABLON     | Forwards both OA and OB to same target longitudinally only | 11            |

We will also specify that any one data node will have, at most, one
hyperlink modifier preceding it. That might mean that it takes multiple
intermediary hyperlinks in order to reach a target, but it will help
simplify processing logic. A hyperlink modifier is specified with a CTL
sequence of `01`.

| CTL    | OPCODE  | HL       |
| ------ | ------- | -------- |
| `2b01` | `[2:4]` | `[5:15]` |

Now, there is one more additional concern about these hyperlinks. We'd
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

## The Transmission Gate
