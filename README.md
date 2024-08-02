# Distatus
## Getting Started
1. **Create a `.env` file** in the projects directory and add your Discord `TOKEN`:
```env
TOKEN=your-discord-token-here
```
2. **Edit the `status.json` file** to set your own statuses:
```json
[
  {
    "emoji_id": "925772676197388358",
    "emoji_name": "pixeldiscord",
    "text": "Hello, world!",
  }
]
```

```rust
#[derive(Debug, Deserialize, Serialize)]
struct CustomStatus {
    emoji_id: Option<u64>,
    emoji_name: Option<String>,
    text: String,
}
```
