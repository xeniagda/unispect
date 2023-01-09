# unispect

Like `uniname` but worse.

Made because nix doesn't have uniname and I couldn't find any way to install it and also it kinda sucked so like

## Example

```
$ echo "Δ, Й, ק ,م, ๗, あ, 叶, 葉, and 말" | ./unispect

U+0394   <ce94>     Lu [Greek and Coptic]        | GREEK CAPITAL LETTER DELTA
U+002C   <2c>       Po [Basic Latin]             | COMMA
U+0020   <20>       Zs [Basic Latin]             | SPACE
U+0419   <d099>     Lu [Cyrillic]                | CYRILLIC CAPITAL LETTER SHORT I
U+002C   <2c>       Po [Basic Latin]             | COMMA
U+0020   <20>       Zs [Basic Latin]             | SPACE
U+05E7   <d7a7>     Lo [Hebrew]                  | HEBREW LETTER QOF
U+0020   <20>       Zs [Basic Latin]             | SPACE
U+002C   <2c>       Po [Basic Latin]             | COMMA
U+0645   <d985>     Lo [Arabic]                  | ARABIC LETTER MEEM
U+002C   <2c>       Po [Basic Latin]             | COMMA
U+0020   <20>       Zs [Basic Latin]             | SPACE
U+0E57   <e0b997>   Nd [Thai]                    | THAI DIGIT SEVEN
U+002C   <2c>       Po [Basic Latin]             | COMMA
U+0020   <20>       Zs [Basic Latin]             | SPACE
U+3042   <e38182>   Lo [Hiragana]                | HIRAGANA LETTER A
U+002C   <2c>       Po [Basic Latin]             | COMMA
U+0020   <20>       Zs [Basic Latin]             | SPACE
U+53F6   <e58fb6>   Lo [CJK Unified Ideographs]  | CJK UNIFIED IDEOGRAPH-53F6
U+002C   <2c>       Po [Basic Latin]             | COMMA
U+0020   <20>       Zs [Basic Latin]             | SPACE
U+8449   <e89189>   Lo [CJK Unified Ideographs]  | CJK UNIFIED IDEOGRAPH-8449
U+002C   <2c>       Po [Basic Latin]             | COMMA
U+0020   <20>       Zs [Basic Latin]             | SPACE
U+0061   <61>       Ll [Basic Latin]             | LATIN SMALL LETTER A
U+006E   <6e>       Ll [Basic Latin]             | LATIN SMALL LETTER N
U+0064   <64>       Ll [Basic Latin]             | LATIN SMALL LETTER D
U+0020   <20>       Zs [Basic Latin]             | SPACE
U+B9D0   <eba790>   Lo [Hangul Syllables]        | HANGUL SYLLABLE MAL
U+002E   <2e>       Po [Basic Latin]             | FULL STOP
U+0020   <20>       Zs [Basic Latin]             | SPACE
```

## Usage

```
./unispect --help

Usage: unispect [OPTIONS]

Options:
  -f, --format <FORMAT>
          What attributes to include, in order. Key:
          p: (Code)point (eg. U+e621)
          e: UTF-8 Encoding (eg. <e621>)
          a: Age/version added (eg. 1.1.0)
          c: Category (eg. Ll)
          b: Block (eg. [Basic Latin])
          C: Case
              i: ignorable case
              c: cased
              f: changes when casefolded
              m: changes when casemapped
              l: changes when lowercased
              u: changes when uppercased
              t: changes when titlecased
              L: lowercase
              U: uppercase

          [default: pecb]

  -q, --no-name
          Do not display the name of the character

  -t, --tight
          Don't pad each attribute to align them

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```

## TODO:

* Handle invalid UTF-8?
* Print the characters themselves? (Spacing will be hard to get right, combining characters must be handled, etc.)
