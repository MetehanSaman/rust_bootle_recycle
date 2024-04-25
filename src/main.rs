use std::io;
use std::io::Write;
use sha2::{Sha256, Digest};
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::storage;
use std::collections::HashMap;

#[derive(CandidType, Deserialize)]
struct HesapSahibi { // Hesap verilerini tutmak için oluşturulan yapı
    ad: String,
    soyad: String,
    şişe_sayisi: i32,
    bakiye: i32,
    sifre_hash: String,
    giriş: bool
}
fn hasher(sifre: &str) -> String { // Bu fonksiyon alınan yazı tipi değişkenini SHA256 ile şifrelemek için kullanılır.
    let mut _hash = Sha256::new();
    _hash.update(sifre);
    let hash_sonuc = _hash.finalize();
    let hash_sonuc_string = format!("{:x}", hash_sonuc);
    hash_sonuc_string
}
fn giriş_yap(kullanici: &mut HesapSahibi) -> bool {
    loop {
        println!(" ");
        println!("Sayın {} {}, İşleme Devam Etmek İçin Şifre Girmeniz Gerekmektedir! \n",kullanici.ad,kullanici.soyad);

        let mut sifre_karsilastirma : String = String::new();
        print!("Şifrenizi Giriniz: ");
        io::stdout().flush().expect("Konsola yazdirma hatasi");
        io::stdin().read_line(&mut sifre_karsilastirma).expect("Şifre okuma hatasi");

        if kullanici.sifre_hash == hasher(&sifre_karsilastirma.trim()) {
            println!("Şifre doğru.");
            kullanici.giriş = true;
            return kullanici.giriş;
        }
        else {
            println!("Şifre yanliş. Tekrar deneyin.");
        }
    }
}
fn hesap_oluştur() -> HesapSahibi { // Kullanıcıdan verileri alıp "HesapSahibi" yapısına ekleyen fonksiyon
    let mut _ad = String::new();
    print!("İsminizi Giriniz: ");
    io::stdout().flush().expect("Konsola yazdirma hatasi");
    io::stdin().read_line(&mut _ad).expect("İsim okuma hatasi");

    let mut _soyad = String::new();
    print!("Soyisminizi Giriniz: ");
    io::stdout().flush().expect("Konsola yazdirma hatasi");
    io::stdin().read_line(&mut _soyad).expect("Soyisim okuma hatasi");

    let mut _sifre = String::new();
    print!("Şifrenizi Belirleyiniz: ");
    io::stdout().flush().expect("Konsola yazdirma hatasi");
    io::stdin().read_line(&mut _sifre).expect("Şifre okuma hatasi");

    _sifre = _sifre.trim().to_string();

    print!("Hesap Oluşturma Başarılı \n");

    //println!("SHA256: {:x}", hasher(&_sifre););

    HesapSahibi {
        ad: _ad.trim().to_string(),
        soyad: _soyad.trim().to_string(),
        şişe_sayisi: 0,
        bakiye: 0,
        sifre_hash: hasher(&_sifre),
        giriş: false
    }
}
#[update]
fn hesap_kaydet(kullanici: HesapSahibi) {
    let hesaplar = storage::get_mut::<HashMap<String, HesapSahibi>>();
    let key = format!("{}{}", kullanici.ad, kullanici.soyad);
    hesaplar.insert(key, kullanici);
}
#[query]
fn hesap_yükle(ad: String, soyad: String) -> Option<HesapSahibi> {
    let hesaplar = storage::get::<HashMap<String, HesapSahibi>>();
    let key = format!("{}{}", ad, soyad);
    hesaplar.get(&key).cloned()
}

