# About
**Quick Password Manager** is a password manager written in Rust. It uses encryption, which helps you store passwords securely.



# Installation
First of all, install the required dependencies in order to build qpm.
|**Distribution**       |**Instructions**                      |
| --------------------- |--------------------------------------|
| Debian, Ubuntu, Kali  | `apt install libsqlite3-dev gcc`     |
| Arch                  | `pacman -S sqlite gcc`               |
| Void                  | `xbps-install -S sqlite-devel gcc`   |
| Gentoo                | `emerge dev-db/sqlite sys-devel/gcc` |
| Fedora                | `dnf install sqlite-devel gcc`       |

You will also need to install Rust using rustup(recommended). Run the following in your terminal, then follow the onscreen instructions:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Next, you can choose two installation paths: through the cargo package manager or compile it yourself from sources


### Install via cargo
Run the following command in your terminal:
```
cargo install qpm --locked
```


### Install from sources
#### Step 1. 
Clone the repository:
```shell
git clone https://github.com/khimchuk/qpm-cli.git
```

#### Step 2. 
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
When adding a password, you need to enter your secret. A secret is a universal key to your passwords. It can be unique for everyone, you can also come up with your own secret for each password or group of passwords, you are not limited in this way. The main advantage is that **qpm** does not report whether the password was decrypted correctly, so you need to remember the secret, otherwise if you enter the wrong secret you will get an incorrectly decrypted password.

You can learn about all the functions of qpm by running the command `qpm --help` in your terminal.
```
$ qpm --help
Usage: qpm [OPTION]
       qpm [OPTION] [ARGUMENT]

Options:
    -h, --help              help message.
    -V, --version           qpm version.

     s, set [NAME]          set password. 
     g, get                 get password.
     d, delete              remove password.
     r, rename              rename password.
     l, list                get all password names.
     
Type for more information:
    qpm help [OPTION]

Report bugs to <khimchuk.io@gmail.com>
```

You can also find out how to use a specific function by running the command `qpm help [OPTION]`. For example:
```
$ qpm help l
Usage: qpm list
       qpm l
```



# License
Quick Password Manager is released under the MIT License. You can find a copy of the license text here: [LICENSE](../master/LICENSE)
