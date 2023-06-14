# fuck
On websocket connection the artist plugin sends the following to the websocket server

```json5
{
	"packet": "turt_connect", // packet type so the server knows what the fuck this is
	"data": { // casual data
		"name": "No name", // turtle name gotten from `os.getComputerLabel()` defaults to `No name` if there is no name
		"id": 5 // computer id gotten from `os.getComputerID()`, has no default
	}
}
```

When artist starts it sends multiple packets: `artist_furnace_update`, `artist_inventory_update` and `artist_item_change`.

### Packet `artist_furnace_update`
```json5
{
	"packet": "artist_furnace_update",
	"data": {
		"hot_furnaces": [
			{
				"name": "minecraft:furnace_0",
				"cooking": true
			}
		],
		"cold_furnaces": [
			{
				"name": "minecraft:furnace_1",
				"cooking": false
			}
		]
	}
}
```

### Packet `artist_inventory_update`

### Packet `artist_item_change`