fn main() {
    print!("\nŞişe Geri Dönüştürücüsüne Hoş Geldiniz \n");
    println!("1 Şişe = 2 TL \n");

    let mut giriş: bool = false; // Kullanıcının giriş yapıp yapamdığını kontrol eden değişken

    let mut _kullanici = HesapSahibi { // Kullanıcı değişkenlerini tutacak yapının atanması
        ad: "".to_owned(),
        soyad: "".to_owned(),
        şişe_sayisi: 0,
        bakiye: 0,
        sifre_hash: hasher(""),
        giriş: false
    };

    loop {
        let mut seçenek_string = String::new();

        print!("1-Şişe Ekle \n2-Şişe Sayısını ve Bakiyeyi Göster \n3-Şişeleri Paraya Dönüştür \n4-Para Çek \n5-Kullanıcı Yükle \n0-Çikiş \nSeçenek:");
        io::stdout().flush().expect("Konsola yazdirma hatasi");
        io::stdin().read_line(&mut seçenek_string).expect("Seçenek okuma hatasi");

        let seçenek: i8 = match seçenek_string.trim().parse() { // Seçeneği yazı tipinden sayı tipine dönüştürme ve hata kontrolü
            Ok(num) => num,
            Err(_) => {-1}
        };
        
        if giriş !=true && seçenek !=0 { // Kullanıcının giriş yapıp yapmadığını kontrol edip, yapmadıysa hesap açma adımına yönlendirme
            println!("Devam Etmek İçin Hesap Açmanız Gerekmektedir!");
            _kullanici = hesap_oluştur();
            //giriş = _kullanici.giriş;
        }
        
        match seçenek{ // Kullanıcının seçtiği seçeneğe göre fonksiyonları yerine getirme aşaması
            0 => {
                hesap_kaydet(_kullanici.clone()); // Kullanıcıyı depolama
                break;
            }
            1 => {
                if giriş !=true {
                    giriş = giriş_yap(&mut _kullanici);
                }
                let mut şişe_sayisi_string : String = String::new();
                print!("Şişe Sayisini Giriniz: ");
                io::stdout().flush().expect("Konsola yazdirma hatasi");
                io::stdin().read_line(&mut şişe_sayisi_string).expect("Sayi okuma hatasi");
                //print!("{} \n",şişe_sayisi_string);
                let şişe_sayisi: i32 = match şişe_sayisi_string.chars().filter(|c| c.is_numeric()).collect::<String>().parse() /*Filtre*/ {
                    Ok(deger) => deger,
                    Err(e) => {
                        eprintln!("Dizeyi u32'ye dönüştürme hatasi: {}", e);
                        0 /*Varsayılan değer*/
                    }
                };
                _kullanici.şişe_sayisi += şişe_sayisi; // Kullanıcı şişe sayısını güncelleme
                println!("");
                println!("Toplam Şişe Sayısı: {} \n",_kullanici.şişe_sayisi);
            }
            2 => {
                if giriş !=true {
                    giriş = giriş_yap(&mut _kullanici);
                }
                println!("");
                println!("Sayın {} {}, Toplam Şise Sayınız: {} , Toplam Bakiyeniz: {} TL \n",_kullanici.ad,_kullanici.soyad,_kullanici.şişe_sayisi, _kullanici.bakiye);
            }
            3=> {
                if giriş !=true {
                    giriş = giriş_yap(&mut _kullanici);
                }
                let mut şişe_sayısı_temp : String = String::new();
                print!("Dönüştürmek İstediğiniz Şişe Sayısı: ");
                io::stdout().flush().expect("Konsola yazdirma hatasi");
                io::stdin().read_line(&mut şişe_sayısı_temp).expect("Sayi okuma hatasi");
                let şişe_sayısı_temp: i32 = match şişe_sayısı_temp.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {0}
                };
                if şişe_sayısı_temp<=_kullanici.şişe_sayisi{
                    _kullanici.şişe_sayisi-=şişe_sayısı_temp;
                    _kullanici.bakiye+=şişe_sayısı_temp*2;
                    println!("");
                    println!("Şişeleri Paraya Dönüştürme İşlemi Başarılı \n");
                }
                else{
                    println!("");
                    println!("Şişe Sayınızı Tekrar Kontrol Ediniz!\n");
                }
            }
            4=> {
                if giriş !=true {
                    giriş = giriş_yap(&mut _kullanici);
                }
                let mut bakiye_temp : String = String::new();
                print!("Çekmek İstediğiniz Para Miktarı: ");
                io::stdout().flush().expect("Konsola yazdirma hatasi");
                io::stdin().read_line(&mut bakiye_temp).expect("Sayi okuma hatasi");
                let bakiye_temp: i32 = match bakiye_temp.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {0}
                };
                if bakiye_temp<=_kullanici.bakiye{
                    _kullanici.bakiye-=bakiye_temp;
                    println!("");
                    println!("Para Çekme İşlemi Başarılı! \n");
                }
                else{
                    println!("");
                    println!("Miktarı Kontrol Ediniz! \n");
                }
            }
            5=> {

                let mut _ad = String::new();
                print!("İsminizi Giriniz: ");
                io::stdout().flush().expect("Konsola yazdirma hatasi");
                io::stdin().read_line(&mut _ad).expect("İsim okuma hatasi");

                let mut _soyad = String::new();
                print!("Soyisminizi Giriniz: ");
                io::stdout().flush().expect("Konsola yazdirma hatasi");
                io::stdin().read_line(&mut _soyad).expect("Soyisim okuma hatasi");

                let yüklenen_kullanici = hesap_yükle(_ad.clone(), _soyad.clone()); // Kullanıcıyı yükleme
                if let Some(kullanici) = yüklenen_kullanici {
                    _kullanici = kullanici; // Yüklenen kullanıcıyı mevcut kullanıcıya atama
                } else {
                    println!("Kullanıcı bulunamadı.");
                }
            }
            _ => {
                println!("");
                println!("Hatalı Giriş Yaptınız, Tekrar Deneyiniz!");
                println!("");
            }
        }
    }
}

// Metehan Saman