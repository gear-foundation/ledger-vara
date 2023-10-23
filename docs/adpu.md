# APDU Commands

## General

Request structure:

| Field:        | CLA   | INS  | P1  | P2  | Data len  | Data      |
|-------------- | ----- | ---- | --- | --- | --------- | --------- |
| Offset:       | 0     | 1    | 2   | 3   | 4         | 5         |
| Size (bytes): | 1     | 1    | 1   | 1   | 1         | 0–255     |
| Value:        | 0x89  | Any  | Any | Any | 0–255     | Any       |

- **CLA**: Class (0x89 for Vara application)
- **INS**: Instruction
- **P1**: Parameter 1
- **P2**: Parameter 2
- **Data len**: Length of the data field
- **Data**: Data field

Response structure:

| Field:        | Data      | Status |
| ------------- | --------- | ------ |
| Offset:       | 0         | 0–255  |
| Size (bytes): | 0–255     | 2      |

- **Data**: Data field
- **Status**: Status code

Status codes:

| Status | Description         |
| ------ | ------------------- |
| 0x6982 | Nothing received    |
| 0x6d00 | Unknown error       |
| 0x6e00 | Invalid CLA         |
| 0x6e01 | Invalid INS         |
| 0x6e02 | Invalid P1/P2       |
| 0x6e03 | Invalid data length |
| 0x9000 | Success             |
| 0xe000 | Panic error         |

## Get Version

### Request

| Field:        | CLA  | INS  | P1 | P2 | Data len | Data |
|-------------- | ---- | ---- | -- | -- | -------- | ---- |
| Offset:       | 0    | 1    | 2  | 3  | 4        | 5    |
| Size (bytes): | 1    | 1    | 1  | 1  | 1        | 1    |
| Value:        | 0x89 | 0x00 | —  | —  | —        | —    |

Example: `0x8900`

### Response

| Field:        | Major | Minor | Patch | Status |
| ------------- |------ | ----- | ----- | ------ |
| Offset:       | 0     | 2     | 4     | 6      |
| Size (bytes): | 2     | 2     | 2     | 2      |
| Endianness:   | BE    | BE    | BE    | BE     |

Example (v1.20.2840): `0x0001'0014'0B18'9000`

## Get Public Key

### Request

| Field:        | CLA  | INS  | P1    | P2    | Data len  | Data      |
|-------------- | ---- | ---- | ----- | ----- | --------- | --------- |
| Offset:       | 0    | 1    | 2     | 3     | 4         | 5         |
| Size (bytes): | 1    | 1    | 1     | 1     | 1         | 20        |
| Value:        | 0x89 | 0x01 | 0 / 1 | 0 / 1 | 20 (0x14) | See below |

**P1**: Scheme

- **0**: ED25519
- **1**: SR25519

**P2**: Interactive mode with confirmation

- **0**: false (non-interactive mode)
- **1**: true (interactive mode)

**Data**:

| Data             | Size (bytes) | Endianness | Value              |
| ---------------- | ------------ | ---------- | ------------------ |
| Purpose = 44'    | 4            | LE         | 0x8000002c         |
| Coin Type = 913' | 4            | LE         | 0x80000391         |
| Account Type     | 4            | LE         | 0x80000000 + Type  |
| Change           | 4            | LE         | 0x80000000         |
| Address Index    | 4            | LE         | 0x80000000 + Index |

Example: `0x8901'0000'14'2c000080'91030080'00000080'00000080'00000080`

### Response

| Field:        | Public Key | Status |
| ------------- | ---------- | ------ |
| Offset:       | 0          | 32     |
| Size (bytes): | 32         | 2      |
| Endianness:   | BE         | BE     |

Example: `0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d'9000`

## Sign the Message

### Request

| Field:        | CLA  | INS  | P1    | P2        | Data len | Data      |
|-------------- | ---- | ---- | ----- | --------- | -------- | --------- |
| Offset:       | 0    | 1    | 2     | 3         | 4        | 5         |
| Size (bytes): | 1    | 1    | 1     | 1         | 1        | Depends   |
| Value:        | 0x89 | 0x02 | 0 / 1 | 0 / 1 / 2 | Depends  | See below |

**P1**: Scheme

- **0**: ED25519
- **1**: SR25519

**P2**: Subcommand

- **0**: Init
- **1**: Append Message
- **2**: Get Signature

**Init Data**:

- Data len: 20 (0x14)

| Data             | Size (bytes) | Endianness | Value              |
| ---------------- | ------------ | ---------- | ------------------ |
| Purpose = 44'    | 4            | LE         | 0x8000002c         |
| Coin Type = 913' | 4            | LE         | 0x80000391         |
| Account Type     | 4            | LE         | 0x80000000 + Type  |
| Change           | 4            | LE         | 0x80000000         |
| Address Index    | 4            | LE         | 0x80000000 + Index |

**Append Message Data**:

| Data          | Size (bytes)   |
| ------------- | -------------- |
| Message bytes | Message length |

Example:

- Init: `0x8902'0000'14'2c000080'91030080'00000080'00000080'00000080`
- Append Message (`Hello`): `0x8902'0001'05'48656c6c6f`
- Get Signature: `0x8902'0002'00`

### Response

| Field:        | Signature | Status |
| ------------- | --------- | ------ |
| Offset:       | 0         | 64     |
| Size (bytes): | 64        | 2      |
| Endianness:   | BE        | BE     |

Example: `0x09ae68a39f24d6ded499875b9242a4aecaff15c7a7d44fcf634a091c3223a8f2802cefdcb65e1297e645f5c3f288f29ad79e82994bf4f7e85659d6bd3d67f80e9000'9000`
