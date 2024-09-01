# bee

[![Build status](https://img.shields.io/github/actions/workflow/status/tveness/bee/rust.yml?style=for-the-badge)](https://github.com/tveness/bee/actions/workflows/rust.yml)
[![License](https://img.shields.io/github/license/tveness/bee?style=for-the-badge)](https://opensource.org/license/agpl-v3)
![Release](https://img.shields.io/github/v/tag/tveness/bee?label=latest%20release&style=for-the-badge)

This is a simple CLI tool to solve the NYT spelling bee.

## Installation
If you don't have rust, then you can install the toolchain by following the
[official instructions](https://www.rust-lang.org/tools/install), after which
life should be as simple as:
```bash
cargo install --git https://github.com/tveness/bee
```

A number of [binaries](https://github.com/tveness/bee/releases) are also available.


## Usage
Pass one argument, which are the letters in the bee and the central letter is first.

```bash
~ bee maoitrv
Central letter: 'm'
Other letters: ['a', 'o', 'r', 't', 'i', 'v']
 4: [ amia amir ammo atma atom imam maar maim mair mama mara marm mart matt miri miro mirv mitt moai moat moit momi mooi moor moot mora mort moti mott omit omov rami rima roam roma room tomo toom tram trim ]
 5: [ amort amrit armor aroma imari immit mamma maria maror marri matai moira momma morat moria morra morro motor motto tomia vomit vroom ]
 6: [ amrita maomao marari marmot marram matata miriti mirror moirai moorva mortar motmot tamara tamari tammar tarama tatami tomato tomtit varoom vomito ]
 7: [ mammati maormor mormaor tamarao tritoma ]
 8: [ imitator matamata miromiro timariot trimotor ]
 9: [ moratoria motivator vomitoria ]
10: [ amritattva ]
```

Alternatively, if you would only like some simple statistics about the answers:
```bash
Central letter: 'm'
Other letters: ['a', 'o', 'i', 't', 'r', 'v']

WORDS: 97, PANGRAMS: 2

    4  5  6  7  8  9  10 Σ
A:  5  4  1  -  -  -  1  11
I:  1  2  -  -  1  -  -  4
M:  23 13 11 3  2  2  -  54
O:  2  -  -  -  -  -  -  2
R:  5  -  -  -  -  -  -  5
T:  4  1  7  2  2  -  -  16
V:  -  2  2  -  -  1  -  5
Σ:  40 22 21 5  5  3  1  97

AM: 7  AR: 2  AT: 2
IM: 4
MA: 21 MI: 7  MO: 26
OM: 2
RA: 1  RI: 1  RO: 3
TA: 6  TI: 1  TO: 5  TR: 4
VA: 1  VO: 3  VR: 1
```
