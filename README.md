# cas-from-scratch

A rudimentary computer algebra system made from scratch in Rust, using no external crates/libraries.       
This was made for fun/my own learning

#### Capabilities
- Parsing expressions
- Displaying expression trees
- [Implicit graphing of equations](#graphing)
- [Simplifying expressions](#simplification)


![cargo file](img.png)

#### Expressions
This uses an entirely custom expression parser, so it may be more barebones than other CAS implementations

x and y are variables
##### Symbols:
- x + y
- x - y
- x * y
- x / y
- sin x
- cos x
- tan x
- abs x
- sqrt x
- x log y
- x ^ y
- ( x + y ) * x

## Graphing
This program will print an implicit graph in the command line given two user-provided expressions.

### Example
[https://en.wikipedia.org/wiki/Rose_(mathematics)](https://en.wikipedia.org/wiki/Rose_(mathematics))

```
Expressions in memory:
1: ( x ^ 2 + y ^ 2 ) ^ 3
2: -7.5 * ( x ^ 5 - 10x ^ 3 * y ^ 2 + 5x * y ^ 4 )

0. Quit
1. Enter an expression
2. Print expression tree
3. Compute value from expression
4. Graph expressions as equation
5. Simplify an expression
Pick an operation by number: 4
x-axis minimum (default -10):
x-axis maximum (default 10): 
y-axis minimum (default -10): 
y-axis maximum (default 10): 
x scale (default 1.0): 0.15
y scale (default 1.0): 0.3

                                                                   |
                                                                   |
                                                                   |
                                                                   |
                                                                   |
                                                                   |
                                                                   |
                                                                   |
                                                                   |
                                                   ••••            |
                                                  •   •••          |
                                                 ••     ••         |
                                                 •        ••       |
                                                 •         ••      |
                                                  •         •      |
                                                  •          •     |
                                                  •           •    |
                                                   •          ••   |
                                                   ••          •   |                                ••••••••
                                                    •           •  |                            ••••        •
                                                     •          •  |                        ••••           ••
                                                     ••          • |                      •••             ••
                                                      ••         • |                   •••              ••
                                                       ••        • |                 ••               •••
                                                        •         •|              •••               •••
                                                         •        •|            •••              •••
                                                          ••      •|          •••              •••
                                                           ••     •|        •••             •••
                                                            ••     •      •••           ••••
                                                             ••    •     ••          •••
                            •••••••••••••••                   ••   •   ••       •••••
                    ••••••                    ••••••••••        •• •  ••    ••••
                 •••                                     ••••••• ••••• •••••
-----------------•-------------------------------------------------•-•-----------------------------------------------------------------
                 •••                                     ••••••• ••••• •••••
                    ••••••                    ••••••••••        •• •  ••    ••••
                            •••••••••••••••                   ••   •   ••       •••••
                                                             ••    •     ••          •••
                                                            ••     •      •••           ••••
                                                           ••     •|        •••             •••
                                                          ••      •|          •••              •••
                                                         •        •|            •••              •••
                                                        •         •|              •••               •••
                                                       ••        • |                 ••               •••
                                                      ••         • |                   •••              ••
                                                     ••          • |                      •••             ••
                                                     •          •  |                        ••••           ••
                                                    •           •  |                            ••••        •
                                                   ••          •   |                                ••••••••
                                                   •          ••   |
                                                  •           •    |
                                                  •          •     |
                                                  •         •      |
                                                 •         ••      |
                                                 •        ••       |
                                                 ••     ••         |
                                                  •   •••          |
                                                   ••••            |
                                                                   |
                                                                   |
                                                                   |
                                                                   |
                                                                   |
                                                                   |
                                                                   |
                                                                   |
                                                                   |

```

## Simplification
This function is still WIP, and currently attempts to combines all constant values

### Example
```
Expressions in memory: 
1: ( 15 * 2.3 ) * x - ( y + 4 - 3 ) / 3 log 9
2: 

0. Quit
1. Enter an expression
2. Print expression tree
3. Compute value from expression
4. Graph expressions as equation
5. Simplify an expression
Pick an operation by number: 5
Select an expression to simplify: 1
_______________________________________________________________________________________

Expressions in memory:
1:  ( ( 34.5 * x ) - ( ( ( y + 4 ) - 3 ) / 0.5 ) )
2:

0. Quit
1. Enter an expression
2. Print expression tree
3. Compute value from expression
4. Graph expressions as equation
5. Simplify an expression
Pick an operation by number:
```
