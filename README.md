# simple-cas

A rudimentary CLI computer algebra system made from scratch in Rust.

#### Capabilities
- Implicit graphing of equations
- Simplifying expressions (todo)

#### Expressions
Expressions need to be in prefix notation seperated by spaces.

- s = sin
- c = cos
- t = tan
- a = abs
- q = sqrt
- l = log
- e = exp
- use '_' as the negative sign for equations
- x and y are the variables

## Graphing
This program will print an implicit graph in the command line given two user-provided expressions with prefix notation.

### Example
[https://en.wikipedia.org/wiki/Rose_(mathematics)](https://en.wikipedia.org/wiki/Rose_(mathematics))

```
first expression: a e + e x 2 e y 2 2
second expression: * 10 a * _2.5 - e x 3 * 3 * x e y 2
x-axis minimum (default -10): -30
x-axis maximum (default 10): 30
y-axis minimum (default -10): -30
y-axis maximum (default 10): 30
x scale (default 1.0): 0.5
y scale (default 1.0): 1
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
```
