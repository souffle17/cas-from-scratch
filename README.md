# rough-grapher

A CLI grapher in rust, using a rudimentary computer algebra system made from scratch.

This program will print a graph in the command line given two user-provided expressions with prefix notation.

- s = sin
- c = cos
- t = tan
- a = abs
- q = sqrt
- l = log
- e = exp
- use '_' as the negative sign for equations
- x and y are the variables

Increase number of sub-iterations if graph is too sparse

## Example
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