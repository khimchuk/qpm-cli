# About
**Quick Password Manager** is a password manager written in Rust. It uses encryption, which helps you store passwords securely.


# Installation
### Step 1.
First of all, install the required dependencies in order to build qpm.
|**Distribution**  |**Instructions**                      |
| ---------------- |--------------------------------------|
| Debian           | `apt install libsqlite3-dev gcc`     |
| Ubuntu           | `apt install libsqlite3-dev gcc`     |
| Arch             | `pacman -S sqlite gcc`               |
| Void             | `xbps-install -S sqlite-devel gcc`   |
| Gentoo           | `emerge dev-db/sqlite sys-devel/gcc` |
| Kali             | `apt install libsqlite3-dev gcc`     |
| Fedora           | `dnf install sqlite-devel gcc`       |

You will also need to install Rust using rustup(Recommended). Run the following in your terminal, then follow the onscreen instructions:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Step 2. 
Clone the repository:
```shell
git clone https://github.com/khimchuk/qpm-cli.git
```

### Step 3. 
Install qpm using quick. Go to the qpm directory (qpm-cli by default) and run the following command:
```
./quick install
```

Verify that qpm was successfully installed:
```
$ qpm --version
Quick Password Manager 0.1.0
```


# How to use?
When adding a password, you need to enter your secret. A secret is a universal key to your passwords. It can be unique for everyone, but you can also come up with a new secret for each password. The main advantage is that **qpm** does not report whether the password was decrypted correctly, so you need to remember the secret, otherwise if you enter the wrong secret you will get an incorrectly decrypted password.

You can learn about all the functions of qpm by running the command `qpm --help` in your terminal.
```
$ qpm --help
Usage: qpm [OPTION]
       qpm [OPTION] [ARGUMENT]

Options:
    -h, --help              help message.
    -v, --version           qpm version.

    -s, --set [NAME]        set password.
    -g, --get               get password.
    -d, --delete            remove password.
    -l, --list              get all password names.

Type for more information:
    qpm --help [OPTION]
    qpm -h [OPTION]

Report bugs to <khimchuk.io@gmail.com>
```

You can also find out how to use a specific function by running the command `qpm --help [OPTION]`. For example:
```
$ qpm --help -l
Usage: qpm --list
       qpm -l
```
