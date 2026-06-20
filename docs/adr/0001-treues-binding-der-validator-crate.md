# Treues Binding der `validator`-Crate (mit dokumentierten Ausnahmen)

rusty-validator ist ein PyO3-**Binding** der Rust-Crate
[`validator`](https://github.com/Keats/validator), kein eigenständiger Validierer.
Wir reichen die Validierungssemantik der Crate standardmäßig **unverändert** nach
Python durch: Was die Crate als gültig betrachtet, ist hier gültig. Die Crate ist
die Vorgabe — wir definieren keine eigenen Regeln. **Ausnahme:** Wir behalten uns
vor, einzelne Verhaltensweisen bewusst und ausdrücklich dokumentiert zu
überschreiben oder zu normalisieren, wenn das Crate-Verhalten für unsere Nutzer
klar unerwünscht ist.

## Considered Options

- **Eigene Validierungsregeln definieren** (z. B. URL auf `http`/`https`
  einschränken, E-Mails normalisieren). Verworfen: Das untergräbt den Sinn eines
  Bindings, dupliziert Logik und erzeugt Wartungslast gegenüber der Crate.

## Consequences

- Überraschende Ergebnisse sind **beabsichtigt geerbt**, kein Bug: z. B.
  `validate_url("foo://bar") == True` (jedes Schema), IPv6 zählt als IP, keinerlei
  DNS-/Netzwerkprüfung. Solche Fälle nicht „reparieren".
- Jede Abweichung von der Crate ist eine **Ausnahme** und muss explizit
  dokumentiert werden (in `CONTEXT.md` und/oder einem Folge-ADR), sonst gilt sie
  als Bug.
- Das Verhalten ist an die verwendete Crate-Version (`validator 0.18.x`) gekoppelt;
  ein Crate-Upgrade kann Grenzfälle verschieben.
