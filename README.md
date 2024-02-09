# cas-from-scratch

A rudimentary computer algebra system made from scratch in Rust, using no external crates/libraries.       
This was made for fun/my own learning

#### Capabilities
- Parsing expressions
- Displaying expression trees
- Implicit graphing of equations
- Simplifying expressions (todo)

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
- (x + y) * x

## Graphing
This program will print an implicit graph in the command line given two user-provided expressions.

### Example
[https://en.wikipedia.org/wiki/Rose_(mathematics)](https://en.wikipedia.org/wiki/Rose_(mathematics))

```
Expressions in memory:
1: a e + e x 2 e y 2 2
2: * 10 a * -2.5 - e x 3 * 3 * x e y 2

0. Quit
1. Enter an expression
2. Print expression tree
3. Compute value from expression
4. Graph expressions as equation
5. Simplify an expression
Pick an operation by number: 4
x-axis minimum (default -10): -30
x-axis maximum (default 10): 30
y-axis minimum (default -10): -30
y-axis maximum (default 10): 30
x scale (default 1.0): 0.5
y scale (default 1.0):

                                                            |
                                                            |
                                                            |
                                                            |
                                                            |
                                                            |
                                                            |
                                                            |
                                   •••••••                  |                  •••••••
                                 ••       •••               |               •••       ••
                                ••           •••            |            •••           ••
                                •              •••          |          •••              •
                                •                ••         |         ••                •
                                •                  •        |        •                  •
                                ••                  ••      |      ••                  ••
                                 •                   ••     |     ••                   •
                                 ••                   ••    |    ••                   ••
                                  ••                   •    |    •                   ••
                                   ••                   •   |   •                   ••
                                    ••                  ••  |  ••                  ••
                                      •                  •  |  •                  •
                                       ••                 • | •                 ••
                                        ••                • | •                ••
                                          ••               •|•               ••
                                            ••             •|•             ••
                           ••••               ••           •|•           ••               ••••
                 ••••••            •••••••••    •••         •         •••    •••••••••            ••••••
             ••••                           •••••••••       |       •••••••••                           ••••
           ••                                     ••••••    |    ••••••                                     ••
          ••                                           •••  |  •••                                           ••
----------•------------------------------------------------•+•------------------------------------------------•----------
          ••                                           •••  |  •••                                           ••
           ••                                     ••••••    |    ••••••                                     ••
             ••••                           •••••••••       |       •••••••••                           ••••
                 ••••••            •••••••••    •••         •         •••    •••••••••            ••••••
                           ••••               ••           •|•           ••               ••••
                                            ••             •|•             ••
                                          ••               •|•               ••
                                        ••                • | •                ••
                                       ••                 • | •                 ••
                                      •                  •  |  •                  •
                                    ••                  ••  |  ••                  ••
                                   ••                   •   |   •                   ••
                                  ••                   •    |    •                   ••
                                 ••                   ••    |    ••                   ••
                                 •                   ••     |     ••                   •
                                ••                  ••      |      ••                  ••
                                •                  •        |        •                  •
                                •                ••         |         ••                •
                                •              •••          |          •••              •
                                ••           •••            |            •••           ••
                                 ••       •••               |               •••       ••
                                   •••••••                  |                  •••••••
                                                            |
                                                            |
                                                            |
                                                            |
                                                            |
                                                            |
                                                            |
                                                            |

_______________________________________________________________________________________
```
