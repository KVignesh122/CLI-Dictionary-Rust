# 🌐 CLI Dictionary & Thesaurus – Rust-powered Word Lookup Tool

**CLI Dictionary & Thesaurus** is a **fast and sleek Rust-based command-line tool** 🚀 that **fetches definitions, synonyms, and antonyms** for any **English** word you query. With **ASCII art headers, color-coded output, and cross-platform binaries for Windows, macOS, and Linux**, this tool offers a fun and efficient way to explore words **directly from the terminal**. ✨

---

## 📥 Installation & Usage

### **Option 1: Download Pre-Compiled Binary (Recommended)**

1. **Download** the binary for your platform from the latest [Release Page](https://github.com/your-username/rust-eng-dictionary/releases):

   - [Windows](https://github.com/KVignesh122/CLI-Dictionary-Rust/releases/download/v1.0.1/rust-eng-dictionary.exe)
   - [macOS/Linux](https://github.com/KVignesh122/CLI-Dictionary-Rust/releases/download/v1.0.0/rust-eng-dictionary)
2. **Open Terminal/Command Prompt** and navigate to the download directory.
3. **Run the Program**:

   - On **Windows**:
     ```cmd
     rust-eng-dictionary.exe <your_word>
     ```
   - On **macOS/Linux**:
     ```bash
     ./rust-eng-dictionary <your_word>
     ```

---

### **Option 2: Install via Cargo (Requires Rust)**

If you have Rust installed, you can clone the repo and build the binary yourself:

```bash
git clone https://github.com/KVignesh122/CLI-Dictionary-Rust
cd rust-eng-dictionary
cargo build
cargo run -- <your_word>
```

---

## 🎨 Sample Output

```bash
./rust-eng-dictionary rust
```

```
 ██████╗ ██╗   ██╗ ███████╗████████╗
 ██╔══██╗██║   ██║ ██╔════╝╚══██╔══╝
 ██████╔╝██║   ██║ ███████╗   ██║   
 ██╔══██╗██║   ██║ ╚════██║   ██║   
 ██║  ██║╚██████╔╝ ███████║   ██║   
 ╚═╝  ╚═╝ ╚═════╝  ╚══════╝   ╚═╝   

===============================
        As a NOUN
-------------------------------
  1) The deteriorated state of iron or steel as a result of moisture and oxidation.
  2) A similar substance based on another metal (usually with qualification, such as "copper rust").
  3) A reddish-brown color.
  4) A disease of plants caused by a reddish-brown fungus.
  5) Damage caused to stamps and album pages by a fungal infection.


===============================
        As a VERB
-------------------------------
  1) To oxidize, especially of iron or steel.
  2) To cause to oxidize.
  3) To be affected with the parasitic fungus called rust.
  4) To (cause to) degenerate in idleness; to make or become dull or impaired by inaction.

Synonyms: corrode, oxidise, oxidize
```

---

## 🛠️ Built With

- **Rust** – A fast and safe systems programming language.
- **Reqwest** – For making HTTP requests.
- **Serde** – For JSON parsing and deserialization.
- **Clap** – For handling command-line input.
- **text-to-ascii-art** – For generating ASCII art headers.
- **Tokio** – An async runtime for Rust.

---

## ❓ FAQ

1. **How do I handle API errors or connection issues?**
   - Ensure you have an **active internet connection**. The API used requires internet access to fetch word data.
2. **Why is a word not found?**
   - Some rare words or slang may not be in the dictionary. Try other forms of the word if needed.

---

## 🙌 Acknowledgements

- [DictionaryAPI](https://dictionaryapi.dev/) – Free dictionary and thesaurus API.
- ASCII art generation in this project is made possible with the **text-to-ascii-art** crate by [osmak1234](https://github.com/osmak1234/text-to-ascii-art).
- Special thanks to the Rust community for providing awesome tools and resources.

---

## 🌟 Show Your Support

If you find this project useful, please **star** the repository on GitHub to show your support! 🚀

---

## 📄 License

This project is licensed under the **MIT License** – see the [LICENSE](LICENSE) file for details.

---
