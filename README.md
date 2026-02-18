# brodox

A minimalist terminal-based web browser written in Rust. Designed for speed, simplicity, and low-resource environments like Termux.

## Features
* **Integrated Search**: Direct search via DuckDuckGo HTML (no JS required).
* **Smart URL Handling**: Automatically detects if you're searching or entering a link.
* **Lightweight**: Zero overhead, perfect for mobile terminals.
* **Clean Interface**: Built-in `clear` command to keep your workspace tidy.

## Installation
1. Clone the repository:
   ```bash
git clone https://github.com/NickIBrody/Brodox.git
   cd brodox
# Build and run
cargo run

# Usage
​Type a URL (e.g., google.com) to visit a site.
​Type a search query (e.g., rust programming) to search.
​Type clear to clean the terminal screen.
​Type exit to close the browser.

## Requirements
| Requirement | Version |
|-------------|---------|
| Rust        | 1.56+   |
| Cargo       | Latest  |
| OS          | Linux / Android (Termux) / macOS |

