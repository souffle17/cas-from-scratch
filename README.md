# rough-grapher

A very rough CLI grapher in rust.

This program will print a graph in the command line, with user provided arguments for the range and scale of the axis, and two halves of a user provided equation in prefix notation.

- s = sin
- c = cps
- t = tan
- a = abs
- q = sqrt
- l = log
- e = exp
- use '_' as the negative sign for equations

## Example
[https://en.wikipedia.org/wiki/Rose_(mathematics)](https://en.wikipedia.org/wiki/Rose_(mathematics))
- Input: -3 3 -3 3 "a e + e x 2 e y 2 2" "a * _2.5 - e x 3 * 3 * x e y 2" 0.05 0.07
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
                                                            |
                                                            |
                                                            |
                                                            |
                                   •                        |                        •
                                                            |
                                                            |
                                                            |
                                •                           |                           •
                                                            |
                                                            |
                                                            |
                                                    •       |       •
                                                            |
                                                            |
                                                            |
                                                            |
                                                            |
                                                        •   |   •
                                                            |
                                                         •  |  •
                                                            |
                                                            |
                                                          • | •
                                                            |
                                                           •|•
                                            •              •|•              •
                                             •             •|•             •
                                               •           •••           •
                                     ••         •          •••          •         ••
                •                          ••     •        •••        •     ••                          •
                                               ••• •••   •••••••   ••• •••
                                                  •••••••••••••••••••••
                                                    •••••••••••••••••
                                                      •••••••••••••
----------•-------------------------------------------•••••••••••••-------------------------------------------•----------
                                                      •••••••••••••
                                                    •••••••••••••••••
                                                  •••••••••••••••••••••
                                               ••• •••   •••••••   ••• •••
                •                          ••     •        •••        •     ••                          •
                                     ••         •          •••          •         ••
                                               •           •••           •
                                             •             •|•             •
                                            •              •|•              •
                                                           •|•
                                                            |
                                                          • | •
                                                            |
                                                            |
                                                         •  |  •
                                                            |
                                                        •   |   •
                                                            |
                                                            |
                                                            |
                                                            |
                                                            |
                                                    •       |       •
                                                            |
                                                            |
                                                            |
                                •                           |                           •
                                                            |
                                                            |
                                                            |
                                   •                        |                        •
                                                            |
                                                            |
                                                            |
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