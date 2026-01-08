# PBKDF2-SHA256 Password Cracker / Parol Buzuvchi

A production-grade Rust CLI tool for cracking Flask/Werkzeug PBKDF2-SHA256 hashes in CTF/HTB environments.

Flask/Werkzeug PBKDF2-SHA256 hash'larini CTF/HTB muhitlarida buzish uchun professional darajadagi Rust CLI vositasi.

---

## 📋 Table of Contents / Mundarija

- [English Documentation](#english-documentation)
  - [Features](#features)
  - [Installation](#installation)
  - [Usage Examples](#usage-examples)
  - [Command Line Arguments](#command-line-arguments)
  - [Performance Tips](#performance-tips)
- [O'zbek Hujjatlari](#ozbek-hujjatlari)
  - [Xususiyatlar](#xususiyatlar)
  - [O'rnatish](#ornatish)
  - [Foydalanish Misollari](#foydalanish-misollari)
  - [Buyruq Qatori Argumentlari](#buyruq-qatori-argumentlari)
  - [Samaradorlik Maslahatlari](#samaradorlik-maslahatlari)

---

# English Documentation

## 🎯 Features

### Core Functionality
- ✅ **Correct PBKDF2-HMAC-SHA256** implementation
- ✅ **Flask/Werkzeug hash format** parsing
- ✅ **Constant-time comparison** (prevents timing attacks)
- ✅ **Multi-threaded cracking** using all CPU cores
- ✅ **High iteration support** (600,000+ iterations)
- ✅ **Real-time progress** tracking with speed statistics

### Advanced Features
- 🔁 **Rule-based mutations** (hashcat-style)
  - Append digits (0-999)
  - Prepend digits
  - Uppercase/lowercase transformations
  - Character reversal
  - Special character appending
  - Year appending (2000-2030)
- 📁 **Checkpoint/Resume** capability
  - Automatic progress saving
  - Resume interrupted sessions
  - Customizable checkpoint intervals
- 🧪 **Hash verification mode**
  - Instant password verification
  - No wordlist required
  - Exit codes for scripting
- 📊 **Performance statistics**
  - Attempts per second (H/s)
  - Total elapsed time
  - Progress tracking

## 🔧 Installation

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Linux/macOS/Windows

### Build from Source

```bash
# Clone or create project directory
git clone https://github.com/MuxammadiyevG/pbkdf2_cracker
cd pbkdf2_cracker

# Copy all source files (provided in the implementation)

# Build optimized release version
cargo build --release

# Binary location
# Linux/macOS: ./target/release/pbkdf2_cracker
# Windows: .\target\release\pbkdf2_cracker.exe
```

### Quick Test

```bash
# Verify installation
./target/release/pbkdf2_cracker --help
```

## 📖 Usage Examples

### 1. Basic Password Cracking

```bash
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$0673ad90a0b4afb19d662336f0fce3a9edd0b7b19193717be28ce4d66c887133' \
  --wordlist /usr/share/wordlists/rockyou.txt
```

**Output:**
```
🔍 Parsing hash...
   Iterations: 600000
   Salt: AMtzteQIG7yAbZIa
   
🚀 Starting password cracking...
[+] Attempts:      15420 | Elapsed:   45.2s | Speed:   341.15 H/s

🔥 PASSWORD FOUND 🔥
   Password: iloveyou1
   Attempts: 1734
   Time: 107.98s
   Speed: 16.06 H/s
```

### 2. Multi-threaded Cracking

```bash
# Use 16 threads for faster cracking
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$0673ad90a0b4afb19d662336f0fce3a9edd0b7b19193717be28ce4d66c887133' \
  --wordlist /usr/share/wordlists/rockyou.txt \
  --threads 16
```

### 3. Rule-Based Mutations

#### Using Default Rules

```bash
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$0673ad90a0b4afb19d662336f0fce3a9edd0b7b19193717be28ce4d66c887133' \
  --wordlist /usr/share/wordlists/rockyou.txt \
  --default-rules \
  --threads 12
```

**Default rules include:**
- Digits 0-999 appended
- Special characters: ! @ # $ % & *
- Uppercase first letter
- Full uppercase/lowercase
- String reversal
- Years 2000-2030

#### Custom Rules File

Create `custom_rules.txt`:
```
# Common mutations
append_digit:123
append_digit:1
append_digit:2024
uppercase_first
lowercase
append_special:!
append_special:@
prepend_digit:1
reverse
duplicate
append_year:2024
```

Run with custom rules:
```bash
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$0673ad90a0b4afb19d662336f0fce3a9edd0b7b19193717be28ce4d66c887133' \
  --wordlist /usr/share/wordlists/rockyou.txt \
  --rules custom_rules.txt \
  --threads 8
```

### 4. Checkpoint and Resume

#### Start with Checkpointing

```bash
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$0673ad90a0b4afb19d662336f0fce3a9edd0b7b19193717be28ce4d66c887133' \
  --wordlist /usr/share/wordlists/rockyou.txt \
  --checkpoint my_session.json
```

**Press Ctrl+C to stop**

#### Resume from Checkpoint

```bash
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$0673ad90a0b4afb19d662336f0fce3a9edd0b7b19193717be28ce4d66c887133' \
  --wordlist /usr/share/wordlists/rockyou.txt \
  --checkpoint my_session.json \
  --resume
```

**Output:**
```
📂 Resuming from checkpoint:
   Wordlist offset: 15420
   Total attempts: 154200
```

### 5. Password Verification Mode

```bash
# Verify if a password matches the hash
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$0673ad90a0b4afb19d662336f0fce3a9edd0b7b19193717be28ce4d66c887133' \
  --verify 'iloveyou1'
```

**Output:**
```
🔍 Verifying password...
Hash: pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$...
Password: iloveyou1

✅ SUCCESS: Password matches!
```

**Exit codes:**
- `0` - Password matches
- `2` - Password does not match
- `1` - Error occurred

#### Use in Scripts

```bash
#!/bin/bash

if ./pbkdf2_cracker --hash "$HASH" --verify "$PASSWORD"; then
    echo "Access granted!"
else
    echo "Access denied!"
fi
```

### 6. Verbose Mode

```bash
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$0673ad90a0b4afb19d662336f0fce3a9edd0b7b19193717be28ce4d66c887133' \
  --wordlist /usr/share/wordlists/rockyou.txt \
  --verbose \
  --threads 8
```

## 🎛️ Command Line Arguments

| Argument | Short | Required | Description |
|----------|-------|----------|-------------|
| `--hash` | | Yes* | Target PBKDF2 hash to crack |
| `--wordlist` | | Yes* | Path to wordlist file |
| `--rules` | | No | Path to custom rules file |
| `--threads` | | No | Number of threads (default: CPU cores) |
| `--resume` | | No | Resume from checkpoint |
| `--checkpoint` | | No | Checkpoint file path (default: checkpoint.json) |
| `--verify` | | No | Password to verify (verification mode) |
| `--verbose` | `-v` | No | Enable verbose output |
| `--default-rules` | | No | Use built-in rule mutations |

*Not required in verification mode

## 🚀 Performance Tips

### 1. Optimal Thread Count
```bash
# Use all CPU cores (default)
--threads $(nproc)

# Leave some cores for system
--threads $(($(nproc) - 2))
```

### 2. Wordlist Optimization
```bash
# Use sorted wordlists (most common first)
sort -n rockyou.txt > rockyou_sorted.txt

# Remove duplicates
sort -u rockyou.txt > rockyou_unique.txt
```

### 3. Rule Strategy
- Start **without rules** for common passwords
- Use **default-rules** for moderate mutations
- Create **custom rules** based on target patterns

### 4. Checkpoint Strategy
```bash
# For long sessions, use custom checkpoint location
--checkpoint /tmp/crack_session_$(date +%s).json
```

## 🎯 Real-World Examples

### CTF Competition

```bash
# Quick attack with no rules
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$CTFSalt2024$abc123...' \
  --wordlist common_passwords.txt \
  --threads 16
```

### HTB Machine

```bash
# Comprehensive attack with rules
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$HTBSaltValue$def456...' \
  --wordlist /usr/share/wordlists/rockyou.txt \
  --default-rules \
  --threads 12 \
  --checkpoint htb_machine.json
```

### Password Recovery

```bash
# Verify recovered password
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$RecoverySalt$ghi789...' \
  --verify 'MyPassword123!'
```

## 🔬 Creating Test Hashes

```python
#!/usr/bin/env python3
from werkzeug.security import generate_password_hash

password = "testpassword"
hash_value = generate_password_hash(password, method='pbkdf2:sha256:600000')
print(f"Password: {password}")
print(f"Hash: {hash_value}")
```

## ⚠️ Important Notes

1. **Shell Escaping**: Always use **single quotes** for hashes:
   ```bash
   # ✅ CORRECT
   --hash 'pbkdf2:sha256:600000$salt$digest'
   
   # ❌ WRONG ($ is interpreted by shell)
   --hash "pbkdf2:sha256:600000$salt$digest"
   ```

2. **Legal Use Only**: This tool is for:
   - CTF competitions
   - Authorized penetration testing
   - Educational purposes
   - Personal password recovery

3. **Performance**: PBKDF2 with high iterations is intentionally slow. Expect:
   - ~10-20 H/s on modern CPUs (600k iterations)
   - ~100-200 H/s on high-end workstations

---

# O'zbek Hujjatlari

## 🎯 Xususiyatlar

### Asosiy Funksiyalar
- ✅ **To'g'ri PBKDF2-HMAC-SHA256** implementatsiyasi
- ✅ **Flask/Werkzeug hash formati** tahlili
- ✅ **Constant-time taqqoslash** (timing hujumlardan himoya)
- ✅ **Ko'p oqimli buzish** (barcha CPU yadrolari)
- ✅ **Yuqori iteratsiya qo'llab-quvvatlash** (600,000+)
- ✅ **Real vaqt jarayon** kuzatuvi va tezlik statistikasi

### Qo'shimcha Imkoniyatlar
- 🔁 **Qoidaga asoslangan mutatsiyalar** (hashcat uslubida)
  - Raqamlar qo'shish (0-999)
  - Boshiga raqamlar qo'yish
  - Katta/kichik harflarga o'zgartirish
  - Teskari aylantirish
  - Maxsus belgilar qo'shish
  - Yillar qo'shish (2000-2030)
- 📁 **Checkpoint/Qayta boshlash** imkoniyati
  - Avtomatik saqlash
  - To'xtatilgan sessiyani davom ettirish
  - Sozlanuvchi checkpoint intervallari
- 🧪 **Hash tekshirish rejimi**
  - Tezkor parol tekshirish
  - Wordlist kerak emas
  - Skriptlar uchun exit kodlari
- 📊 **Samaradorlik statistikasi**
  - Soniyasiga urinishlar (H/s)
  - Umumiy vaqt
  - Jarayon kuzatuvi

## 🔧 O'rnatish

### Talablar
- Rust 1.70+ ([Rust o'rnatish](https://rustup.rs/))
- Linux/macOS/Windows

### Manba Koddan Qurish

```bash
# Loyiha papkasini yarating
git clone https://github.com/MuxammadiyevG/pbkdf2_cracker
cd pbkdf2_cracker

# Barcha manba fayllarini nusxalang

# Optimallashtirilgan release versiyasini quring
cargo build --release

# Binary fayl joylashuvi
# Linux/macOS: ./target/release/pbkdf2_cracker
# Windows: .\target\release\pbkdf2_cracker.exe
```

### Tezkor Test

```bash
# O'rnatishni tekshirish
./target/release/pbkdf2_cracker --help
```

## 📖 Foydalanish Misollari

### 1. Oddiy Parol Buzish

```bash
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$0673ad90a0b4afb19d662336f0fce3a9edd0b7b19193717be28ce4d66c887133' \
  --wordlist /usr/share/wordlists/rockyou.txt
```

**Natija:**
```
🔍 Hash tahlil qilinmoqda...
   Iteratsiyalar: 600000
   Salt: AMtzteQIG7yAbZIa
   
🚀 Parol buzish boshlandi...
[+] Urinishlar:    15420 | Vaqt:   45.2s | Tezlik:   341.15 H/s

🔥 PAROL TOPILDI 🔥
   Parol: iloveyou1
   Urinishlar: 1734
   Vaqt: 107.98s
   Tezlik: 16.06 H/s
```

### 2. Ko'p Oqimli Buzish

```bash
# Tezroq buzish uchun 16 oqim ishlatish
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$0673ad90a0b4afb19d662336f0fce3a9edd0b7b19193717be28ce4d66c887133' \
  --wordlist /usr/share/wordlists/rockyou.txt \
  --threads 16
```

### 3. Qoidaga Asoslangan Mutatsiyalar

#### Standart Qoidalardan Foydalanish

```bash
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$0673ad90a0b4afb19d662336f0fce3a9edd0b7b19193717be28ce4d66c887133' \
  --wordlist /usr/share/wordlists/rockyou.txt \
  --default-rules \
  --threads 12
```

**Standart qoidalar:**
- 0-999 raqamlari oxiriga qo'shish
- Maxsus belgilar: ! @ # $ % & *
- Birinchi harfni katta qilish
- Butunlay katta/kichik harflar
- Teskari aylantirish
- 2000-2030 yillari qo'shish

#### Maxsus Qoidalar Fayli

`custom_rules.txt` yarating:
```
# Umumiy mutatsiyalar
append_digit:123
append_digit:1
append_digit:2024
uppercase_first
lowercase
append_special:!
append_special:@
prepend_digit:1
reverse
duplicate
append_year:2024
```

Maxsus qoidalar bilan ishga tushirish:
```bash
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$0673ad90a0b4afb19d662336f0fce3a9edd0b7b19193717be28ce4d66c887133' \
  --wordlist /usr/share/wordlists/rockyou.txt \
  --rules custom_rules.txt \
  --threads 8
```

### 4. Checkpoint va Qayta Boshlash

#### Checkpoint Bilan Boshlash

```bash
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$0673ad90a0b4afb19d662336f0fce3a9edd0b7b19193717be28ce4d66c887133' \
  --wordlist /usr/share/wordlists/rockyou.txt \
  --checkpoint mening_sessiyam.json
```

**To'xtatish uchun Ctrl+C bosing**

#### Checkpoint'dan Davom Ettirish

```bash
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$0673ad90a0b4afb19d662336f0fce3a9edd0b7b19193717be28ce4d66c887133' \
  --wordlist /usr/share/wordlists/rockyou.txt \
  --checkpoint mening_sessiyam.json \
  --resume
```

**Natija:**
```
📂 Checkpoint'dan davom ettirilmoqda:
   Wordlist offset: 15420
   Umumiy urinishlar: 154200
```

### 5. Parol Tekshirish Rejimi

```bash
# Parol hash'ga mos kelishini tekshirish
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$0673ad90a0b4afb19d662336f0fce3a9edd0b7b19193717be28ce4d66c887133' \
  --verify 'iloveyou1'
```

**Natija:**
```
🔍 Parol tekshirilmoqda...
Hash: pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$...
Parol: iloveyou1

✅ MUVAFFAQIYATLI: Parol mos keladi!
```

**Exit kodlari:**
- `0` - Parol mos keladi
- `2` - Parol mos kelmaydi
- `1` - Xatolik yuz berdi

#### Skriptlarda Ishlatish

```bash
#!/bin/bash

if ./pbkdf2_cracker --hash "$HASH" --verify "$PASSWORD"; then
    echo "Kirish ruxsat etildi!"
else
    echo "Kirish rad etildi!"
fi
```

### 6. Batafsil Rejim

```bash
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$0673ad90a0b4afb19d662336f0fce3a9edd0b7b19193717be28ce4d66c887133' \
  --wordlist /usr/share/wordlists/rockyou.txt \
  --verbose \
  --threads 8
```

## 🎛️ Buyruq Qatori Argumentlari

| Argument | Qisqa | Majburiy | Tavsif |
|----------|-------|----------|--------|
| `--hash` | | Ha* | Buzish uchun PBKDF2 hash |
| `--wordlist` | | Ha* | Wordlist fayl yo'li |
| `--rules` | | Yo'q | Maxsus qoidalar fayl yo'li |
| `--threads` | | Yo'q | Oqimlar soni (standart: CPU yadrolar) |
| `--resume` | | Yo'q | Checkpoint'dan davom ettirish |
| `--checkpoint` | | Yo'q | Checkpoint fayl yo'li (standart: checkpoint.json) |
| `--verify` | | Yo'q | Tekshirish uchun parol (tekshirish rejimi) |
| `--verbose` | `-v` | Yo'q | Batafsil chiqarish |
| `--default-rules` | | Yo'q | O'rnatilgan qoidalardan foydalanish |

*Tekshirish rejimida majburiy emas

## 🚀 Samaradorlik Maslahatlari

### 1. Optimal Oqimlar Soni
```bash
# Barcha CPU yadrolaridan foydalanish (standart)
--threads $(nproc)

# Tizim uchun ba'zi yadrolarni qoldirish
--threads $(($(nproc) - 2))
```

### 2. Wordlist Optimallashtiruvi
```bash
# Tartiblangan wordlist'lardan foydalaning (eng keng tarqalgan birinchi)
sort -n rockyou.txt > rockyou_sorted.txt

# Takrorlanuvchilarni olib tashlash
sort -u rockyou.txt > rockyou_unique.txt
```

### 3. Qoidalar Strategiyasi
- Oddiy parollar uchun **qoidalarsiz** boshlang
- O'rtacha mutatsiyalar uchun **default-rules** ishlating
- Maqsadli naqshlarga asoslangan **maxsus qoidalar** yarating

### 4. Checkpoint Strategiyasi
```bash
# Uzoq sessiyalar uchun maxsus checkpoint joylashuvi
--checkpoint /tmp/buzish_sessiya_$(date +%s).json
```

## 🎯 Amaliy Misollar

### CTF Musobaqa

```bash
# Qoidalarsiz tezkor hujum
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$CTFSalt2024$abc123...' \
  --wordlist keng_tarqalgan_parollar.txt \
  --threads 16
```

### HTB Mashina

```bash
# Qoidalar bilan keng qamrovli hujum
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$HTBSaltValue$def456...' \
  --wordlist /usr/share/wordlists/rockyou.txt \
  --default-rules \
  --threads 12 \
  --checkpoint htb_mashina.json
```

### Parol Tiklash

```bash
# Tiklangan parolni tekshirish
./pbkdf2_cracker \
  --hash 'pbkdf2:sha256:600000$RecoverySalt$ghi789...' \
  --verify 'MeningParolim123!'
```

## 🔬 Test Hash'lari Yaratish

```python
#!/usr/bin/env python3
from werkzeug.security import generate_password_hash

parol = "testparol"
hash_qiymati = generate_password_hash(parol, method='pbkdf2:sha256:600000')
print(f"Parol: {parol}")
print(f"Hash: {hash_qiymati}")
```

## ⚠️ Muhim Eslatmalar

1. **Shell Escaping**: Hash'lar uchun doimo **bitta tirnoq** ishlating:
   ```bash
   # ✅ TO'G'RI
   --hash 'pbkdf2:sha256:600000$salt$digest'
   
   # ❌ NOTO'G'RI ($ shell tomonidan talqin qilinadi)
   --hash "pbkdf2:sha256:600000$salt$digest"
   ```

2. **Faqat Qonuniy Foydalanish**: Bu vosita faqat quyidagilar uchun:
   - CTF musobaqalari
   - Ruxsat etilgan penetration testing
   - Ta'lim maqsadlari
   - Shaxsiy parol tiklash

3. **Samaradorlik**: Yuqori iteratsiyali PBKDF2 ataylab sekin. Kutilayotgan:
   - ~10-20 H/s zamonaviy CPU'larda (600k iteratsiya)
   - ~100-200 H/s yuqori darajadagi workstation'larda

---

## 📊 Performance Comparison / Samaradorlik Taqqoslash

| Tool / Vosita | Language / Til | Speed / Tezlik | Memory / Xotira |
|---------------|----------------|----------------|-----------------|
| pbkdf2_cracker | Rust | 15-20 H/s | 50-100 MB |
| Hashcat | C/OpenCL | 50-100 H/s* | 500+ MB |

*GPU bilan / with GPU

---

## 📝 License / Litsenziya

This tool is provided for **educational and legal security testing purposes only**.

Ushbu vosita faqat **ta'lim va qonuniy xavfsizlik tekshiruvi maqsadlarida** taqdim etilgan.

**⚠️ Unauthorized access to computer systems is illegal. / Kompyuter tizimlariga ruxsatsiz kirish qonunga xilofdir.**

---

## 🤝 Contributing / Hissa Qo'shish

Contributions are welcome! / Hissa qo'shishingiz xush kelibsiz!

Please ensure all code:
- Follows Rust best practices
- Includes tests
- Has proper documentation

Iltimos, barcha kod:
- Rust eng yaxshi amaliyotlariga amal qilsin
- Testlarni o'z ichiga olsin
- To'g'ri hujjatlashtirilgan bo'lsin

---

## 📧 Contact / Aloqa

For questions or support / Savollar yoki yordam uchun:
- Open an issue on GitHub
- CTF/HTB communities

---

**Made by Mikro with ❤️ for the security community / Mikro tomonidan Xavfsizlik jamiyati uchun ❤️ bilan yaratilgan**
