local Trainer = require("romestead-trainer.trainer")
local Config = require("romestead-trainer.config")

local romestead = Trainer:new(Config)

function romestead:run()
    self:load_profile()
    self:apply_patches()
    self:start_monitor()
    print("Romestead Trainer v" .. Config.VERSION .. " active")
end

return romestead