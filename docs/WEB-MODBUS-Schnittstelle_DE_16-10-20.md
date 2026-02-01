# GUNTAMATIC Schnittstellenbeschreibung

Dokumentation für WEB/HTTP und MODBUS/TCP Schnittstellen

---

## Inhaltsverzeichnis

- [WEB-Schnittstelle](#web-schnittstelle)
  - [Bussystem "Ethernet"](#bussystem-ethernet)
  - [Datenzugriff](#datenzugriff)
  - [Externe Befehle](#externe-befehle)
  - [Demoseite](#demoseite)
- [MODBUS/TCP-Schnittstelle](#modbustcp-schnittstelle)
  - [Bussystem "Modbus/TCP"](#bussystem-modbustcp)
  - [Datenzugriff](#datenzugriff-1)
  - [Modbus Debugger](#modbus-debugger)
  - [Programmierbeispiel](#programmierbeispiel)
- [Nutzungsbedingungen](#nutzungsbedingungen)
- [Anhang](#anhang)

---

## WEB-Schnittstelle

### Bussystem "Ethernet"

Der Webserver ist über den TCP-Port 80 verfügbar.

### Datenzugriff

#### Ausgabeformat

Daten werden UTF-8 kodiert im JSON-Format ausgegeben:

```json
[{"id":Nummer,"name":"Bezeichnung","type":"Datentyp","unit":"Einheit"},…]
```

#### Berechtigungsstufen

- **W0**: keine Datenausgabe + externe Befehle (Endkunde)
- **W1**: Datenausgabe lt. Berechtigungsstufe W1 + externe Befehle (Endkunde)
- **W2**: Datenausgabe lt. Berechtigungsstufe W2 + externe Befehle (Servicepartner)

#### Datenzugriff

**Befehl**: `http://<ip>/ext/daqdata.cgi?key=<schlüssel>`

**Funktion**: Ausgabe der aktuellen Analogwerte und Zustände

**Ausgabe**: z.B.: `[25.50,-20.00,…]`

#### Mapping

Das Mapping kann über http-Request für die freigegeben Berechtigungsstufe über die Ethernet-Schnittstelle am BCE von der Steuerung ausgelesen werden. Zusätzlich sind die Mapping-Tabellen für die unterschiedlichen Produkte im Anhang dieses Dokumentes enthalten.

**Befehl**: `http://<ip>/ext/daqdesc.cgi?key=<schlüssel>`

**Funktion**: Ausgabe der aktuellen Beschreibung der ausgegebenen Analogwerte und Zustände

**Ausgabe**: z.B.:
```json
[{"id":3,"name":"Kesseltemperatur","type":"float","unit":"°C"},{"id":10,"name":"Puffer T5","type":"float","unit":"°C"},…]
```

### Externe Befehle

#### Einstellen der Kesselfreigabe

**Synonym bei Powerchip/Powercorn/Biocom/Pro**: PK002  
**Synonym bei Therm/Biostar**: K0010

| Befehl | Funktion | Ausgabe |
|--------|----------|---------|
| `http://<IP>/ext/parset.cgi?syn=<Synonym>&value=0&key=<schlüssel>` | Kesselfreigabe wird auf AUTO eingestellt | `{"ack":"Bestätigungsnachricht"}` oder `{"err":"Fehlermeldung"}` |
| `http://<ip>/ext/parset.cgi?syn=<Synonym>&value=1&key=<schlüssel>` | Kesselfreigabe wird auf AUS eingestellt | `{"ack":"Bestätigungsnachricht"}` oder `{"err":"Fehlermeldung"}` |
| `http://<ip>/ext/parset.cgi?syn=<Synonym>&value=2&key=<schlüssel>` | Kesselfreigabe wird auf EIN eingestellt | `{"ack":"Bestätigungsnachricht"}` oder `{"err":"Fehlermeldung"}` |

**Beispiel**:
```
http://10.0.0.101/ext/parset.cgi?syn=PK002&value=0&key=2221C5AE387A344F963EFA533881ACCE63AA
```
Funktion: Kesselfreigabe wird auf AUTO eingestellt

#### Einstellen des Reglerprogrammes

| Befehl | Funktion | Ausgabe |
|--------|----------|---------|
| `http://<ip>/ext/parset.cgi?syn=PR001&value=0&key=<schlüssel>` | Reglerprogramm wird auf AUS eingestellt | `{"ack":"Bestätigungsnachricht"}` oder `{"err":"Fehlermeldung"}` |
| `http://<ip>/ext/parset.cgi?syn=PR001&value=1&key=<schlüssel>` | Reglerprogramm wird auf NORMAL eingestellt | `{"ack":"Bestätigungsnachricht"}` oder `{"err":"Fehlermeldung"}` |
| `http://<ip>/ext/parset.cgi?syn=PR001&value=2&key=<schlüssel>` | Reglerprogramm wird auf WARMWASSER eingestellt | `{"ack":"Bestätigungsnachricht"}` oder `{"err":"Fehlermeldung"}` |
| `http://<ip>/ext/parset.cgi?syn=PR001&value=8&key=<schlüssel>` | Reglerprogramm wird auf HANDBETRIEB eingestellt (nur bei PC/BC/PH/TH/BS/PRO) | `{"ack":"Bestätigungsnachricht"}` oder `{"err":"Fehlermeldung"}` |

**Beispiel**:
```
http://10.0.0.101/ext/parset.cgi?syn=PR001&value=1&key=2221C5AE387A344F963EFA533881ACCE63AA
```
Funktion: Reglerprogramm wird auf NORMAL eingestellt

#### Einstellen des Heizkreisprogrammes

| Befehl | Funktion | Ausgabe |
|--------|----------|---------|
| `http://<ip>/ext/parset.cgi?syn=HKx01&value=0&key=<schlüssel>` | Heizkreis x wird auf AUS eingestellt | `{"ack":"Bestätigungsnachricht"}` oder `{"err":"Fehlermeldung"}` |
| `http://<ip>/ext/parset.cgi?syn=HKx01&value=1&key=<schlüssel>` | Heizkreis x wird auf NORMAL (Uhrenprogramm) eingestellt | `{"ack":"Bestätigungsnachricht"}` oder `{"err":"Fehlermeldung"}` |
| `http://<ip>/ext/parset.cgi?syn=HKx01&value=2&key=<schlüssel>` | Heizkreis x wird auf HEIZEN eingestellt | `{"ack":"Bestätigungsnachricht"}` oder `{"err":"Fehlermeldung"}` |
| `http://<ip>/ext/parset.cgi?syn=HKx01&value=3&key=<schlüssel>` | Heizkreis x wird auf ABSENKEN eingestellt | `{"ack":"Bestätigungsnachricht"}` oder `{"err":"Fehlermeldung"}` |

**Hinweis**: x=0..8 (Heizkreis)

**Beispiel**:
```
http://10.0.0.101/ext/parset.cgi?syn=HK201&value=3&key=2221C5AE387A344F963EFA533881ACCE63AA
```
Funktion: Heizkreis 2 wird auf ABSENKEN eingestellt

#### Einstellen der Warmwasser-Nachladung

**Befehl**: `http://<ip>/ext/parset.cgi?syn=BKx06&value=1&key=<schlüssel>`

**Funktion**: Warmwasser x wird auf NACHLADEN eingestellt

**Ausgabe**: `{"ack":"Bestätigungsnachricht"}` oder `{"err":"Fehlermeldung"}`

**Hinweis**: x=0..2 (Warmwasserkreis)

**Beispiel**:
```
http://10.0.0.101/ext/parset.cgi?syn=BK106&value=1&key=2221C5AE387A344F963EFA533881ACCE63AA
```
Funktion: Warmwasser 1 wird auf NACHLADEN eingestellt

#### Einstellen der Zusatz-Warmwasser-Nachladung

**Befehl**: `http://<ip>/ext/parset.cgi?syn=ZKx06&value=1&key=<schlüssel>`

**Funktion**: Zusatz-Warmwasser x wird auf NACHLADEN eingestellt

**Ausgabe**: `{"ack":"Bestätigungsnachricht"}` oder `{"err":"Fehlermeldung"}`

**Hinweis**: x=0..2 (Zusatz-Warmwasserkreis)

**Beispiel**:
```
http://10.0.0.101/ext/parset.cgi?syn=ZK206&value=1&key=2221C5AE387A344F963EFA533881ACCE63AA
```
Funktion: Zusatz-Warmwasser 2 wird auf NACHLADEN eingestellt

### Demoseite

**Befehl**: `http://<ip>/demo/index.htm`

**Funktion**: Öffnet eine Demoseite im Internet-Browser

**Beispiel**:
```
http://192.168.1.78/demo/index.htm
```

**Hinweis**: Bei Verwendung des Internetexplorers kann es vorkommen, dass JSON Dateien nicht im Klartext dargestellt werden. In diesem Fall wird empfohlen einen anderen Browser (z.B.: Firefox, Opera oder Chrome) zu verwenden.

---

## MODBUS/TCP-Schnittstelle

### Bussystem "Modbus/TCP"

Modbus TCP wird über TCP/IP-Pakete versendet. Die Verbindung wird auf TCP-Port 502 durchgeführt. Es kann immer nur eine Verbindung zu einem Zeitpunkt aufgebaut werden. Die Kesselsteuerung wird als Slave betrieben, d.h. es wird nur auf empfangene Befehle geantwortet.

#### Allgemeiner Protokollaufbau

| Byte Nr. | Größe (Byte) | Wert (Hex) | Beschreibung |
|----------|--------------|------------|--------------|
| 1-2 | 2 | xx xx | Transaktionsnummer bei mehreren gleichzeitigen Anfragen |
| 3-4 | 2 | 00 00 | Protokollkennzeichen (immer 0) |
| 5-6 | 2 | xx xx | Anzahl der folgenden Bytes |
| 7 | 1 | 00 | Geräteadresse |
| 8 | 1 | xx | Funktionscode |
| 9-n | n | | n Byte Daten |

Die übertragenen Daten sind oft auf mehrere Register verteilt. Die Daten werden Big-Endian übertragen (high-Byte wird zuerst übertragen).

**Hinweis**: Bei der Angabe der Leseadresse ist zwischen der Speicheradresse und der Registernummer zu unterscheiden! Register haben den Indexbereich 1 bis 65536. Während die Adressen den Bereich 0 bis 65535 haben. D.h. Register 1 liegt auf Adresse 0, Register 2 auf Adresse 1 usw.

#### Unterstützte Funktionen

- **03** - Read Holding Registers
- **04** - Read input registers
- **06** - Preset single register
- **16** - Preset multiple registers

#### Übersicht Adressbereiche

Die folgende Tabelle gibt den Zweck der Adressbereiche an. Auf nicht angegebene Adressen darf nicht zugegriffen werden. Der Registerbereich 0x101 bis 0x140 ist für die Schlüsseleingabe reserviert. Dieser Registerbereich kann nur mit Funktion 16 beschrieben werden, jedoch nicht ausgelesen werden. Für unterschiedlichen Funktionsbefehle ist der Registerbereich 0x201 bis 0x250 reserviert. Aktuelle Zustände bzw. Messwerte sind im Registerbereich 0x4001 bis 0x4400 enthalten. Eine Beschreibung über die Datentypen zu diesen Werten ist im Registerbereich 0x4801 bis 0x4C00 hinterlegt. Bei längeren Texten (>4 Zeichen) kann erweiterte Text mit bis zu 64 Zeichen ab Register 0x5001 ausgelesen werden. Es muss immer der ganze Text (32 Register) pro Anfrage ausgelesen werden.

| Registerbereich | Größe (Byte) | R/W | Funktion | Bezeichnung |
|-----------------|--------------|-----|----------|-------------|
| 0x0101 – 0x0140 | 128 | W | 16 | Schlüssel für weitere Berechtigungen |
| 0x0201 | 2 | R/W | 3,6,16 | Reglerprogramm |
| 0x0203 | 2 | R/W | 3,6,16 | Kesselfreigabe |
| 0x0211 – 0x0220 | 32 | R/W | 3,6,16 | Heizkreisprogramme (bis zu 16 HK) |
| 0x0231 – 0x0240 | 32 | R/W | 3,6,16 | Warmwasser-Nachladung (bis zu 16 WW) |
| 0x0241 – 0x0250 | 32 | R/W | 3,6,16 | Zusatz-Warmwasser-Nachladung (bis zu 16 ZWW) |
| 0x4001 – 0x4400 | 2048 | R | 4 | DAQ-Daten |
| 0x4801 – 0x4C00 | 2048 | R | 4 | DAQ-Typinformationen |
| 0x5001 – 0x5800 | 4096 | R | 4 | DAQ-Texte mit je max. 64 Zeichen |

### Datenzugriff

#### Berechtigungsstufen

In Abhängigkeit der Berechtigungsstufe (Schlüssel) werden die entsprechenden Register freigegeben:

- **M0**: keine Datenausgabe + externe Befehle (Endkunde)
- **M1**: Datenausgabe lt. Berechtigungsstufe M1 + externe Befehle (Endkunde)
- **M2**: Datenausgabe lt. Berechtigungsstufe M2 + externe Befehle (Servicepartner)

#### Zugriff freischalten

Bevor ein Datenzugriff vorgenommen werden kann, muss der Schlüssel mit der Funktion 16 (0x10) in die Register ab 0x0101 (Adresse 0x0100) als Text lt. Ascii geschrieben werden. Der Zugriff bleibt bis zur Unterbrechung der Verbindung geöffnet bzw. die Verbindung wird geschlossen wenn länger als 30 Sekunden kein Zugriff erfolgt. Danach muss der Schlüssel erneut übermittelt werden.

Der Schlüssel muss wie folgt aufbereitet werden:

1. Die einzelnen Zeichen des Schlüssel werden laut folgender Tabelle in Hex-Ascii Codes konvertiert:

| char | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'A' | 'B' | 'C' | 'D' | 'E' | 'F' |
|------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| Ascii HEX | 0x30 | 0x31 | 0x32 | 0x33 | 0x34 | 0x35 | 0x36 | 0x37 | 0x38 | 0x39 | 0x41 | 0x42 | 0x43 | 0x44 | 0x45 | 0x46 |

2. Jeweils zwei aufeinanderfolgende Hex-Codes werden zu einer 16 Bit Zahl zusammengefasst und dann fortlaufend in die Register ab 0x0101 geschrieben. In das letzte Register muss der Wert 0x0000 als Ende-Kennung geschrieben werden.

**Beispiel**:
```
Schlüssel = "2221C5AE387A344F963EFA533881ACCE63AA"
Schlüssellänge = 36 Zeichen
Registeranzahl = 18 + 1 Register für die Endekennung
```

| Register | 0x0101 | 0x0102 | 0x0103 | … | 0x111 | 0x112 | 0x0113 |
|----------|--------|--------|--------|---|-------|-------|--------|
| Schlüssel Zeichen | '2' '2' | '2' '1' | 'C' '5' | … | '6' '3' | 'A' 'A' | |
| Ascii Hex | 0x32 0x32 | 0x32 0x31 | 0x43 0x35 | … | 0x36 0x33 | 0x41 0x41 | |
| 16-Bit Zahl | 0x3232 | 0x3231 | | … | 0x3633 | 0x4141 | 0x0000 |

Falls die Schlüssellänge ungerade ist, dann muss in das freibleibende Byte im letzten Register die Endekennung geschrieben werden.

3. Der Request erfolgt dann mittels Modbus Funktion 16 (0x10 preset multiple registers) mit Startregister 0x0101 und der Anzahl zu schreibender Register.

**Hinweis**: Der Schlüssel kann nicht über Modbus ausgelesen werden. Das Schreiben des Schlüssels liefert immer den Status OK zurück, auch wenn der Schlüssel falsch oder unvollständig ist.

**Achtung**: Der Zugriff wird nach 10 ungültigen Versuchen für eine bestimmte Zeit gesperrt. Jeder weitere ungültige Versuch erhöht diese Sperrzeit. Durch einen Steuerungsneustart kann die Sperrzeit wieder zurückgesetzt werden.

#### Beispiel "Anmeldung mit Schlüssel"

Dieses Beispiel zeigt, wie der Schlüssel übermittelt wird.

```
Systemcode lt. Kessel-BCE in der "Detailanzeige-Netzwerk": GDNOUDD2
Schlüssel lt. GUNTAMATIC: "2221C5AE387A344F963EFA533881ACCE63AA"
Schlüssellänge = 36 Zeichen -> Anzahl zu schreibender Register = 36/2 + 1 = 19
```

Schlüssel in 2er-Paare aufteilen und in das Format HEX umwandeln:

| Register | 0x0101 | 0x0102 | 0x0103 | … | 0x0112 | 0x0113 |
|----------|--------|--------|--------|---|--------|--------|
| char | "22" | "21" | "C5" | … | "AA" | Endekennung |
| HEX | 0x3232 | 0x3231 | 0x4335 | … | 0x4141 | 0x0000 |

**Modbus TCP Anfrage "Preset Multiple Registers"** Start bei Register 0x0101 (Adr 0x100) Anzahl = 19 (0x13)

| Byte Nr. | Größe (Byte) | Wert (Hex) | Beschreibung |
|----------|--------------|------------|--------------|
| **Header** | | | |
| 1-2 | 2 | xx xx | Transaktionummer |
| 3-4 | 2 | 0000 | Protokollkennzeichen (immer 00 00) |
| 5-6 | 2 | 002d | Zahl der folgenden Bytes |
| 7 | 1 | 00 | Geräteadresse (wird ignoriert) |
| 8 | 1 | 10 | Funktionscode |
| **Funktion 16** | | | |
| 9-10 | 2 | 0100 | Startadresse Schreiben (Register 0x0101 - Offset 1) |
| 11-12 | 2 | 0013 | Anzahl der Register (Bytes * 2) |
| 13 | 1 | 26 | Anzahl der folgenden Bytes (0x26 = 38d) |
| 14-15 | 2 | 3232 | Register 0x0101 => 0x3232 "22" |
| 16-17 | 2 | 3231 | Register 0x0102 => 0x3231 "21" |
| 18-47 | 30 | … | Register 0x0103 bis 0x0111 |
| 48-49 | 2 | 4141 | Register 0x0112 => 0x4141 "AA" |
| 50-51 | 2 | 0000 | Register 0x0113 => 0x0000 (Schlüssel abgeschlossen) |

**Antwort:**

| Byte Nr. | Größe (Byte) | Wert (Hex) | Beschreibung |
|----------|--------------|------------|--------------|
| **Header** | | | |
| 1-2 | 2 | xx xx | Transaktionummer |
| 3-4 | 2 | 0000 | Protokollkennzeichen (immer 00 00) |
| 5-6 | 2 | 0006 | Zahl der folgenden Bytes |
| 7 | 1 | 00 | Geräteadresse (wird ignoriert) |
| 8 | 1 | 10 | Funktionscode |
| **F16** | | | |
| 9-10 | 2 | 0100 | Startadresse Schreiben |
| 11-12 | 2 | 0013 | Anzahl der Register |

#### Datenabfrage

Der Datenzugriff erfolgt mit der Funktion 04 des Modbus-Protokolls. Pro Wert sind zwei Register im Prozessorabbild reserviert. Der Adressebereich umfasst die Register von 0x4001 bis 0x4400. Die Register 0x4801 bis 0x4C00 sind die Typinformationen jedes Werts anrufbar.

**Hinweis**: Es muss immer der ganze Wert mit einer Abfrage abgerufen werden (d.h. es müssen 2 Register gelesen werden).

#### Mapping

##### Datentypen

Die einzelnen Werte können in vier unterschiedlichen Datentypen kodiert sein. Welcher Wert welchen Datentyp verwendet ist ebenfalls im Mapping ersichtlich.

- **int**: Ganzzahl mit Vorzeichen – Wertebereich −2.147.483.648 bis 2.147.483.647. Das Vorzeichen wird mithilfe des Zweierkomplements gebildet.
- **float**: Gleitkommazahl (32Bit) kodiert nach IEEE-754.
- **bool**: Boolscher Wert – das LSB hat den Wert 0 oder 1.
- **string**: Bis zu 4 Zeichen Text. Wenn der Text kürzer als 4 Zeichen ist, wird dieser mit dem Zeichen 0x00 terminiert. Es wird der Zeichensatz "iso-8859-1" verwendet.

##### Mapping auslesen

Das Mapping kann über eine http-Anfrage für die freigegebene Berechtigungsstufe über die Ethernet-Schnittstelle am BCE von der Steuerung ausgelesen werden. Zusätzlich sind die Mapping-Tabellen für die unterschiedlichen Produkte im Anhang dieses Dokumentes enthalten.

**Befehl**: `http://<ip>/mbmap.cgi?key=<schlüssel>`

**Funktion**: Ausgabe der aktuellen Beschreibung der ausgegebenen Analogwerte und Zustände

**Ausgabe**: Modbus Mappingtabelle für die freigegebene Berechtigungsstufe

**Beispiel "Mapping auslesen"**

Anfrage:
```
http://10.0.0.101/mbmap.cgi?key=2221C5AE387A344F963EFA533881ACCE63AA
```

#### Beispiel "DAQ Datenabfrage"

Dieses Beispiel zeigt, wie die ersten 4 Kanäle ausgelesen werden können.

**Anfrage:**

| Byte Nr. | Größe (Byte) | Wert (Hex) | Beschreibung |
|----------|--------------|------------|--------------|
| **Header** | | | |
| 1-2 | 2 | xx xx | Transaktionummer |
| 3-4 | 2 | 00 00 | Protokollkennzeichen (immer 00 00) |
| 5-6 | 2 | 00 06 | Zahl der folgenden Bytes |
| 7 | 1 | 00 | Geräteadresse (wird ignoriert) |
| 8 | 1 | 04 | Funktionscode |
| **F04** | | | |
| 9-10 | 2 | 40 00 | Startadresse Lesen |
| 11-12 | 2 | 00 08 | Anzahl der zu lesenden Register |

**Antwort:**

| Byte Nr. | Größe (Byte) | Wert (Hex) | Beschreibung |
|----------|--------------|------------|--------------|
| **Header** | | | |
| 1-2 | 2 | xx xx | Transaktionummer |
| 3-4 | 2 | 00 00 | Protokollkennzeichen (immer 00 00) |
| 5-6 | 2 | 00 13 | Zahl der folgenden Bytes |
| 7 | 1 | 00 | Geräteadresse (wird ignoriert) |
| 8 | 1 | 04 | Funktionscode |
| 9 | 1 | 10 | Anzahl der folgenden Datenbytes |
| **F04** | | | |
| 10-13 | 4 | xx xx xx xx | DAQ Kanal 0 |
| 14-17 | 4 | xx xx xx xx | DAQ Kanal 1 |
| 18-21 | 4 | xx xx xx xx | DAQ Kanal 2 |
| 22-25 | 4 | xx xx xx xx | DAQ Kanal 3 |

##### Wertdekodierung

1. **int**
   - `00 01 02 03` = 66051
   - `FF FF FF F6` = -10

2. **float**
   - `00 00 00 00` = 0.0
   - `C1 A0 00 00` = -20.0
   - `41 A0 00 00` = 20.0
   - `42 5D 33 33` = 55,3

3. **bool**
   - `00 00 00 00` = Aus
   - `00 00 00 01` = Ein

4. **string**
   - `41 55 53 00` = "AUS"

### Externe Befehle

Die Externen Befehle können mit der Funktion 06 (nur einen Wert ändern) oder der Funktion 16 (mehrere Werte ändern) durchgeführt werden. Der aktuelle Wert kann mit der Funktion 03 gelesen werden.

#### Einstellen des Reglerprogrammes

| Register | Adresse | AUS | NORMAL | WARMWASSER | HEIZEN | ABSENKEN | HANDBETRIEB |
|----------|---------|-----|--------|------------|--------|----------|-------------|
| | | **Wert** | **Wert** | **Wert** | **Wert** | **Wert** | **Wert** |
| 0x201 | 0x0200 | 00 00 | 00 01 | 00 02 | 00 03 | 00 04 | 00 08 |

#### Einstellen der Kesselfreigabe

| Register | Adresse | AUTO | AUS | DAUER |
|----------|---------|------|-----|-------|
| | | **Wert** | **Wert** | **Wert** |
| 0x0203 | 0x0202 | 00 00 | 00 01 | 00 02 |

#### Einstellen des Heizkreisprogramme

| | Register | Adresse | AUS | NORMAL | HEIZEN | ABSENKEN |
|--|----------|---------|-----|--------|--------|----------|
| | | | **Wert** | **Wert** | **Wert** | **Wert** |
| Programm HK0 | 0x0211 | 0x0210 | 00 00 | 00 01 | 00 02 | 00 03 |
| Programm HK1 | 0x0212 | 0x0211 | 00 00 | 00 01 | 00 02 | 00 03 |
| Programm HK2 | 0x0213 | 0x0212 | 00 00 | 00 01 | 00 02 | 00 03 |
| Programm HK3 | 0x0214 | 0x0213 | 00 00 | 00 01 | 00 02 | 00 03 |
| Programm HK4 | 0x0215 | 0x0214 | 00 00 | 00 01 | 00 02 | 00 03 |
| Programm HK5 | 0x0216 | 0x0215 | 00 00 | 00 01 | 00 02 | 00 03 |
| Programm HK6 | 0x0217 | 0x0216 | 00 00 | 00 01 | 00 02 | 00 03 |
| Programm HK7 | 0x0218 | 0x0217 | 00 00 | 00 01 | 00 02 | 00 03 |
| Programm HK8 | 0x0219 | 0x0218 | 00 00 | 00 01 | 00 02 | 00 03 |

#### Einstellen der Warmwasser-Nachladung und Zusatz-Warmwasser-Nachladung

| | Register | Adresse | NORMAL | NACHLADEN |
|--|----------|---------|--------|-----------|
| | | | **Wert** | **Wert** |
| WW 0 Nachladen | 0x0231 | 0x0230 | 00 00 | 00 01 |
| WW 1 Nachladen | 0x0232 | 0x0231 | 00 00 | 00 01 |
| WW 2 Nachladen | 0x0233 | 0x0232 | 00 00 | 00 01 |
| ZWW 0 Nachladen | 0x0241 | 0x0240 | 00 00 | 00 01 |
| ZWW 1 Nachladen | 0x0242 | 0x0241 | 00 00 | 00 01 |
| ZWW 2 Nachladen | 0x0243 | 0x0242 | 00 00 | 00 01 |

#### Beispiel "Parameter auslesen"

Dieses Beispiel zeigt, wie die aktuelle Einstellung des Reglerprogrammes ausgelesen wird.

**Anfrage:**

| Byte Nr. | Größe (Byte) | Wert (Hex) | Beschreibung |
|----------|--------------|------------|--------------|
| **Header** | | | |
| 1-2 | 2 | xx xx | Transaktionummer |
| 3-4 | 2 | 00 00 | Protokollkennzeichen (immer 00 00) |
| 5-6 | 2 | 00 06 | Zahl der folgenden Bytes |
| 7 | 1 | 00 | Geräteadresse (wird ignoriert) |
| 8 | 1 | 03 | Funktionscode |
| **F03** | | | |
| 9-10 | 2 | 02 00 | Startadresse Lesen |
| 11-12 | 2 | 00 01 | Anzahl der zu lesenden Register |

**Antwort:**

| Byte Nr. | Größe (Byte) | Wert (Hex) | Beschreibung |
|----------|--------------|------------|--------------|
| **Header** | | | |
| 1-2 | 2 | xx xx | Transaktionummer |
| 3-4 | 2 | 00 00 | Protokollkennzeichen (immer 00 00) |
| 5-6 | 2 | 00 05 | Zahl der folgenden Bytes |
| 7 | 1 | 00 | Geräteadresse (wird ignoriert) |
| 8 | 1 | 03 | Funktionscode |
| 9 | 1 | 02 | Anzahl der folgenden Datenbytes |
| **F03** | | | |
| 10-11 | 2 | 00 xx | Reglerprogramm |

#### Beispiel "Parameter schreiben"

Dieses Beispiel zeigt, wie das Reglerprogramm auf NORMAL eingestellt wird.

| Byte Nr. | Größe (Byte) | Wert (Hex) | Beschreibung |
|----------|--------------|------------|--------------|
| **Header** | | | |
| 1-2 | 2 | xx xx | Transaktionummer |
| 3-4 | 2 | 00 00 | Protokollkennzeichen (immer 00 00) |
| 5-6 | 2 | 00 06 | Zahl der folgenden Bytes |
| 7 | 1 | 00 | Geräteadresse (wird ignoriert) |
| 8 | 1 | 06 | Funktionscode |
| **F06** | | | |
| 9-10 | 2 | 02 00 | Registeradresse |
| 11-12 | 2 | 00 01 | Neuer Wert (1..Normal) |

### Fehlerbehandlung

Wenn invalide Nachrichten empfangen werden wird die Verbindung von der Steuerung geschlossen. Dies ist der Fall wenn:
- Modbus-Paket zu groß ist, oder
- mehr Daten empfangen werden als im Header angegeben wurden.

In allen anderen Fällen wird eine Modbus-Ausnahme als Antwort zurückgesendet.

#### Fehlercodes

Falls bei der Bearbeitung des Modbus Telegramms ein Fehler auftritt, wird ein standardisierter Fehlercode zurückgesendet.

| Fehlercode | Bezeichnung | Beschreibung |
|------------|-------------|--------------|
| 1 | Illegal Function | Diese Modbus-Funktion wird nicht unterstützt. |
| 2 | Illegal Data Address | Eine ungültige Register-Adresse wurde zugegriffen. |
| 3 | Illegal Data Value | Datenwert ungültig. |
| 4 | Slave Device failure | Ein Fehler ist bei der Verarbeitung des Befehls aufgetreten. |
| 5 | Slave Device busy | Steuerung kann zur Zeit keine Befehle verarbeiten. |

#### Beispiel "Fehlerhafte Anfrage"

Dieses Beispiel zeigt, wie aufgrund einer fehlerhaften Anfrage ein Fehler zurückgeliefert wird.

**Anfrage:**

| Byte Nr. | Größe (Byte) | Wert (Hex) | Beschreibung |
|----------|--------------|------------|--------------|
| **Header** | | | |
| 1-2 | 2 | xx xx | Transaktionummer |
| 3-4 | 2 | 00 00 | Protokollkennzeichen (immer 00 00) |
| 5-6 | 2 | 00 06 | Zahl der folgenden Bytes |
| 7 | 1 | 00 | Geräteadresse (wird ignoriert) |
| 8 | 1 | 06 | Funktionscode |
| **F06** | | | |
| 9-10 | 2 | 02 00 | Registeradresse |
| 11-12 | 2 | 00 07 | Neuer Wert ist nicht erlaubt |

**Antwort:**

| Byte Nr. | Größe (Byte) | Wert (Hex) | Beschreibung |
|----------|--------------|------------|--------------|
| **Header** | | | |
| 1-2 | 2 | xx xx | Transaktionummer |
| 3-4 | 2 | 00 00 | Protokollkennzeichen (immer 00 00) |
| 5-6 | 2 | 00 03 | Zahl der folgenden Bytes |
| 7 | 1 | 00 | Geräteadresse (wird ignoriert) |
| 8 | 1 | 86 | Funktionscode + 0x80 → Fehler |
| 9 | 1 | 03 | Fehlercode |

### Modbus Debugger

Zum Testen der Modbus-Programmierung wird empfohlen die Testsoftware "Guntamatic Modbus Debugger" zu verwenden. Diese Software ermöglicht die Überprüfung der Anmeldung mit Schlüssel, Abfrage der DAQ-Datentypen und Rohdaten und der Abfrage der Sonderregister für längere Texte.

#### Anmeldung

In diesem Tabellenreiter ist die Anmeldung lt. Kapitel "Zugriff freischalten" dargestellt.

#### Parameter

Mit diesem Tabellenreiter können einzelne Parameter entsprechend Kapitel "Externe Befehle" geändert werden.

#### DAQ Typen/Rohdaten/konvertierte Daten/Texte

Mit diesen Tabellenreitern kann DAQ-Datenabfrage entsprechend Kapitel "Datenabfrage" getestet werden.

- DAQ-Datentypen (0x4801 – 0x4C00)
- DAQ-Rohdaten (0x4001 – 0x4400)
- Konvertierte DAQ-Daten
- Sonderkanäle für längere Texte (0x5001 – 0x5800)

### Programmierbeispiel

#### Beispiel mit "libmodbus"

"libmodbus" ist eine freie Programmbibliothek welche für die Modbus-Kommunikation verwendet werden kann. Die folgenden Angaben beziehen sich auf das Betriebssystem "Ubuntu 14.04".

##### Installation

```bash
sudo apt-get install libmodbus-dev
```

##### Kompilieren

```bash
gcc -I/usr/include/modbus -o modbus main.c -lmodbus
```

##### Ausführen

**Betriebsart abfrage:**
```bash
./modbus ip key 0x200 1
```

**DAQ Kanäle abfragen:**
```bash
./modbus ip key 0x4006 2
```

##### Programmcode "main.c"

```c
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <errno.h>
#include <modbus.h>
#include <arpa/inet.h> /* htons */

static int write_key(modbus_t *ctx, const char *key) {
    uint16_t loginkey[32];
    /* key length in words plus terminating 0 */
    uint16_t keylen = (strlen(key) / 2) + 1;
    strlcpy(loginkey, key, sizeof(loginkey));

    /* change byte order */
    uint16_t i;
    for(i = 0; i < keylen; i++) {
        loginkey[i] = htons(loginkey[i]);
    }

    return modbus_write_registers(ctx, 0x100, keylen, loginkey);
}

int main(int argc, char * argv[]) {
    modbus_t *ctx;
    uint16_t addr = 0;
    uint16_t resp[125];
    uint16_t resp_daq_type[125];
    uint16_t reg_to_read = 0;
    uint16_t i = 0;
    int ret = -1;

    if(argc < 5) {
        printf("usage %s <ip address> <key> <start address> <number of registers>\n", argv[0]);
        printf("start address = register - 1\n");
        return -1;
    }

    addr = strtol(argv[3], NULL, 0);
    reg_to_read = strtol(argv[4], NULL, 0);
    if(reg_to_read > 125) {
        fprintf(stderr, "Too many registers requested\n");
        return -1;
    }

    ctx = modbus_new_tcp(argv[1], 502);
    if (modbus_connect(ctx) == -1) {
        fprintf(stderr, "Connection failed: %s\n", modbus_strerror(errno));
        modbus_free(ctx);
        return -1;
    }

    if(write_key(ctx, argv[2]) == -1) {
        fprintf(stderr, "Write key failed: %s\n", modbus_strerror(errno));
        goto close;
    }

    /* if address points to a daq channel */
    if(addr >= 0x4000 && addr < 0x4400) {
        /* read data */
        if(modbus_read_input_registers(ctx, addr, reg_to_read, resp) == -1) {
            fprintf(stderr, "Read input register %d failed: %s\n", addr, modbus_strerror(errno));
            goto close;
        }
        /* read type information */
        if(modbus_read_input_registers(ctx, addr+0x0800, reg_to_read, resp_daq_type) == -1) {
            fprintf(stderr, "Read input register %d failed: %s\n", addr+0x0800, modbus_strerror(errno));
            goto close;
        }
        /* print data */
        for(i = 0; i < reg_to_read; i+=2) {
            uint32_t val = resp[i] << 16 | resp[i+1];
            uint32_t val_reversed = ntohl(val);
            uint32_t type = resp_daq_type[i] << 16 | resp_daq_type[i+1];
            printf("reg[0x%04X..0x%04X] = 0x%08X = ", addr + i + 1, addr + i + 2, val);
            if(val==0xFFFFFFFF)
                printf("no value defined\n");
            else if(val==0xFFFFFFFE)
                printf("access denied\n");
            else {
                switch(type) {
                    case 1: printf("int(%d)\n", *(int*)(&val)); break;
                    case 2: printf("float(%f)\n", *(float*)(&val)); break;
                    case 3: printf("bool(%s)\n", val ? "true" : "false"); break;
                    case 4: printf("string(%.4s)\n", (char*)&val_reversed); break;
                    default: printf("undefined type\n"); break;
                }
            }
        }
    } else {
        if(modbus_read_registers(ctx, addr, reg_to_read, resp) == -1) {
            fprintf(stderr, "Read register %d failed: %s\n", addr + 1, modbus_strerror(errno));
            goto close;
        }

        for(i = 0; i < reg_to_read; i++) {
            printf("reg[0x%04X] = %04X\n", addr + i + 1, resp[i]);
        }
    }

    ret = 0;

close:
    modbus_close(ctx);
    modbus_free(ctx);

    return ret;
}
```

---

## Nutzungsbedingungen

### Geltungsbereich

Die vorliegenden Nutzungsbedingungen gelten für die Nutzung nachfolgender Softwareprogramme und Dokumente (im weiteren Text kurz als "Anwendung" bezeichnet) und alle im Rahmen dieser Anwendungen angebotenen Leistungen und Informationen: "GSM Anwendungen", "Kesselvisualisierung", "Schnittstellenbeschreibung", "Guntamatic Modbus Debugger", "Guntamatic APP und/oder zugehörige WEB Programme".

Die Nutzung einer oder mehrerer dieser Anwendungen ist ausschließlich aufgrund dieser Nutzungsbedingungen zulässig und erfordert eine Registrierung des Nutzers. Abweichungen von diesen Nutzungsbedingungen erfordern eine ausdrückliche schriftliche Bestätigung durch GUNTAMATIC Heiztechnik GmbH, im weiteren Text kurz als "GUNTAMATIC" bezeichnet.

### Nutzungsberechtigung

Nutzungsberechtigt sind ausschließlich juristische oder natürliche, volljährige und vollständig eigenberechtigte Personen, welche eine GUNTAMATIC-Heizanlage betreiben, GUNTAMATIC-Heizanlagen einbauen oder Vertriebs- und Servicetätigkeiten für GUNTAMATIC-Geräte durchführen. Natürliche Personen vor Vollendung des 18. Lebensjahres oder volljährige, nicht voll eigenberechtigte Personen sind nur nach ausdrücklicher Zustimmung des gesetzlichen Vertreters zur Nutzung berechtigt.

### Nutzungsbedingung

Eine erlaubte Nutzung der Anwendungen verpflichtet den Nutzer zur Registrierung bei GUNTAMATIC. Der Nutzer ist hierbei zur wahrheitsgemäßen Angabe seiner Daten verpflichtet.

Es wird ausdrücklich festgehalten, dass ausschließlich die Nutzung der für Kunden freigegebenen Berechtigungsstufe erlaubt ist. Kesselprogrammier-oder interne Ebenen von GUNTAMATIC dürfen ausschließlich durch Werksangehörige, Werksvertretungen oder schriftlich Berechtigte genutzt werden. Im Falle einer widerrechtlichen Nutzung von nicht erlaubten Ebenen behält sich GUNTAMATIC vor, evtl. entstehende Schäden einzufordern (als Mindestschadenssumme gelten € 10.000,- als vereinbart).

Der Nutzer ist verpflichtet, alle erforderlichen Vorkehrungen im Heiz- und Lagerraum sowie an der Heizanlage (Kessel, Fördertechnik, Brennstofflagerraum, Anlagenhydraulik) zu treffen, um eine im Zusammenhang mit den Anwendungen stehende Bedienung oder Fernbedienung der Anlage hinsichtlich Sach- und Personenschäden gefahrenlos zu ermöglichen. Unter anderem ist der Lagerraum ständig verschlossen zu halten, Wartungs-, Reinigungs- oder Störungsbehebungstätigkeiten dürfen ausnahmslos nur bei stromlosem Gerät (Netzstecker abgezogen) durchgeführt werden.

**ACHTUNG**: Eine Nutzung darf keinesfalls das Betriebsverhalten des Gerätes verändern (z. B. permanentes Ein- und Ausschalten des Gerätes in ähnlicher Weise wie ein Ölbrenner führt bei Biomasseheizungen zu massivem Fehlverhalten)

Eine kostenfreie Nutzung berechtigt GUNTAMATIC dazu, sämtliche Programmierungen und Programme (sowie zugehörige Hardwareinformationen) welche in direktem Zusammenhang mit der Nutzung stehen einzufordern, im Hinblick auf mögliche Risiken div. Einschränkungen oder eine völlige Einstellung der Programm-Anwendung zu verlangen oder ggfs. derartige Programme auch selbst nutzen zu dürfen.

Der Nutzer ist verpflichtet, die Zugangsdaten zu den Anwendungen sorgfältig zu verwahren und diese vor Missbrauch zu schützen. Verletzt er diese Verpflichtung oder überlässt er die Zugangsdaten Dritten oder macht er die Daten sonst öffentlich zugänglich, so haftet der Nutzer für alle Handlungen und Unterlassungen, die unter Verwendung seiner Zugangsdaten vorgenommen werden.

Für den Fall der widerrechtlichen Nutzungserlangung des Zugangs durch Dritte (z.B. Password-Sniffing, Hacking, etc.) verpflichtet sich der Nutzer zur unverzüglichen Meldung an GUNTAMATIC nach Kenntnisnahme des Missbrauchs oder des bloßen Verdachtes hiervon.

Es ist dem Nutzer untersagt, die Anwendungen technisch über die vorhandenen Konfigurationsmöglichkeiten hinaus zu modifizieren oder zu anderen als den technischen vorgesehenen Zwecken zu nutzen.

Jegliche gewerbliche Vermarktung von Hard- oder Software, welche in direktem Zusammenhang mit der kostenfreien Nutzung steht, ist strikt untersagt.

Unabhängig der Unterfertigung dieser Nutzungsbedingungen oder einer nicht durchgeführten Registrierung ist der Inhalt dieser Nutzungsvereinbarung mit Start einer Nutzung als gültig und verbindlich zu betrachten, da diese Nutzungsbedingungen mit jeder Dokumentation veröffentlicht und kenntlich gemacht wurden.

Im Falle einer Verletzung dieser Nutzungsbedingungen haftet der Nutzer für alle verletzungskausalen Schäden, die GUNTAMATIC unmittelbar oder mittelbar entstehen, einschließlich der Kosten für eine zweckentsprechende Rechtsverfolgung.

### Abänderung von Nutzungsbedingungen

GUNTAMATIC garantiert, dass im Sinne von Abänderungen der Nutzungsbedingungen keine ungewollten Kosten entstehen können, da jedem Nutzer für derartige Abänderungen ein sofortiges Kündigungsrecht eingeräumt wird.

GUNTAMATIC behält sich das Recht vor, die vorliegenden Nutzungsbedingungen jederzeit abzuändern und den Nutzer über die Änderungen der für ihn geltenden Nutzungsbedingungen zu informieren. Diese Bekanntgabe erfolgt in der Regel elektronisch, entbindet den Nutzer aber nicht, sich über die Aktualität der Nutzungsbedingungen selbst zu informieren.

Wenn der Nutzer gegebenenfalls geänderten Nutzungsbedingungen nicht innerhalb einer Frist von zwei Wochen nach Bekanntgabe der Änderung der Nutzungsbedingungen widerspricht oder ohne Widerspruchserhebung in der Nutzung der Anwendungen fortfährt, gelten die geänderten Nutzungsbedingungen als angenommen.

Wird einer Änderung der Nutzungsbedingungen widersprochen, ist die weitere Nutzung nicht mehr erlaubt (auch nicht zu den bisherigen Bedingungen). Eine folglich fortgesetzte, von GUNTAMATIC stillschweigend tolerierte oder nicht beachtete Nutzung bedeutet nicht, dass die bisherigen Nutzungsbedingungen in Geltung bleiben oder die Nutzung weiter erlaubt ist.

### Kündigung

Die zur Gänze entgeltlose Nutzungsmöglichkeit ist seitens GUNTAMATIC jederzeit frei widerruflich. Der Nutzer hat keinen Rechtsanspruch auf eine fortgesetzte Nutzung der Anwendungen.

Der Nutzer ist umgekehrt jederzeit berechtigt, die Nutzung der Anwendungen ohne Angabe von Gründen einzustellen und die Löschung seiner Registrierung zu verlangen.

GUNTAMATIC ist zur Darbietung und/oder Aufrechterhaltung der Anwendungen nicht verpflichtet.

GUNTAMATIC behält sich das Recht vor, die angeführten Anwendungen jederzeit ganz oder teilweise ohne Angabe von Gründen abzuändern, einzustellen oder die Zugangsberechtigung durch Sperrung der Zugangsdaten zu widerrufen.

### Datenschutzerklärung

Zur Leistungserbringung gegenüber dem Nutzer werden von GUNTAMATIC elektronische, automatisierte Datenverarbeitungsanlagen bzw. -einrichtungen eingesetzt. Zur Nutzung der Anwendungen ist die Registrierung mit personenbezogenen Daten wie insbesondere der Name und die Adresse des Nutzers erforderlich. Diese werden im notwendigen Ausmaß von GUNTAMATIC erfasst und für die Dauer der Registrierung gespeichert. Zum Zwecke der stetigen Verbesserung der Anwendungen werden ausschließlich nicht personenbezogene Daten wie etwa Fehlermeldungen von GUNTAMATIC statistisch erfasst und gespeichert.

Der Nutzer ist berechtigt, jederzeit das Recht auf Auskunft über die diesbezüglich gespeicherten Daten geltend zu machen und nach Maßgabe der gesetzlichen Bestimmungen die Berichtigung oder Löschung dieser Daten zu verlangen.

Die von GUNTAMATIC gespeicherten personenbezogenen Daten werden ausschließlich von GUNTAMATIC zum Zweck der Leistungserbringung und –entwicklung verwendet. Eine Weitergabe oder Veröffentlichung dieser Daten erfolgt nicht, es sei denn, es bedarf dieser Weitergabe oder Veröffentlichung zur Erfüllung einer gesetzlichen Verpflichtung oder Durchsetzung oder Verteidigung von Rechten von GUNTAMATIC.

Die Nutzung der personenbezogenen Daten zum Zweck der Informierung der Nutzer über die Produkte von GUNTAMATIC bedarf des gesonderten Einverständnisses des Nutzers.

### Gewährleistung und Haftung

Eine kostenfreie Nutzung versteht sich nicht als Rechtsgeschäft sondern als kostenfreie Überlassung einer Anwendung mit einer einhergehenden Verpflichtung zur Selbstprüfung der damit verbundenen Sicherheit.

Die von GUNTAMATIC zur Verfügung gestellten Anwendungen wurden sorgfältig erstellt und grundsätzlichen Funktions- und Fehleranalysen unterzogen. GUNTAMATIC leistet aber keine Gewähr für die Verfügbarkeit, Funktionsfähigkeit, Aktualität, Qualität, Verwendbarkeit und Weiterentwicklungen der Anwendungen oder der von diesen erstellten Daten und Inhalten.

GUNTAMATIC haftet weder für Sach- und Personenschäden im Zusammenhang mit der Nutzung der Anwendungen, noch für Schäden, welche infolge missbräuchlicher Verwendung der Zugangsdaten oder der Veröffentlichung oder Überlassung der Zugangsdaten an Dritte durch einen berechtigen oder unberechtigten Nutzer entstehen.

### Gerichtsstand

Diese Nutzungsbedingungen unterliegen ausschließlich österreichischem Recht. Für Streitigkeiten aus oder im Zusammenhang mit der Nutzung der Anwendungen ist das Landesgericht Wels (Österreich) ausschließlicher Gerichtsstand.

---

## Anhang

### Mapping

#### Level 1 (Endkunde)

Siehe Dokument "Mapping_Level-1_DE_WEB-MODBUS_xx-yy-zz"

#### Level 2 (Servicepartner)

Siehe Dokument "Mapping_Level-2_DE_WEB-MODBUS_xx-yy-zz"

---

*GUNTAMATIC ©2020 - Änderungen vorbehalten*
