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

    öffndle(kisdn) funktion vielleicht(i: u32) -> Möglichkeit<Eagebnis<u32, Zeichnkedn>> {
        wenn i % 2 == 1 {
            wenn i == 42 {
                Ebsend(Schlechd(Zeichnkedn::fo("So a schmare...")))
            } sunst {
                Ebsend(Guad(33))
            }
        } sunst {
            Nixend
        }
    }

    asynchron funktion beispui() {
    }

    asynchron funktion beispui2() {
        beispui().obwoadn;
    }

    funktion eistieg() {
        loss ändabar x = 31;

        entsprich x {
            42 => {
                gsoßrad!("Wienerschnizl")
            }
            _ => gsoßrad!("Na gehd doch")
        }

        fia i in 0..10 {
            loss val = schleifn {
                afhean i;
            };

            während nix x < val {
                x += 1;
            }

            x = wenn loss Ebsend(ergebnis) = vielleicht(i) {
                ergebnis.afmocha()
            } sunst {
                12
            };
        }

        nutz std::vgl::Oadnung;

        loss _mod7 = tabein![0; 100].wieda()
            .nimm(50)
            .zuaoadnen(|numma| numma %  7)
            .samen::<Tabein<i32>>()
            .zu_wieda()
            .foid(0, |a, numma| entsprich numma.vgl(&a) {
                Oadnung::Meah => a - numma,
                Oadnung::Wenga => a + numma,
                Oadnung::Gleich => a,
            });
    }
}
