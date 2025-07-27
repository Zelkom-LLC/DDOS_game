<!-- # Referee Service API

**Base URL:** `http://localhost:3001`

---

## POST /start

**Description:** Запускает DDOS-симуляцию («рефери-игру») по целевым сервисам из состояния приложения.

**Request:**
- **URL:** `/start`
- **Method:** `POST`
- **Headers:**
  - `Content-Type: application/json`
- **Body (JSON):** соответствует структуре `GameSettings`:
  ```json
  {
    "threads": number,            // число параллельных потоков
    "connections_amount": number, // соединений на поток
    "round_sec": number,          // длительность раунда в секундах
    "attack_type": {              // тип атаки, один из:
      "burn": number,             // нагрузочная функция
      "bomb": {},                 // простой запрос
      "fib_rec": number,          // рекурсивный Фибоначчи
      "fib_iter": number          // итеративный Фибоначчи
    }
  }
  ```

**Response:**
- **200 OK**  
  - `Content-Type: text/plain`  
  - Тело: отчёт (`String`) по каждому потоку, разделённый переводом строки.
- **4xx** — неверные параметры запроса.
- **5xx** — внутренняя ошибка сервиса.

---

### Example

```bash
curl -X POST http://localhost:3001/start \
     -H "Content-Type: application/json" \
     -d '{
       "threads": 4,
       "connections_amount": 10,
       "round_sec": 20,
       "attack_type": { "burn": 100 }
     }'
``` -->


# Referee Service API

Каждый запуск имеет базовые параметры
- **Тип запроса**: `POST`
- **Base URL:** `http://localhost:3001` (по умолчанию)
- **Ручка**: `/start`
- **Json structure** в виде:
    ```json
    {
        "threads": number,            // число параллельных потоков
        "connections_amount": number, // соединений на поток
        "round_sec": number,          // длительность раунда в секундах
        "attack_type": { ... }
    }
    ```
## Инициализация игры
### 1. Атака типа burn
```json
{
    ...,
    "attack_type": { 
        "burn": number // сложность расчётов
    }
}
```

### 2. Атака типа bomb
```json
{
    ...,
     "attack_type": "bomb"
}
```

### 3. Атака типа fib_rec (Fibonacci recursive)
```json
{
    ...,
    "attack_type": { 
        "fib_rec": number // глубина расчёта
    }
}
```

### 4. Атака типа fib_rec (Fibonacci iter)
```json
{
    ...,
    "attack_type": { 
        "fib_iter": number // глубина расчёта
    }
}
```