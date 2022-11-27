# Password Manager

It is a command line interface for managing passwords

---

## Build Requirements

- _Git_  
  https://git-scm.com/
- _Cargo_  
  https://www.rust-lang.org/tools/install

---

## Program Requirements

- _".env"_ file  
  `You can create your own .env file to exe path`  
  `Note that you have to add _SECRET_KEY = ""`  
  `You can assign any secret key you want`

- _"table.db"_ file  
  `You can create your own table.db file to db folder in exe path`

🟡 _Note that program will be create these file automatically_  
🟡 _Do not have to do it manually_

---

## 🟢Usage (recommended)

    git clone https://github.com/UtkuAltnkaya/Password-Manager-Cli.git
    cargo build --release
    cd target
    cd release

- _copy the path_
- add user environment path

Then you can access it from anywhere

---

## 🔴Usage (not recommended)

    git clone https://github.com/UtkuAltnkaya/Password-Manager-Cli.git
    cargo build --release
    cd target
    cd release
    ./pm
