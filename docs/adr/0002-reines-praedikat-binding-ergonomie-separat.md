# rusty-validator bleibt reines Prädikat-Binding; Ergonomie in separatem Paket

rusty-validator exponiert die Validatoren der `validator`-Crate als **reine
Prädikate** (`-> bool`; wirft keinen Validierungsfehler) — inklusive der parametrisierten
Constraint-Validatoren (`length`, `range`, `contains`, `must_match`). Ergonomische
Bausteine — **Guards** (werfen bei ungültig), **Constraint-Objekte** und
**Property-/Feld-Decorators** — gehören bewusst **nicht** in dieses Paket, sondern
in ein separates Ergonomie-Paket obendrauf, das rusty-validator als Abhängigkeit
nutzt. So bleibt das Binding klein und 1:1 zur Crate (siehe ADR 0001), und „was ist
gültig?" (Prädikat) ist hart getrennt von „wie erzwinge/komponiere ich
Validierung?" (Ergonomie).

## Considered Options

- **Alles in einem Paket** (vollwertiges Framework). Verworfen: vermischt zwei
  Verantwortlichkeiten, bläht das Binding zum Framework auf und koppelt die
  Release-Zyklen aneinander.

## Consequences

- rusty-validator hat genau **eine** Ergebnis-Semantik: Prädikat (`bool`). Kein
  throw-Stil bei Ungültigkeit, keine Objekte, keine Decorators in diesem Paket.
- Prädikate werfen keinen *Validierungs*-Fehler, sehr wohl aber `TypeError` bei
  typ-falscher Eingabe (Programmierfehler) — siehe ADR 0004.
- Wer Guards/Decorators/Constraint-Objekte braucht, nutzt das separate
  Ergonomie-Paket. Dessen Name und Versionierung sind noch offen.
- Die parametrisierten Constraint-Validatoren erfordern hier eine API-Form, die
  Parameter trägt (über die bisherige flache `f(str) -> bool`-Form hinaus).
