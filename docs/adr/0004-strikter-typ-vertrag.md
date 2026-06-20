# Strikter Typ-Vertrag: `TypeError` bei typ-falscher Eingabe

Die Prädikate des Bindings geben für **korrekt typisierte** Eingaben immer ein
`bool` zurück und werfen **keinen Validierungsfehler**. Eine **typ-falsche** Eingabe
(z. B. `None` oder ein Nicht-`str`, wo ein `str` erwartet wird; ein
Nicht-Numerisches bei `validate_range`) ist ein **Programmierfehler** und löst einen
`TypeError` aus (PyO3-Default) — kein stilles `False`. „Ungültig" (inhaltlich
falsch, aber richtig typisiert) und „falscher Typ" sind damit klar getrennt.

## Considered Options

- **Lenient**: typ-falsche Eingabe → `False`. Verworfen: verbirgt echte
  Programmierfehler hinter einem stillen `False`, erzwingt `Optional`/`Any`-
  Signaturen und widerspricht „errors should never pass silently".

## Consequences

- `validate_email(None)` → `TypeError`; `validate_email("kaputt")` → `False`.
- Signaturen bleiben eng typisiert (`str`, Zahl); keine `Optional`-Aufweichung.
- Präzisiert die Formulierung „reine Prädikate (wirft nie)" aus ADR 0002.
