# XSS Hunter (Rust Version)

## Environment Variables
```
BCRYPT_ROUNDS
SCREENSHOTS_DIR
DATABASE_URL
MAX_DB_CONNECTIONS
CONFIG_DIR
``` 

```http
GET http://localhost:3000/ HTTP/1.1
content-type: application/json
```

```http
POST http://localhost:3000/page_callback HTTP/1.1
content-type: application/json
{
    "uri": "https://google.com",
    "html": "hello"
}
```

```http
GET http://localhost:3000/screenshots/11111 HTTP/1.1
content-type: application/json
```

```http
GET http://localhost:3000/health HTTP/1.1
content-type: application/json
```