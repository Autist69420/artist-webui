local websocket_url = "ws://127.0.0.1:8080/ws" -- change this
local logger = require("artist.lib.log")
local json = require("examples.json")

local log = logger.get_logger("artist-webui")

local turtle_information = {
	name = os.getComputerLabel() or "No name",
	id = os.getComputerID(),
}

local function sluts_contain_item(sluts, item)
	for _, v in pairs(sluts) do
		if v.details.name == item.details.name then
			return true
		end
	end

	return false
end

local function get_sluts(items, inventories)
	local all_sluts = {}

	for _, inventory in pairs(inventories) do
		local sluts = inventory.slots

		for _, slut in pairs(sluts or {}) do
			if slut.count > 0 then
				local item = items.item_cache[slut.hash]
				if not sluts_contain_item(all_sluts, item) then
					table.insert(all_sluts, item)
				end
			end
		end
	end
	return all_sluts
end

local function get_furnaces(furnaces)
	local furni = {}

	for _, furnace in pairs(furnaces.hot_furnaces) do
		local name = furnace.name
		local cooking = furnace.cooking
		table.insert(furni, { cooking = cooking, name = name })
	end

	for _, furnace in pairs(furnaces.cold_furnaces) do
		local name = furnace.name
		local cooking = furnace.cooking
		table.insert(furni, { cooking = cooking, name = name })
	end

	return furni
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
				packet = "artist_inventory_update",

				data = {
					used_slots = used_slots,
					full_slots = full_slots,
					total_slots = total_slots,

					slots = get_sluts(items, items.inventories),
				},
			}
			--log(json.encode(data))
			websocket.send(json.encode(data))
		end
	end

	local function send_item_change(cock)
		log(textutils.serialise(cock[1]))
	end

	local function send_furnace_change()
		-- just a lil protection
		if waka == false then
			local gotten_furni = get_furnaces(furnaces)

			local data = {
				packet = "artist_furnace_update",

				data = { furnaces = gotten_furni },
			}
			websocket.send(json.encode(data))
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
				-- TODO: I should probably make this actually do something
				-- local url, message = eventData[2], eventData[3]
				-- log(url, message)
			elseif event == "websocket_close" then
				log("Websocket closed :( %s", eventData[2])

				websocket.close()
			elseif event == "websocket_success" then
				log("Connected to " .. websocket_url)
			elseif event == "websocket_failure" then
				log("Could not connect to " .. websocket_url)

				break
			end
		end
	end)

	-- Basically spawn a "thread"
	context:spawn(function(_)
		-- Do sending shit and stuff
		repeat
			local data = {
				packet = "turt_connect",
				data = turtle_information,
			}

			websocket.send(json.encode(data))
			os.pullEvent("websocket_message")
			waka = false
		until waka == false
	end)

	log("Finished.")
end
