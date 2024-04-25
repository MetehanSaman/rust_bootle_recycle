# Şişe Geri Dönüşüm Sistemi

<img src="logo.jpeg" alt="Şişe Geri Dönüşüm Sistemi Logo" width="500" height="500">

## Geliştirici

Metehan Saman, Abdullah Gül Üniversitesi'nde Elektrik ve Elektronik Mühendisliği öğrencisi olarak eğitim görmektedir. Yazılım dünyasındaki becerilerini geliştirmek amacıyla, Rise In Internet Computer Rust Workshop'a katılmıştır. Bu etkinlik, Metehan'a Rust programlama dilini öğrenme ve yazılım geliştirme becerilerini daha da güçlendirme fırsatı sunmuştur. Farklı dillerde yazılım geliştirme konusundaki tutkusuyla, Metehan gelecekte farklı mühendislik alanlarında yazılımı etkili bir şekilde kullanmayı hedeflemektedir. Metehan, teknik bilgisi ve yazılım becerileriyle gelecekteki başarılarını artırmayı amaçlamaktadır.

## Proje Açıklaması
Bu proje, kullanıcıların şişe geri dönüşümünü kolaylaştırmak ve izlemek amacıyla tasarlanmış bir sistemdir. Kullanıcılar, bu sistem aracılığıyla bir dizi işlem gerçekleştirebilirler. İlk olarak, kullanıcılar şişe ekleyebilir ve topladıkları şişe sayısını sisteme kaydedebilirler. Daha sonra, sistem, kullanıcıların şişe sayılarını ve hesaplarındaki bakiyeyi kontrol etmelerini sağlar, böylece geri dönüştürülen şişelerin miktarını ve biriken bakiyeyi izleyebilirler.

Ayrıca, kullanıcılar sistemi kullanarak biriken şişeleri paraya dönüştürebilirler. Bu işlem, biriken şişe sayısına göre hesaplanan bir kurallar setine göre gerçekleşir ve kullanıcıların bakiyelerine eklenir. Son olarak, kullanıcılar hesaplarındaki birikmiş parayı çekebilirler.

Sistem aynı zamanda kullanıcıların hesaplarına güvenli bir şekilde giriş yapmalarını sağlamak için bir kimlik doğrulama mekanizması içerir. Bu sayede, kullanıcıların hesaplarına sadece yetkili erişim sağlanır ve kişisel bilgileri korunmuş olur. Bu kimlik doğrulama mekanizması, kullanıcıların gizli şifrelerini güvenli bir şekilde saklamak için modern kriptografik yöntemler kullanır.

Genel olarak, bu sistem kullanıcıların geri dönüşüm faaliyetlerini izlemelerini, yönetmelerini ve bu süreçten elde ettikleri geliri takip etmelerini sağlayarak çevre dostu bir davranışı teşvik etmeyi amaçlamaktadır.

## Vizyon

Bu programın vizyonu, çevreye duyarlı bir toplum oluşturmak ve geri dönüşüm alışkanlıklarını teşvik etmektir. Bu sistem, kullanıcıların geri dönüşüm sürecine daha fazla katılımını sağlayarak çevresel etkiyi azaltmaya yardımcı olmayı hedeflemektedir. Amacı, kullanıcıların geri dönüşüm faaliyetlerini daha kolay bir şekilde yönetmelerini ve izlemelerini sağlayarak çevresel farkındalığı artırmaktır. Böylelikle, toplumun genelinde geri dönüşüm alışkanlıklarının yaygınlaşmasını destekleyerek, doğal kaynakların korunması ve çevresel sürdürülebilirliğin sağlanmasına katkıda bulunmayı amaçlamaktadır.

## Özellikler

### 1. Güvenli Kimlik Doğrulama
- Kullanıcılar, güvenli bir şekilde hesaplarına giriş yapmak için şifreyle kimlik doğrulaması yaparlar. Şifreler, SHA-256 algoritması kullanılarak güvenli bir şekilde saklanır.

### 2. Şişe Takibi ve Bakiye Hesaplama
- Sistem, kullanıcıların ekledikleri şişe sayısını takip eder ve her bir şişenin belirli bir değere sahip olduğunu hesaplar. Kullanıcılar, ekledikleri şişe sayısını ve bakiyelerini istedikleri zaman kontrol edebilirler.

### 3. Otomatik Para Dönüşümü ve Para Çekme
- Kullanıcılar, hesaplarındaki şişe sayısını belirli bir değer karşılığında paraya dönüştürebilirler ve bu parayı para çekme seçeneğini kullanarak elde edebilirler. Sistem, bu dönüşümü otomatik olarak gerçekleştirir ve kullanıcıların bakiyelerini günceller.

### 4. ICP Entegrasyonu
   - Sistem ICP protokülünü de entegre ederek kullanıcı hesaplarına ve işlemlere güvenli bir şekilde yedeklemeyi ve erişmeyi sağlar.

## Yükleme Detayları
Projeyi yerel bir ortamda çalıştırmak için aşağıdaki adımları izleyin:

### Projeyi İndirme veya Klonlama:
- İlk adım olarak, projeyi GitHub deposundan klonlayın veya zip olarak indirin.
- Klonlamak için aşağıdaki komutu kullanabilirsiniz: `bash git clone https://github.com/MetehanSaman/rust_bootle_recycle`
- Zip olarak indirdiyseniz, dosyaları bir klasöre çıkarın.

### Terminali Açma:
- Projenin dizinine gitmek için bir terminal veya komut istemcisini açın.
- İndirme veya klonlama işleminden sonra, proje klasörünün dizinine geçin.

### Projenin Çalıştırılması:
- Terminalde, projenin dizininde olduğunuzdan emin olun.
- Ardından, aşağıdaki komutu kullanarak projeyi çalıştırın: `cargo run`
- Bu komut, Rust projeyi derleyecek ve ardından çalıştıracaktır.

### Kullanıcı Karşılama ve İşlem Menüsü:
- Projeyi başarıyla çalıştırdıktan sonra, sistem sizi karşılayacak ve işlem menüsünü sunacaktır.
- Bu menüde, kullanıcılar şişe ekleyebilir, şişe sayılarını ve bakiyelerini kontrol edebilir, şişeleri paraya dönüştürebilir ve para çekebilirler.
- Bu adımları takip ederek, projeyi yerel bir ortamda başarıyla çalıştırabilir ve sistem üzerinde işlem yapmaya başlayabilirsiniz.





