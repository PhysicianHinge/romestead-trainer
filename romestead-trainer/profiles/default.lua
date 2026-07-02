return {
    name = "Default Profile",
    patches = {
        {
            name = "Infinite Health",
            enabled = true,
            type = "memory",
            address = 0x004A5B2C,
            value = 999
        },
        {
            name = "Auto Attack",
            enabled = false,
            type = "input",
            key = "SPACE"
        }
    }
}