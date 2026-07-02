local Trainer = require("romestead-trainer.trainer")
local Config = require("romestead-trainer.config")

local function test_initialization()
    local t = Trainer:new(Config)
    assert(t.config == Config, "Config not stored")
    assert(#t.active_patches == 0, "Should start empty")
    print("PASS: initialization")
end

local function test_load_profile()
    local t = Trainer:new(Config)
    t:load_profile()
    assert(t.profile ~= nil, "Profile should exist")
    assert(t.profile.name == "default", "Wrong profile name")
    print("PASS: load_profile")
end

local function test_apply_patches()
    local t = Trainer:new(Config)
    t:load_profile()
    t:apply_patches()
    assert(#t.active_patches == 1, "Should have 1 enabled patch")
    assert(t.active_patches[1].name == "Infinite Health", "Wrong patch")
    print("PASS: apply_patches")
end

test_initialization()
test_load_profile()
test_apply_patches()
print("All tests passed!")