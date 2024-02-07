# simple-cas

A rudimentary CLI computer algebra system made from scratch in Rust, with no external crates.

This uses an entirely custom expression parser, so it will not be as practical as other packages.

#### Capabilities
- Parsing expressions
- Displaying expression trees
- Implicit graphing of equations
- Simplifying expressions (todo)

#### Expressions
Expressions need to be in prefix notation seperated by spaces.

- s = sine
- c = cosine
- t = tangent
- a = absolute value
- q = square root
- l = log
- e = exponent
- x and y are the variables

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
