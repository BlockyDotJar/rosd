# Rosd

![](https://github.com/michidk/rost/blob/hauptzweig/logo.jpg)

Aren't you _miad_ from writing Rust programs in English? Do you like saying
"so a schmare" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Bavarian touch to your
programs?

**rosd** (Bavarian for _Rust_) is here to save your day, as it allows you to
write Rust programs in Bavarian, using Bavarian keywords, Bavarian function names and
Bavarian idioms.

Here's an example of what can be achieved with Rosd:

### trait and impl (aka Konvention und Umsezung)

```rust
rosd::rosd! {
    nutz std::grusch::Biachl ois biachl;

    eignschafd Schlisslwert {
        funktion schreib(&seibsd, schlsl: Zeichnkedn, wert: Zeichnkedn);
        funktion les(&seibsd, schlsl: Zeichnkedn) -> Eagebnis<Möglichkeit<&Zeichnkedn>, Zeichnkedn>;
    }

    stadisch ändabar BIACHL: Möglichkeit<biachl<Zeichnkedn, Zeichnkedn>> = Nixend;

    struktur Konkret;

    umstz Schlisslwert fia Konkret {
        funktion schreib(&seibsd, schlsl: Zeichnkedn, wert: Zeichnkedn) {
            loss biachl = gfealeh {
                BIACHL.hoi_oda_füg_ei_mid(Standard::standard)
            };
            biachl.eifügn(schlsl, wert);
        }

        funktion les(&seibsd, schlsl: Zeichnkedn) -> Eagebnis<Möglichkeit<&Zeichnkedn>, Zeichnkedn> {
            wenn loss Ebsend(biachl) = gfealeh { BIACHL.ois_ref() } {
                Guad(biachl.hoi(&schlsl))
            } sunst {
                Schlechd("Hoi des biachl.".eina())
            }
        }
    }
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. Des wars oida.

## Seibsd is da Coder

First of all, _mease dia_ for considering participating to this joke, the
Bavarian government will thank you later! Feel free to throw in a few identifiers
here and there, and open a pull-request against the `hauptdrum` (Bavarian for
`main`) branch.

## But why would you do *des*

Ja warum an ned oida?

## S' rechtliche Zeigs (aka LIZENZ)

[WTFPL](http://www.wtfpl.net/).
The images do not fall under this license, see below.

**Image attributions:**

* "Brezel und Filzhut zum Oktoberfest" by Tim Reckmann | a59.de is licensed under CC BY 2.0
* "Lederhose" is licensed under CC BY-NC-SA 4.0
