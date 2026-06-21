# CLAUDE.md

Dieser Leitfaden hilft Claude Code (claude.ai/code) bei der Arbeit in diesem Repository.

## Projektüberblick

**rusty-validator** ist ein schmales **Python-Binding** der Rust-Crate
[`validator`](https://github.com/Keats/validator), gebaut mit **PyO3** und
**maturin**. Es stellt Validierungs-**Prädikate** bereit: Funktionen, die prüfen, ob
eine Zeichenkette wohlgeformt ist (E-Mail-Adresse, URL, IP-Adresse) und `True`/`False`
zurückgeben. Die eigentliche Prüf-Logik liefert die Crate — dieses Projekt reicht sie
nur treu nach Python durch.

**Vor Änderungen lesen** — die Domänensprache und die tragenden Entscheidungen sind
hier festgehalten:

- [CONTEXT.md](CONTEXT.md) — Glossar (Validieren, Format-/Constraint-Validator,
  E-Mail-Adresse, URL, IP-Adresse) + Beispieldialog. Reines Glossar, keine
  Implementierungsdetails.
- [docs/adr/](docs/adr/) — Architektur-Entscheidungen:
  - [0001](docs/adr/0001-treues-binding-der-validator-crate.md) — Treues Binding der
    `validator`-Crate (Crate = Vorgabe, mit dokumentierten Ausnahmen)
  - [0002](docs/adr/0002-reines-praedikat-binding-ergonomie-separat.md) — Reines
    Prädikat-Binding; Ergonomie (Guards/Decorators/Constraint-Objekte) in separatem Paket
  - [0003](docs/adr/0003-pythonische-parameternamen.md) — Pythonische Parameternamen
  - [0004](docs/adr/0004-strikter-typ-vertrag.md) — Strikter Typ-Vertrag: `TypeError`
    bei Typfehler, sonst Prädikat

## Architektur

Drei Schichten, von Rust nach Python:

1. **Rust** ([src/lib.rs](src/lib.rs)): Jede Validierung ist eine `#[pyfunction]`
   (`validate_email`/`validate_url`/`validate_ip`), die an die `validator`-Crate
   delegiert. Registriert werden sie im `#[pymodule] _validator`.
2. **maturin** baut die Rust-`cdylib` zur nativen Erweiterung `rusty_validator._validator`
   (siehe `[tool.maturin]` in [pyproject.toml](pyproject.toml): `python-source = "python"`,
   `module-name = "rusty_validator._validator"`).
3. **Python** ([`python/rusty_validator/__init__.py`](python/rusty_validator/__init__.py))
   re-exportiert die Funktionen aus `._validator`; die Typen stehen im Stub
   [`__init__.pyi`](python/rusty_validator/__init__.pyi).

**Leitprinzipien** (Details in den ADRs):

- **Treues Binding**: Verhalten = `validator`-Crate. Überraschende Crate-Ergebnisse
  (z. B. `validate_url("ftp://x")` ist `True`, `validate_ip("::1")` ist `True`) sind
  beabsichtigt geerbt — **nicht** „reparieren". Abweichungen sind dokumentierte
  Ausnahmen (ADR 0001).
- **Reines Prädikat**: Jede Funktion gibt `bool` zurück. Kein „wirf-bei-ungültig"-Stil
  (Guard), keine Constraint-Objekte, keine Decorators — die gehören in ein **separates**
  Ergonomie-Paket (ADR 0002).
- **Pythonische Parameternamen**: Parametrisierte Validatoren bekommen lesbare Namen
  (z. B. `min_length` statt Crate-`min`) — bewusste Ausnahme zu ADR 0001 (ADR 0003).
- **Strikter Typ-Vertrag**: Korrekt typisierte, aber ungültige Eingabe → `False`;
  **typ-falsche** Eingabe (z. B. `None`) → `TypeError`, kein stilles `False` (ADR 0004).

## Repository-Struktur

```text
rusty-validator/
├── CLAUDE.md                     # Diese Datei
├── CONTEXT.md                    # Domänen-Glossar
├── README.md                     # Nutzer-Doku
├── Cargo.toml / Cargo.lock       # Rust-Crate (Name, Version, Dependencies)
├── pyproject.toml                # maturin-Build + mypy-Konfiguration
├── docs/adr/                     # Architektur-Entscheidungen (ADRs)
├── src/lib.rs                    # Rust: #[pyfunction]s + #[pymodule]
├── python/rusty_validator/
│   ├── __init__.py               # Re-Export der nativen Funktionen
│   ├── __init__.pyi              # Typ-Stubs der öffentlichen API
│   └── py.typed                  # Markiert das Paket als typisiert (PEP 561)
├── tests/                        # pytest-Suite (parametrisiert)
└── .github/workflows/CI.yml      # Wheels bauen (Multi-Target) + PyPI-Release bei Tags
```

## Entwicklungsbefehle

[uv](https://docs.astral.sh/uv/) verwaltet venv, Dev-Dependencies (`[dependency-groups]`
in [pyproject.toml](pyproject.toml)) und die Python-Version ([.python-version](.python-version));
**maturin bleibt das Build-Backend** (`[build-system]`). Einmal `uv sync` holt die Tools
ins `.venv`; danach laufen alle Befehle ohne globale Installation via `uv run`. (Ohne uv
funktionieren die blanken Befehle im aktiven venv weiterhin.)

```bash
uv sync                    # .venv + Dev-Tools (pytest, mypy, maturin) aus uv.lock
```

### Bauen & in das venv installieren (Entwicklung)

```bash
uv run maturin develop            # kompiliert die Rust-Erweiterung in das venv
uv run maturin develop --release  # optimierter Build (langsamer zu bauen, schneller zur Laufzeit)
```

Nach jeder Änderung an [src/lib.rs](src/lib.rs) muss `maturin develop` neu laufen, bevor
die Python-Tests die Änderung sehen — **uv recompiliert kompilierte Projekte nicht
automatisch**.

### Tests

```bash
uv run pytest              # Python-Tests (benötigt vorher: maturin develop)
uv run pytest --cov        # mit Coverage
```

Es gibt bewusst **keine** Rust-`#[cfg(test)]`-Tests: das Binding wird über die
pytest-Suite gegen die echte Extension geprüft (treuer als ein In-Process-Embedded-Test),
und die `validator`-Crate testen wir nicht nach (ADR 0001).

### Typprüfung (mypy-Config in pyproject.toml)

```bash
uv run mypy python         # prüft Stubs/Python-Quelle (disallow_untyped_defs aktiv)
```

### Rust-Standardwerkzeuge

```bash
cargo fmt                  # formatieren
cargo clippy               # Lints
```

### Wheels bauen

```bash
uv run maturin build --release    # baut ein Wheel nach target/wheels/
```

## Einen neuen Validator hinzufügen

Ziel des Projekts ist breite Abdeckung der `validator`-Crate (siehe CONTEXT.md). Beim
Hinzufügen eines Validators die ADR-Prinzipien einhalten:

1. **Rust** ([src/lib.rs](src/lib.rs)): `#[pyfunction] fn validate_x(...) -> PyResult<bool>`
   anlegen, an die passende `validator`-Trait-Methode delegieren, und im
   `#[pymodule] _validator` per `wrap_pyfunction!` registrieren.
   - **Format-Validator** (argumentlos): Signatur `f(value: String) -> PyResult<bool>`.
   - **Constraint-Validator** (parametrisiert): pythonische Keyword-Parameter (ADR 0003);
     Ergebnis bleibt `bool` (ADR 0002).
2. **Python-Oberfläche**: Funktion in [`__init__.py`](python/rusty_validator/__init__.py)
   (`__all__`) und im Stub [`__init__.pyi`](python/rusty_validator/__init__.pyi) ergänzen.
3. **Tests** ([tests/](tests/)): parametrisierte `pytest`-Fälle (gültig **und** ungültig);
   für den Typ-Vertrag auch einen `TypeError`-Fall (ADR 0004).
4. `maturin develop` ausführen, dann `pytest`.
5. **`graphify update .`** ausführen, um den Wissensgraphen aktuell zu halten.

## Release

- Die veröffentlichte Version steht in [Cargo.toml](Cargo.toml) (`[package].version`);
  Python bezieht sie dynamisch daraus.
- Ein **Git-Tag** auslösen → [CI.yml](.github/workflows/CI.yml) baut Wheels für alle
  Targets und published zu PyPI (`MATURIN_PYPI_TOKEN`).

## Codebase-Navigation (graphify)

Dieses Projekt hat einen Wissensgraphen unter `graphify-out/` (god nodes, Community-
Struktur, dateiübergreifende Beziehungen).

Regeln:

- Für Fragen zur Codebasis zuerst `graphify query "<frage>"` ausführen, wenn
  `graphify-out/graph.json` existiert. `graphify path "<A>" "<B>"` für Beziehungen,
  `graphify explain "<konzept>"` für fokussierte Konzepte. Diese liefern einen
  begrenzten Subgraphen, meist deutlich kleiner als `GRAPH_REPORT.md` oder rohe
  grep-Ausgabe.
- Existiert `graphify-out/wiki/index.md`, dieses zur groben Navigation nutzen statt
  rohem Quelltext-Browsing.
- `graphify-out/GRAPH_REPORT.md` nur für breite Architektur-Reviews lesen oder wenn
  query/path/explain nicht genug Kontext liefern.
- Nach Code-Änderungen `graphify update .` ausführen (nur AST, keine API-Kosten).
