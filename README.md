# Publisher

## Understanding publisher and message broker

### Berapa banyak data yang dikirim publisher ke message broker dalam satu kali jalan?

Publisher mengirim lima event `UserCreatedEventMessage` dalam satu kali jalan. Setiap event berisi `user_id` dan `user_name`.

### Apa arti `amqp://guest:guest@localhost:5672`?

URL tersebut mengarah ke RabbitMQ message broker yang sama dengan yang digunakan oleh subscriber. Artinya, publisher dan subscriber sama-sama terhubung ke RabbitMQ di komputer lokal menggunakan protokol AMQP, dengan username `guest`, password `guest`, host `localhost`, dan port `5672`.
