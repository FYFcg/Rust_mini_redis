namespace rs volo.example

enum RedisCommand {
    Get,
    Set,
    Ping,
    Del,
    Publish,
    Subscribe,
}

struct GetItemRequest {
    1: required RedisCommand command,
    2: optional string key,
    3: optional string value,
}

struct GetItemResponse {
    1: required bool flag,
    2: optional string res,
}

service ItemService {
    GetItemResponse GetItem (1: GetItemRequest req),
}
