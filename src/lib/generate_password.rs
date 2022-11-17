use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use rand::Rng;

pub struct GeneratePassword {
    password_name: String,
    password: String,
    len: usize,
}

impl GeneratePassword {
    pub fn new(password_name: String, size: usize) -> Self {
        Self {
            password_name: password_name,
            password: "".to_owned(),
            len: size,
        }
    }
    pub fn generate_password(&mut self) -> Result<String, String> {
        let items = [
            "qwertyuiopasdfghjklzxcvbnm",
            "QWERTYUIOPASDFGHJKLZXCVBNM",
            "!'^#$+%&/|?=-_<>(){}[]",
            "0123456789",
        ];
        let size = self.len / 4;
        let mut range = rand::thread_rng();
        let mut count = [0, 0, 0, 0];
        let mut last: i32 = -1;
        let mut i = 0;
        while i < self.len {
            let rand1: usize = range.gen_range(0..4);
            if rand1 == last as usize {
                continue;
            }
            if count[rand1] >= size {
                if count[0] == size && count[1] == size && count[2] == size && count[3] == size {
                    break;
                }
                last = rand1 as i32;
                continue;
            }
            count[rand1] += 1;
            let rand2 = range.gen_range(0..items[rand1].len() - 1);
            self.password += &items[rand1].chars().nth(rand2).unwrap().to_string();
            i += 1;
            last = rand1 as i32;
        }
        match self.encrypt() {
            Ok(result) => Ok("Password successfully generated and".to_owned() + &result),
            Err(error) => Err(error),
        }
    }

    fn encrypt(&mut self) -> Result<String, String> {
        let mc = new_magic_crypt!("killer_queen", 256);
        let hash = mc.encrypt_str_to_base64(&self.password);

        match hash.as_str() {
            "" => Err("An error occurred while encrypting".to_owned()),
            _ => Ok("encrypted".to_owned()),
        }
    }

    pub fn decrypt(password: String) -> String {
        let mc = new_magic_crypt!("killer_queen", 256);
        mc.decrypt_base64_to_string(password).unwrap()
    }

    pub fn get_password(&self) -> &String {
        &self.password
    }

    pub fn set_password_name(&mut self, password_name: String) {
        self.password_name = password_name;
    }
}
