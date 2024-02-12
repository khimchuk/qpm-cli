# About
**Quick Password Manager** is a password manager written in Rust. It uses encryption, which helps you store passwords securely.

# Installation
## Step 1.
First of all, install the required dependencies in order to build qpm.
|**Distribution**  |**Instructions**                      |
| ---------------- |:------------------------------------:|
| Debian           | `apt install libsqlite3-dev gcc`     |
| Ubuntu           | `apt install libsqlite3-dev gcc`     |
| Arch             | `pacman -S sqlite gcc`               |
| Void             | `xbps-install -S sqlite-devel gcc`   |
| Gentoo           | `emerge dev-db/sqlite sys-devel/gcc` |
| Kali             | `apt install libsqlite3-dev gcc`     |
| Fedora           | `dnf install sqlite-devel gcc`       |

You will also need to install Rust using rustup. Run the following in your terminal, then follow the onscreen instructions:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Step 2. 
Clone the repository:
```
git clone https://github.com/khimchuk/qpm-cli.git
```

## Step 3. 
Install qpm using quick. Go to the qpm directory (qpm-cli by default) and run the following command:
```
./quick install
```

Verify that qpm was successfully installed:
```
qpm --version
```
Expected output:
```
Quick Password Manager 0.1.0
```
