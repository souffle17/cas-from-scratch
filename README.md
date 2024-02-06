# rough-grapher

A CLI grapher in rust.

This program will print a graph in the command line, with user provided arguments for the range and scale of the axis, and two halves of a user provided equation in prefix notation.

- s = sin
- c = cos
- t = tan
- a = abs
- q = sqrt
- l = log
- e = exp
- use '_' as the negative sign for equations

## Example
[https://en.wikipedia.org/wiki/Rose_(mathematics)](https://en.wikipedia.org/wiki/Rose_(mathematics))
- Input: -30 30 -30 30 "a e + e x 2 e y 2 2" "* 10 a * _2.5 - e x 3 * 3 * x e y 2" 100 0.5 1
- Output:
```
                                                            |
                                                            |
                                                            |
                                                            |
                                                            |
                                                            |
                                                            |
                                                            |
                                  ••••••••                  |                 ••••••••
                                 •••       ••               |              ••••       •
                                ••            •             |            •••
                                •               •           |          •••              •
                                •                           |         ••                •
                                •                  •        |       •••                 •
                                                            |      ••                  ••
                                 •                          |     ••                   •
                                                            |    ••                   ••
                                                       •    |   ••                   ••
                                                        •   |   ••                  ••
                                                            |  ••                  ••
                                      •                  •  | ••                 •••
                                                          • | ••                ••
                                         •                • | •               •••
                                           •                |••             •••
                                                           •|•             ••
                        •••••••••             ••           •|•          ••••           •••••••••
                •••••••••         ••••••••••     •          ••        •••    •••••••••••         •••••••
             ••••                           •••••• •        •       ••••••••••                           ••
           •••                                     ••••     •    ••••••                                     •
          ••                                           •••  • •••••
----------•-----------------------------------------------•••••-----------------------------------------------•----------
                                                      ••••• •  •••                                           ••
            •                                     ••••••    •     ••••                                     •••
              ••                             ••••••••       •        • •••••                            ••••
                 •••••••         •••••••• •••   •••        ••          •     ••••••••••         •••••••••
                         •••••••••           ••••          •|•           ••             •••••••••
                                            ••             •|•
                                          •••             ••|                •
                                        •••               • | •                •
                                       ••                •• | •
                                     •••                 •• |  •                  •
                                    ••                  ••  |
                                   ••                   •   |   •
                                  ••                   ••   |    •
                                 ••                   ••    |
                                 •                   ••     |                          •
                                ••                  ••      |
                                •                 •••       |        •                  •
                                •                ••         |                           •
                                •              •••          |           •               •
                                             •••            |             •            ••
                                  •       ••••              |               ••       •••
                                   ••••••••                 |                  ••••••••
                                                            |
                                                            |
                                                            |
                                                            |
                                                            |
                                                            |
                                                            |
                                                            |
```