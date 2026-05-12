# Publisher

## Understanding publisher and message broker

### Berapa banyak data yang dikirim publisher ke message broker dalam satu kali jalan?

Publisher mengirim lima event `UserCreatedEventMessage` dalam satu kali jalan. Setiap event berisi `user_id` dan `user_name`.

### Apa arti `amqp://guest:guest@localhost:5672`?

URL tersebut mengarah ke RabbitMQ message broker yang sama dengan yang digunakan oleh subscriber. Artinya, publisher dan subscriber sama-sama terhubung ke RabbitMQ di komputer lokal menggunakan protokol AMQP, dengan username `guest`, password `guest`, host `localhost`, dan port `5672`.

![alt text](image.png)
## Sending and Processing Event
Publisher mengirim lima event `UserCreatedEventMessage` ke RabbitMQ melalui queue `user_created`. Subscriber yang sudah berjalan akan menerima dan memproses setiap event tersebut.
![alt text](image-1.png)

## Monitoring Chart Based on Publisher

Ketika publisher dijalankan beberapa kali, jumlah message yang masuk ke RabbitMQ meningkat. Hal ini terlihat pada chart RabbitMQ Management, terutama pada bagian message rates dan queued messages.

Publisher mengirim 5 event setiap kali dijalankan. Karena subscriber memproses message dengan delay, message sempat menumpuk di queue `user_created`. Akibatnya, chart queued messages naik setelah publisher dijalankan, lalu turun kembali ketika subscriber selesai memproses message.
![alt text](image-2.png)