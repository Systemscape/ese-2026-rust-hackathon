# Embedded Rust zum Anfassen: Hackathon auf dem exklusiven ESE 2026 Rust Devboard

> [!IMPORTANT]
> Gerenderte Dokumentation: https://systemscape.github.io/ese-2026-rust-hackathon/


[Das Board](hardware/board.md) · [Hardware](hardware/README.md) · [PCB-Downloads](downloads.md) · [CI](docs/ci.md)

## Worum geht es?

Rust ist auch in der Embedded-Entwicklung auf dem Vormarsch. Aber wie funktioniert “dieses Rust” auf echter Hardware? Wie schreibe ich einen Sensor-Treiber? Wie steuere ich ein Display an? Und wie sieht “Fearless Concurrency” in der Praxis aus?

Im ESE Hackathon finden Sie es heraus: In kleinen Teams entwickeln Sie mit Rust, Embassy und async eine eigene Anwendung zur Erfassung und Darstellung der Luftqualität auf dem Kongressgelände. Wie Sie Messwerte aufbereiten, darstellen und die Bedienung gestalten, entscheiden Sie selbst. Die überzeugendsten Anwendungen werden zum Abschluss ausgezeichnet.

Grundlage ist das exklusive ESE Rust Devboard mit RP2354-Mikrocontroller, Bosch BME690 Gassensor, Display, Buttons und integrierter Debug-Probe. Weitere Peripherie ist steckbar, damit Sie Ihre Anwendung später noch beliebig erweitern können.

Vier erfahrene Embedded-Rust-Entwickler begleiten Sie durchgehend, helfen bei Problemen zügig weiter und zeigen nötigenfalls Lösungswege auf. Zum Abschluss bewertet ein Expertengremium die Sensorlösungen und kürt die besten Embedded-Rust-Hacker.


**Voraussetzungen** 

* Laptop mit installierter Rust-Toolchain
* USB-C-Kabel zum Anschluss des Devboards
* Gute Programmierkenntnisse in C/C++ oder vergleichbar
* GitHub-Account für die Zusammenarbeit im Team

Vorteilhaft:

* erste Rust-Erfahrung; ESE-Hackathon-Repository vorab von GitHub geklont und kompiliert (spart Zeit vor Ort).


## Praktischer Nutzen

Nach dem Hackathon können Sie mit einer modernen, speichersicheren Sprache Embedded-Software entwickeln – und wissen aus eigener Erfahrung, wo Rust gegenüber C/C++ im Alltag punktet.

Konkret nehmen Sie mit:

* Das exklusive ESE Rust Devboard mit Bosch BME690 Gassensor, einem RP2354 Microcontroller (in Cortex-M und RISC-V Cores bootbar) und integriertem Debugger
* Ein Embedded-Rust-Projekt mit Embassy von Grund auf aufsetzen
* Einen Treiber für einen I²C-Sensor schreiben und ein Display ansteuern
* Nebenläufigkeit mit async Rust umsetzen – ohne RTOS und ohne Data Races
* Logs per RTT auslesen und eine Rust-Applikation mit probe-rs debuggen
* Peripherie wie GPIO und I²C über das embedded-hal-Ökosystem nutzen
* Embedded-Software im Team und mit Git-Workflow entwickeln

Das Gelernte lässt sich direkt auf andere Cortex-M-Plattformen übertragen.

***

# Vorbereitung

## Toolchain Setup

Bitte vor dem Hackathon erledigen, denn das WLAN vor Ort ist erfahrungsgemäß langsam.
Allgemeine Infos: <https://rust-lang.org/learn/get-started/>

### 1. Rust installieren

**Linux/macOS**
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
**Windows:** `rustup-init.exe` von <https://rustup.rs> herunterladen und ausführen. Die angebotenen Visual Studio C++ Build Tools mitinstallieren.

### 2. Targets für den RP2354 hinzufügen
```sh
# Cortex-M33 (wird im Hackathon genutzt)
rustup target add thumbv8m.main-none-eabihf
# RISC-V (Hazard3, zum selbst Ausprobieren)
rustup target add riscv32imac-unknown-none-elf
```

### 3. probe-rs installieren (Flashen & Debuggen)

**Linux/macOS**
```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```
**Windows (PowerShell)**
```powershell
irm https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.ps1 | iex
```
**Linux:** zusätzlich die udev-Regeln einrichten: <https://probe.rs/docs/getting-started/probe-setup/>

### 4. Repository klonen und Build testen
```sh
git clone https://github.com/Systemscape/ese-2026-rust-hackathon.git
cd ese-2026-rust-hackathon/firmware
cargo build
```
Wenn der Build durchläuft, ist alles bereit. Beim ersten Mal werden alle Abhängigkeiten heruntergeladen, das kann ein paar Minuten dauern.

### Editor (empfohlen)
VS Code mit der Erweiterung **rust-analyzer**, oder ein anderer Editor mit rust-analyzer-Unterstützung, zum Beispiel Zed.

## Rust lernen
- Übungen: <https://rustlings.rust-lang.org/>
- Das Rust Book: <https://doc.rust-lang.org/stable/book/>
- Interaktive Version mit Quizfragen: <https://rust-book.cs.brown.edu/>

## Kontaktdaten für Rückfragen
Per E-Mail oder Telefon: <https://www.systemscape.com/de/contact/>
