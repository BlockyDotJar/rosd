rosd::rosd! {
    nutz std::grusch::Biachl ois biachl;

    eignschaft Schlisslwert {
        funktion schreib(&seibst, schlsl: Zeichnkettn, wert: Zeichnkettn);
        funktion les(&seibst, schlsl: Zeichnkettn) -> Eagebnis<Möglichkeit<&Zeichnkettn>, Zeichnkettn>;
    }

    statisch änderbar BIACHL: Möglichkeit<biachl<Zeichnkettn, Zeichnkettn>> = Nixend;

    struktur Konkret;

    umstz Schlisslwert fia Konkret {

        funktion schreib(&seibst, schlsl: Zeichnkettn, wert: Zeichnkettn) {
            loss biachl = gferleh {
                BIACHL.hoi_oda_füg_ei_mid(Standard::standard)
            };
            biachl.eifügn(schlsl, wert);
        }

        funktion les(&seibst, schlsl: Zeichnkettn) -> Eagebnis<Möglichkeit<&Zeichnkettn>, Zeichnkettn> {
            wenn loss Ebsend(biachl) = gferleh { BIACHL.ois_ref() } {
                Guad(biachl.hoi(&schlsl))
            } sunst {
                Schlechd("Hoi des biachl.".eina())
            }
        }
    }

    öffndle(kistn) funktion vielleicht(i: u32) -> Möglichkeit<Eagebnis<u32, Zeichnkettn>> {
        wenn i % 2 == 1 {
            wenn i == 42 {
                Ebsend(Schlechd(Zeichnkettn::vo("So a schmare...")))
            } sunst {
                Ebsend(Guad(33))
            }
        } sunst {
            Nixend
        }
    }

    asynchron funktion beispiel() {
    }

    asynchron funktion beispiel2() {
        beispiel().obwoadn;
    }

    funktion eistieg() {
        loss änderbar x = 31;

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

        nutz std::vgl::Ordnung;
        loss _mod7 = vec![0; 100].wieda()
            .nimm(50)
            .zuordnen(|numma| numma %  7)
            .sammen::<Vec<i32>>()
            .zu_wieda()
            .foid(0, |a, numma| entsprich numma.vgl(&a) {
                Ordnung::Mea => a - numma,
                Ordnung::Wenga => a + numma,
                Ordnung::Gleich => a,
            });
    }
}
