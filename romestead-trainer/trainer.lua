local Trainer = {}
Trainer.__index = Trainer

function Trainer:new(config)
    local instance = setmetatable({}, self)
    instance.config = config
    instance.active_patches = {}
    instance.monitor_interval = config.MONITOR_INTERVAL or 1.0
    return instance
end

function Trainer:load_profile()
    local profile_path = self.config.PROFILE_PATH
    local ok, profile = pcall(dofile, profile_path)
    if ok and type(profile) == "table" then
        self.profile = profile
        print("Profile loaded: " .. profile.name)
    else
        self.profile = { name = "default", patches = {} }
        print("Using default profile")
    end
end

function Trainer:apply_patches()
    for _, patch in ipairs(self.profile.patches) do
        if patch.enabled then
            table.insert(self.active_patches, patch)
            print("Applied: " .. patch.name)
        end
    end
end

function Trainer:start_monitor()
    self.monitor_running = true
    while self.monitor_running do
        for _, patch in ipairs(self.active_patches) do
            if patch.type == "memory" then
                self:write_memory(patch.address, patch.value)
            elseif patch.type == "input" then
                self:simulate_input(patch.key)
            end
        end
        os.execute("sleep " .. tostring(self.monitor_interval))
    end
end

function Trainer:write_memory(address, value)
    -- Stub: would interface with OS memory
    print(string.format("MEM: 0x%X <- %d", address, value))
end

function Trainer:simulate_input(key)
    -- Stub: would send key events
    print("INPUT: " .. key)
end

function Trainer:stop()
    self.monitor_running = false
    print("Trainer stopped")
end

return Trainer