# Rusty Validator

Ein schmales Python-Binding der Rust-Crate `validator` (über PyO3). Es prüft, ob
ein Wert eine bestimmte Regel erfüllt — die Form einer E-Mail/URL/IP oder eine
Bedingung wie Länge oder Wertebereich — und gibt das Ergebnis als `bool` zurück.
Eigene Ergonomie (Decorators, Guards, Constraint-Objekte) gehört bewusst in ein
separates Paket, nicht hierher.

## Language

**Validieren / Validierung**:
Die Prüfung, ob ein Wert eine **definierte Regel** erfüllt. Das Ergebnis ist ein
**Prädikat**: für korrekt typisierte Eingaben immer ein `bool`, nie ein
Validierungs-*Fehler* — der „wirf bei ungültig"-Stil (Guard) lebt im separaten
Ergonomie-Paket, nicht hier. **Typ-falsche** Eingaben (z. B. `None`) sind ein
Programmierfehler und lösen wie üblich einen `TypeError` aus, kein stilles `False`.
Invariante: rein lokal — es wird nie geprüft, ob etwas real existiert oder
erreichbar ist (kein DNS, kein Netzwerk, keine IO).
*Avoid*: Prüfen auf Existenz, Verifizieren, Zustellbarkeit; Guard/Erzwingen (andere Operation)

**Format-Validator**:
Ein **argumentloser** Validator, der die **Form** einer Zeichenkette prüft:
`email`, `url`, `ip` (perspektivisch auch `credit_card`, `phone`,
`non_control_character`). Signatur stets `f(str) -> bool`.

**Constraint-Validator**:
Ein **parametrisierter** Validator, der eine **Bedingung** an einem Wert prüft:
`length`, `range`, `contains`, `must_match`. Braucht Argumente (z. B. Grenzen) und
arbeitet nicht zwingend auf Zeichenketten (`range` prüft Zahlen).
*Avoid*: Format-Validator (die andere Unterart)

**E-Mail-Adresse**:
Eine **einzelne**, wohlgeformte Adresse `lokalteil@domain` im Sinne von
`validator::ValidateEmail`. Rein syntaktisch — keine Existenz-/MX-Prüfung. **Keine**
Anzeigenamen-Form (`Max <max@x.de>`) und **keine** Liste mehrerer Adressen.
*Avoid*: E-Mail (zu unscharf, wenn die Adresse gemeint ist), Mail-Konto

**URL**:
Eine wohlgeformte **absolute** URL im Sinne von `validator::ValidateUrl` (RFC 3986),
also *mit* Schema und Host. Das Schema ist **nicht** auf Web beschränkt: `http`,
`https`, `ftp`, `mailto`, `file` und sogar erfundene Schemata gelten als gültig.
„example.com" (ohne Schema) ist hingegen keine gültige URL. Die Grenze definiert
allein die Crate — wir engen sie nicht auf http/https ein.
*Avoid*: URI, Link, Web-Adresse (suggeriert fälschlich „nur http/https")

**IP-Adresse**:
Eine wohlgeformte IP-Adresse im Sinne von `validator::ValidateIp` — **IPv4 oder
IPv6** sind beide eine IP-Adresse (`127.0.0.1` ebenso wie `::1`). Rein syntaktisch,
keine Erreichbarkeitsprüfung. **IPv4** und **IPv6** sind unterscheidbare Unterarten,
die getrennt prüfbar gemacht werden.
*Avoid*: Host, Hostname (das ist ein Name, keine Adresse)

## Beispieldialog

**Entwickler:** `validate_url("ftp://files.example.com")` gibt `True` — ist das ein Bug?

**Expertin:** Nein. Eine **URL** ist bei uns jede wohlgeformte absolute URL, egal
welches Schema. Wir reichen die `validator`-Crate treu durch und engen sie nicht auf
http/https ein.

**Entwickler:** Ich *will* aber nur Web-Adressen zulassen.

**Expertin:** Das ist eine eigene **Regel** über der Validierung — die gehört ins
Ergonomie-Paket, nicht ins Binding. Das Binding sagt nur „wohlgeformt: ja/nein".

**Entwickler:** Und `validate_length`? Das ist doch keine Form.

**Expertin:** Genau. `email`/`url`/`ip` sind **Format-Validatoren** (argumentlos,
prüfen die Form). `length`/`range`/`contains` sind **Constraint-Validatoren**
(parametrisiert, prüfen eine Bedingung). Beides ist **Validierung**, beides liefert
ein **Prädikat**.

**Entwickler:** Und wenn ich `validate_ip(None)` übergebe?

**Expertin:** `TypeError` — ein Programmierfehler, kein „ungültig". Ein kaputter
*String* gibt `False`; ein **falscher Typ** wirft. Und wenn bei Ungültigkeit ein
Fehler fliegen soll, brauchst du einen **Guard** — den gibt es hier nicht, er lebt
im Ergonomie-Paket.

**Entwickler:** `validate_ip("::1")`?

**Expertin:** `True` — eine **IP-Adresse** ist v4 *oder* v6. Willst du gezielt nur
v6, nimm `validate_ipv6`.
