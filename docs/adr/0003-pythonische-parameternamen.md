# Pythonische Parameternamen statt Crate-Namen

Die parametrisierten Binding-Funktionen verwenden **pythonische, selbsterklärende**
Parameternamen statt der knappen Crate-Namen — z. B.
`validate_length(value, *, min_length=None, max_length=None)` statt der
Crate-Parameter `min`/`max`. Das ist eine **bewusste, dokumentierte Ausnahme** zur
Treue-Regel aus ADR 0001: Das *Verhalten* bleibt 1:1 zur Crate, nur die *Oberfläche*
wird pythonisiert. Gründe: Lesbarkeit für Python-Nutzer, Selbstdokumentation und
Vermeidung der Builtins `min`/`max` als Parameternamen.

## Consequences

- Wer das Binding gegen die Crate hält, sieht abweichende Parameternamen — das ist
  **Absicht**, kein Bug; nicht „zurückbenennen".
- Parameternamen sind Teil des öffentlichen Vertrags; Umbenennen ist ein Breaking
  Change.
- Die genaue Pythonisierung wird pro Validator beim Hinzufügen festgelegt
  (konsistente `snake_case`-Namen).
