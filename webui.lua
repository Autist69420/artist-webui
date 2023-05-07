--local widget = require("artist.lib.widget")

local websocket_url = "ws://127.0.0.1:8080/ws" -- change this
local logger = require("artist.lib.log")
local json = require("examples.json")

local log = logger.get_logger("artist-webui")

local turtle_information = {
	name = os.getComputerLabel() or "No name",
	id = os.getComputerID(),
}

local function key_table_len(tbl)
	local len = 0

	for _, _ in pairs(tbl) do
		len = len + 1
	end

	return len
end

local function get_sluts(items, inventories)
	local all_sluts = {}

	for i, inventory in pairs(inventories) do
		local sluts = inventory.slots

		for _, slut in pairs(sluts or {}) do
			if slut.count > 0 then
				local item = items.item_cache[slut.hash]
				table.insert(all_sluts, item)
			end
		end
	end
	return all_sluts
end

return function(context)
	local items = context:require("artist.core.items")
	local furnaces = context:require("artist.items.furnaces")

	log("Connecting to websocket url: %s", websocket_url)

	local websocket = http.websocket(websocket_url)
	local waka = true

	-- gets called when an inventory gets scanned or a deleted/added
	local function send_inventory_change()
		if waka == false then
			local used_slots, full_slots, total_slots = 0, 0, 0
			for _, inventory in pairs(items.inventories) do
				for _, slot in pairs(inventory.slots or {}) do
					total_slots = total_slots + 1

					if slot.count > 0 then
						used_slots = used_slots + 1
						-- Look up the item's metadata in the cache to get the max stack size.
						-- If the item isn't available, assume the slot is full.
						local item = items.item_cache[slot.hash]
						if item and item.details then
							full_slots = full_slots + (slot.count / item.details.maxCount)
						else
							full_slots = full_slots + 1
						end
					end
				end
			end

			local data = {
				packet_type = "inventory_peripherals_update",

				inventory = {
					used_slots = used_slots,
					full_slots = full_slots,
					total_slots = total_slots,

					slots = get_sluts(items, items.inventories)
				}
			}
			websocket.send(json.encode(data))
		end
	end

	local function send_item_change() end

	local function send_furnace_change()
		-- just a lil protection
		if waka == false then
			local hot_furnaces_len, cold_furnaces_len =
				key_table_len(furnaces.hot_furnaces), key_table_len(furnaces.cold_furnaces)

			local data = {
				packet_type = "furnace_update",

				furnaces = {
					hot_furnaces = hot_furnaces_len,
					cold_furnaces = cold_furnaces_len,
				}
			}

			websocket.send(textutils.serialiseJSON(data))
		end
	end

	-- Subscribe to several events, queuing a send.
	context.mediator:subscribe("items.inventories_change", send_inventory_change)
	context.mediator:subscribe("items.change", send_item_change)
	context.mediator:subscribe("furnaces.change", send_furnace_change)

	context:spawn(function(_)
		while true do
			local eventData = { os.pullEvent() }
			local event = eventData[1]

			if event == "websocket_message" then
				local url, message = eventData[2], eventData[3]
				--log(url, message)
			elseif event == "websocket_close" then
				log("Websocket closed :( %s", eventData[2])

				websocket.close()
			elseif event == "websocket_success" then
				print("Connected to " .. websocket_url)
			end
		end
	end)

	-- Basically spawn a "thread"
	context:spawn(function(_)
		-- Do sending shit and stuff
		repeat
			local data = {
				packet_type = "turtle_connect",

				turtle_information = turtle_information,
			}

			websocket.send(textutils.serialiseJSON(data))
			os.pullEvent("websocket_message")
			waka = false
		until waka == false
	end)

	log("Finished.")
end
