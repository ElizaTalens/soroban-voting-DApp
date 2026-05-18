# Soroban Project
Aplikasi desentralisasi (DApp) berbasis smart contract untuk sistem pemungutan suara (voting) yang transparan, aman, dan tanpa perantara, dibangun di atas jaringan Stellar menggunakan Soroban.

## ✨ Fitur Utama
- **Manajemen Kandidat**: Admin memiliki otorisasi untuk melakukan pendaftaran kandidat di dalam sistem (lengkap dengan ID, Nama, dan perhitungan suara).
- **Kontrol Periode Voting**: Admin dapat membuka atau menutup sesi pemungutan suara sesuai dengan jadwal.
- **Satu Pemilih, Satu Suara**: Menggunakan `Address` identitas dari Stellar untuk melacak status `Voted` sehingga memastikan setiap pemilih hanya dapat memberikan suaranya satu kali (mencegah *double-voting* atau *sybil attack*).
- **Transparan & On-Chain**: Seluruh proses perhitungan suara dan data yang tersimpan berjalan secara terbuka serta *immutable* di atas jaringan blockchain.

## 🚀 Teknologi yang Digunakan
- **Bahasa Pemrograman**: [Rust](https://www.rust-lang.org/) (dengan lingkungan `#![no_std]`)
- **Smart Contract Platform**: [Soroban SDK](https://soroban.stellar.org/)
- **Blockchain**: Stellar Network

## 📋 Prasyarat Instalasi
Sebelum menjalankan atau meng-compile proyek ini, pastikan Anda telah menyiapkan beberapa alat pengembangan berikut:
1. **Rust & Cargo**: Instal melalui [rustup](https://rustup.rs/).
2. **Target WebAssembly**: Tambahkan target kompilasi WebAssembly (WASM) dengan menjalankan:

```

```text
File README.md generated successfully.

```bash
   rustup target add wasm32-unknown-unknown

```

3. **Soroban CLI**: Alat antarmuka baris perintah (CLI) untuk berinteraksi dengan kontrak Soroban.
```bash
cargo install --locked soroban-cli

```

## 📂 Susunan Project

Struktur proyek ini mengikuti standar Cargo Workspace untuk proyek Soroban:

```text
soroban-voting-dapp/
├── Cargo.toml               # Konfigurasi workspace Cargo
├── README.md                # Dokumentasi utama proyek
└── contracts/
    └── hello-world/         # Direktori modul smart contract
        ├── Cargo.toml       # Dependensi spesifik contract
        ├── Makefile         # Kumpulan script utilitas untuk proses build & test
        └── src/
            ├── lib.rs       # Logika utama (state, struct Candidate, inisialisasi)
            └── test.rs      # Skenario pengujian (unit testing)

```

## 💻 Contoh Penggunaan

### 1. Build Smart Contract

Jalankan perintah berikut pada direktori *root* untuk mengkompilasi *smart contract* menjadi *file* `.wasm`:

```bash
cargo build --target wasm32-unknown-unknown --release

```

### 2. Menjalankan Unit Test

Pastikan seluruh fungsi, termasuk validasi *auth* dan perhitungan suara, berjalan mulus melalui perintah *test*:

```bash
cargo test

```

### 3. Deploy & Interaksi

Dengan Soroban CLI, Anda dapat langsung memasang dan menginisialisasi *smart contract* ini:

```bash
# 1. Melakukan Deploy contract ke jaringan
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/hello_world.wasm

# 2. Inisialisasi contract (menentukan akun Admin)
soroban contract invoke \\
    --id <CONTRACT_ID> \\
    -- \\
    initialize \\
    --admin <ADMIN_ADDRESS>

```

## 🤝 Kontribusi

Kami sangat mengapresiasi kontribusi untuk memperkaya fitur aplikasi DApp ini!

1. Lakukan **Fork** pada repositori.
2. Buat *branch* fitur Anda (`git checkout -b fitur-keren-anda`).
3. Lakukan *commit* pada perubahan Anda (`git commit -m 'Menambahkan fitur keren'`).
4. *Push* ke *branch* repositori asal (`git push origin fitur-keren-anda`).
5. Ajukan **Pull Request** baru untuk kami tinjau.

Bila Anda menemukan kendala teknis atau punya saran pengembangan, silakan sampaikan lewat fitur **Issues** di GitHub.
